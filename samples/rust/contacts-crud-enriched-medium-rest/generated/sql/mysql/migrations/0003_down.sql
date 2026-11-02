-- Rollback migration for mysql
ALTER TABLE addresses ADD COLUMN `line3` VARCHAR(256);
GO
CREATE TABLE `note_pads` (`id` INT AUTO_INCREMENT NOT NULL, `uuid` CHAR(36) DEFAULT (UUID()) NOT NULL, `created` DATETIME(3) DEFAULT (UTC_TIMESTAMP(3)) NOT NULL, `updated` DATETIME(3) DEFAULT (UTC_TIMESTAMP(3)) NOT NULL, `version` BLOB DEFAULT (X'00') NOT NULL, `contact_id` INT NOT NULL, `body` TEXT, CONSTRAINT `note_pads_primary_key` PRIMARY KEY (`id`), CONSTRAINT `note_pads_uuid_unique_constraint` UNIQUE (`uuid`), CONSTRAINT `note_pads_contact_id_foreign_key` FOREIGN KEY (`contact_id`) REFERENCES `contacts` (`id`));
GO
CREATE TRIGGER `trg_note_pads_updated_at` BEFORE UPDATE ON `note_pads`
FOR EACH ROW SET NEW.`updated` = (UTC_TIMESTAMP(3));
