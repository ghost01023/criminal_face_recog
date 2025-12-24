

-- Trigger for date_of_arrest default

DELIMITER $$

CREATE DEFINER=`root`@`localhost`
TRIGGER trg_criminals_before_insert
BEFORE INSERT ON criminals
FOR EACH ROW
BEGIN
    IF NEW.last_seen IS NULL THEN
        SET NEW.last_seen = NEW.date_of_arrest - INTERVAL 16 HOUR;
    END IF;
END$$

DELIMITER ;
