import insightface
import numpy as np
import cv2
import logging
import contextlib
import os


logger = logging.getLogger(__name__)


class FaceRecognizer:
    def __init__(self, db_path="faces_db.npz"):
        self.db_path = db_path
        print("Initializing AI models (silently)...")
        with open(os.devnull, "w") as fnull:
            with contextlib.redirect_stdout(fnull), contextlib.redirect_stderr(fnull):
                self.app = insightface.app.FaceAnalysis(
                    name="buffalo_l", providers=["CUDAExecutionProvider"]
                )

                self.app.prepare(ctx_id=0, det_size=(640, 640))

        print("AI System Ready.")
        self.known_faces = {}
        self.load_db()

    def load_db(self):
        if not os.path.exists(self.db_path):
            logger.info("Couldn't find local model KB. Initialized fresh one.")
            return

        logger.info("Loading local model KB.")
        data = np.load(self.db_path, allow_pickle=False)
        self.known_faces = {k: data[k] for k in data.files}

    def save_db(self):
        np.savez(self.db_path, **self.known_faces)

    def add_person(self, name, image_path):
        img = cv2.imread(image_path)
        faces = self.app.get(img)
        if faces:
            self.known_faces[name] = faces[0].embedding

    def identify(self, image_path, threshold=0.4):
        img = cv2.imread(image_path)
        faces = self.app.get(img)

        if not faces:
            return None

        test_embedding = faces[0].embedding

        best_match = None
        best_score = threshold

        for name, known_embedding in self.known_faces.items():
            # Cosine similarity
            similarity = np.dot(test_embedding, known_embedding) / (
                np.linalg.norm(test_embedding) * np.linalg.norm(known_embedding)
            )

            if similarity > best_score:
                best_score = similarity
                best_match = name

        return best_match, best_score if best_match else (None, 0)


logging.basicConfig(level=logging.INFO)

fr = FaceRecognizer()
# print("Adding brad...".upper())
# fr.add_person("salman", "salman.jpg")
# fr.add_person("brad", "brad_pitt.webp")
fr.save_db()
print("Attempting to identify")
print(fr.identify("salman.jpg"))
