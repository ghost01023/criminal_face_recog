import insightface
import numpy as np
import cv2
import logging
import contextlib
import os
from sklearn.cluster import KMeans
import sys
import time

time.sleep(0.1)
logger = logging.getLogger(__name__)


def cosine_similarity(a, b):
    return np.dot(a, b) / (np.linalg.norm(a) * np.linalg.norm(b))


class FaceRecognizer:
    def __init__(
        self,
        db_path="faces_db.npz",
        max_embeddings_per_person=10,
        max_centroids=3,
    ):
        self.db_path = db_path
        self.max_embeddings = max_embeddings_per_person
        self.max_centroids = max_centroids

        print("Initializing AI models (silently)...", flush=True)
        with open(os.devnull, "w") as fnull:
            with contextlib.redirect_stdout(fnull), contextlib.redirect_stderr(fnull):
                self.app = insightface.app.FaceAnalysis(
                    name="buffalo_l", providers=["CUDAExecutionProvider"]
                )
                self.app.prepare(ctx_id=0, det_size=(640, 640))

        # name -> List[np.ndarray]
        self.embeddings = {}

        # name -> np.ndarray (centroids or average)
        self.representations = {}

        self.load_db()

    # =========================
    # Persistence
    # =========================

    def load_db(self):
        if not os.path.exists(self.db_path):
            logger.info("No local face DB found. Starting fresh.")
            return

        logger.info("Loading face DB.")
        data = np.load(self.db_path, allow_pickle=True)

        self.embeddings = {}

        for name in data.files:
            arr = data[name]

            # 🔒 Normalize old DB formats
            if arr.ndim == 1:
                # Old format: single embedding
                self.embeddings[name] = [arr.astype(np.float32)]
            elif arr.ndim == 2:
                # New format: multiple embeddings
                self.embeddings[name] = [
                    arr[i].astype(np.float32) for i in range(arr.shape[0])
                ]
            else:
                logger.warning("Invalid embedding shape for %s: %s", name, arr.shape)

        self._rebuild_representations()

    def save_db(self):
        self._rebuild_representations()
        np.savez(
            self.db_path,
            **{name: np.stack(embs) for name, embs in self.embeddings.items()},
        )

    # =========================
    # Internal helpers
    # =========================

    def _rebuild_representations(self):
        """
        Build per-person centroids or averages.
        """
        self.representations = {}

        for name, embs in self.embeddings.items():
            X = np.vstack([e.reshape(1, -1) if e.ndim == 1 else e for e in embs])

            if len(embs) == 1:
                self.representations[name] = X
                continue

            if len(embs) >= self.max_centroids:
                k = min(self.max_centroids, len(embs))
                kmeans = KMeans(n_clusters=k, random_state=42)
                kmeans.fit(X)
                self.representations[name] = kmeans.cluster_centers_
            else:
                avg = np.mean(X, axis=0, keepdims=True)
                self.representations[name] = avg

    def _prune_embeddings(self, name):
        """
        Keep only the most diverse embeddings.
        """
        embs = self.embeddings[name]

        if len(embs) <= self.max_embeddings:
            return

        selected = [embs[0]]

        for emb in embs[1:]:
            similarities = [cosine_similarity(emb, s) for s in selected]
            if max(similarities) < 0.85:
                selected.append(emb)
            if len(selected) >= self.max_embeddings:
                break

        self.embeddings[name] = selected

    # =========================
    # Public API
    # =========================

    def add_person(self, name, image_path):
        img = cv2.imread(image_path)
        if img is None:
            logger.error("Failed to load image: %s", image_path)
            return False

        faces = self.app.get(img)
        if not faces:
            logger.warning("No face detected in %s", image_path)
            return False

        emb = faces[0].embedding

        if name not in self.embeddings:
            self.embeddings[name] = []

        self.embeddings[name].append(emb)
        self._prune_embeddings(name)

        return True

    def identify(self, image_path, threshold=0.4):
        img = cv2.imread(image_path)
        if img is None:
            return None, 0

        faces = self.app.get(img)
        if not faces:
            return None, 0

        test_emb = faces[0].embedding

        best_name = None
        best_score = threshold

        for name, reps in self.representations.items():
            for rep in reps:
                score = cosine_similarity(test_emb, rep)
                if score > best_score:
                    best_score = score
                    best_name = name

        return (best_name, float(best_score)) if best_name else (None, 0)

    def identify_from_video(
        self,
        video_path,
        threshold=0.4,
        frame_skip=5,
        max_frames=300,
    ):
        if not os.path.exists(video_path):
            logger.error("Video not found: %s", video_path)
            return None, 0

        cap = cv2.VideoCapture(video_path)
        if not cap.isOpened():
            return None, 0

        matches = {}
        frame_count = 0
        processed = 0

        while True:
            ret, frame = cap.read()
            if not ret:
                break

            frame_count += 1
            if frame_count % frame_skip != 0:
                continue

            processed += 1
            if processed > max_frames:
                break
            faces = self.app.get(frame)
            if not faces:
                continue

            for face in faces:
                emb = face.embedding

                for name, reps in self.representations.items():
                    for rep in reps:
                        sim = cosine_similarity(emb, rep)
                        if sim >= threshold:
                            matches.setdefault(name, []).append(sim)

            # Early strong exit
            for name, sims in matches.items():
                if len(sims) >= 5 and np.mean(sims) > 0.6:
                    cap.release()
                    return name, float(np.mean(sims))

        cap.release()

        if not matches:
            return None, 0

        best_name = max(matches, key=lambda k: np.mean(matches[k]))
        best_score = float(np.mean(matches[best_name]))

        return best_name, best_score


logging.basicConfig(level=logging.INFO)

fr = FaceRecognizer()
# print("Adding brad...".upper())
# fr.add_person("morgan", "morgan_1.jpg")
# fr.add_person("brad", "brad_pitt.webp")
# fr.add_person("morgan", "morgan_2.jpg")
# fr.save_db()
# print("Attempting to identify")
# print(fr.identify("morgan_1.jpg"))


# print("Attempting to identify from video")
# name, score = fr.identify_from_video("tyler.mp4")
# KEY IS CRIMINAL_ID


# Replace your bottom 'for line in sys.stdin' loop with this:
for line in sys.stdin:
    line = line.strip()
    if not line:
        continue

    recv_msg = line.split(" ")
    cmd = recv_msg[0]

    try:
        if cmd == "identify":
            if len(recv_msg) < 3:
                print("error missing_path", flush=True)
                continue

            media_type = recv_msg[1]
            media_path = recv_msg[2]

            if media_type == "image":
                criminal_id, confidence = fr.identify(media_path)
            elif media_type == "video":
                criminal_id, confidence = fr.identify_from_video(media_path)
            else:
                print("error unknown_media_type", flush=True)
                continue

            out_id = criminal_id if criminal_id else "UNKNOWN"
            print(f"identity {out_id} {confidence:.4f}", flush=True)

        elif cmd == "add":
            if len(recv_msg) < 3:
                print("error missing_add_data", flush=True)
                continue

            criminal_id = recv_msg[1]
            photo_locations = recv_msg[2].split("&")

            for loc in photo_locations:
                fr.add_person(criminal_id, loc)

            fr.save_db()  # Important to persist after adding
            print(f"added {criminal_id}", flush=True)

        elif cmd == "exit":
            break

        else:
            print(f"info ignored_command {cmd}", flush=True)

    except Exception as e:
        # Catch internal errors so the loop doesn't die and cause a Broken Pipe in Rust
        print(f"error {str(e)}", flush=True)
