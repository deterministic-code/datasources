-- Generated migration for mysql
ALTER TABLE addresses ADD COLUMN `latitude` INT;
GO
ALTER TABLE addresses ADD COLUMN `label_note` VARCHAR(128);
GO
ALTER TABLE addresses ADD COLUMN `floor` INT UNSIGNED;
GO
ALTER TABLE addresses ADD COLUMN `score` DECIMAL(10, 2);
GO
ALTER TABLE addresses ADD COLUMN `external_id` CHAR(36);
GO
ALTER TABLE addresses ADD COLUMN `is_verified` TINYINT(1);
GO
ALTER TABLE addresses ADD COLUMN `verified_at` DATETIME(3);
GO
ALTER TABLE addresses ADD COLUMN `stamp` BLOB;
