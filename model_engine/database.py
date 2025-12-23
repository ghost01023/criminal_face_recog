from datetime import datetime, timedelta
from typing import Optional, List
from sqlalchemy import (
    create_engine,
    Column,
    Integer,
    String,
    DateTime,
    ForeignKey,
    LargeBinary,
    func,
)
from sqlalchemy.orm import (
    declarative_base,
    relationship,
    sessionmaker,
    Session,
    validates,
)

Base = declarative_base()


class Criminal(Base):
    __tablename__ = "criminals"

    criminal_id = Column(Integer, primary_key=True, autoincrement=True)
    name = Column(String(255), nullable=False)
    fathers_name = Column(String(255), nullable=True)
    date_of_arrest = Column(DateTime, nullable=False, default=func.now())
    last_seen = Column(DateTime, nullable=True)
    no_of_crimes = Column(Integer, nullable=False, default=1)
    arrested_location = Column(String(255), nullable=True)

    photos = relationship(
        "CriminalPhoto", back_populates="criminal", cascade="all, delete-orphan"
    )

    @validates("last_seen")
    def set_last_seen(self, key, value):
        """
        Automatically set last_seen = date_of_arrest - 16 hours if not provided.
        """
        if value is None:
            if self.date_of_arrest is None:
                value = datetime.now() - timedelta(hours=16)
            else:
                value = self.date_of_arrest - timedelta(hours=16)
        return value


class CriminalPhoto(Base):
    __tablename__ = "criminal_photos"

    photo_id = Column(Integer, primary_key=True, autoincrement=True)
    criminal_id = Column(
        Integer, ForeignKey("criminals.criminal_id", ondelete="CASCADE"), nullable=False
    )
    photo = Column(LargeBinary, nullable=False)

    criminal = relationship("Criminal", back_populates="photos")


class CriminalDB:
    def __init__(self, db_url: str):
        """
        db_url example: 'mysql+mysqlconnector://user:password@localhost/crime_db'
        """
        self.engine = create_engine(db_url, echo=False)
        Base.metadata.create_all(self.engine)
        self.Session = sessionmaker(bind=self.engine)

    # ---------------------------
    # Criminal methods
    # ---------------------------

    def add_criminal(
        self,
        name: str,
        fathers_name: Optional[str] = None,
        arrested_location: Optional[str] = None,
        no_of_crimes: int = 1,
        date_of_arrest: Optional[datetime] = None,
        last_seen: Optional[datetime] = None,
    ) -> int:
        with self.Session() as session:
            criminal = Criminal(
                name=name,
                fathers_name=fathers_name,
                arrested_location=arrested_location,
                no_of_crimes=no_of_crimes,
                date_of_arrest=date_of_arrest,
                last_seen=last_seen,
            )
            session.add(criminal)
            session.commit()
            session.refresh(criminal)
            return criminal.criminal_id

    def get_criminal(self, criminal_id: int) -> Optional[Criminal]:
        with self.Session() as session:
            return session.get(Criminal, criminal_id)

    def list_criminals(self) -> List[Criminal]:
        with self.Session() as session:
            return session.query(Criminal).order_by(Criminal.criminal_id).all()

    def delete_criminal(self, criminal_id: int) -> None:
        with self.Session() as session:
            criminal = session.get(Criminal, criminal_id)
            if criminal:
                session.delete(criminal)
                session.commit()

    # ---------------------------
    # CriminalPhoto methods
    # ---------------------------

    def add_criminal_photo(self, criminal_id: int, photo_bytes: bytes) -> int:
        with self.Session() as session:
            photo = CriminalPhoto(criminal_id=criminal_id, photo=photo_bytes)
            session.add(photo)
            session.commit()
            session.refresh(photo)
            return photo.photo_id

    def get_criminal_photos(self, criminal_id: int) -> List[CriminalPhoto]:
        with self.Session() as session:
            return session.query(CriminalPhoto).filter_by(criminal_id=criminal_id).all()

    def delete_criminal_photo(self, photo_id: int) -> None:
        with self.Session() as session:
            photo = session.get(CriminalPhoto, photo_id)
            if photo:
                session.delete(photo)
                session.commit()


# ---------------------------
# Example usage
# ---------------------------

if __name__ == "__main__":
    db_url = "mysql+mysqlconnector://root:@localhost/criminal_recognizer"
    db = CriminalDB(db_url)

    # Add a criminal
    cid = db.add_criminal(
        name="John Doe", fathers_name="Robert Doe", arrested_location="New York"
    )
    print(f"Inserted criminal ID: {cid}")

    # Add a photo
    with open("salman.jpg", "rb") as f:
        photo_id = db.add_criminal_photo(criminal_id=cid, photo_bytes=f.read())
    print(f"Inserted photo ID: {photo_id}")

    # Fetch criminal
    criminal = db.get_criminal(cid)
    print(vars(criminal))

    # Fetch photos
    photos = db.get_criminal_photos(cid)
    print(f"Number of photos: {len(photos)}")

    # Delete criminal (photos deleted automatically)
    # db.delete_criminal(cid)
