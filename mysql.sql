CREATE TABLE criminals (
    criminal_id INT(10) UNSIGNED NOT NULL AUTO_INCREMENT,
    name VARCHAR(255) NOT NULL,
    fathers_name VARCHAR(255) DEFAULT NULL,
    date_of_arrest DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    last_seen DATETIME DEFAULT NULL,
    no_of_crimes INT(10) UNSIGNED NOT NULL DEFAULT 1,
    arrested_location VARCHAR(255) DEFAULT NULL,
    PRIMARY KEY (criminal_id)
) ENGINE=InnoDB
DEFAULT CHARSET=utf8mb4
COLLATE=utf8mb4_unicode_ci;

CREATE TABLE criminal_photos (
    photo_id INT(10) UNSIGNED NOT NULL AUTO_INCREMENT,
    criminal_id INT(10) UNSIGNED NOT NULL,
    photo LONGBLOB NOT NULL,
    PRIMARY KEY (photo_id),
    KEY criminal_id (criminal_id),
    CONSTRAINT fk_criminal_photos_criminals
        FOREIGN KEY (criminal_id)
        REFERENCES criminals (criminal_id)
        ON DELETE CASCADE
) ENGINE=InnoDB
DEFAULT CHARSET=utf8mb4
COLLATE=utf8mb4_unicode_ci;


DELIMITER $$

CREATE TRIGGER trg_criminals_before_insert
BEFORE INSERT ON criminals
FOR EACH ROW
BEGIN
    IF NEW.last_seen IS NULL THEN
        SET NEW.last_seen = NEW.date_of_arrest - INTERVAL 16 HOUR;
    END IF;
END$$

DELIMITER ;

