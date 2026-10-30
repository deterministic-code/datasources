-- Rollback migration for mysql
ALTER TABLE contacts ADD COLUMN `middle_name` VARCHAR(128);
GO
CREATE TABLE `tags` (`id` INT AUTO_INCREMENT NOT NULL, `uuid` CHAR(36) DEFAULT (UUID()) NOT NULL, `created` DATETIME(3) DEFAULT (UTC_TIMESTAMP(3)) NOT NULL, `updated` DATETIME(3) DEFAULT (UTC_TIMESTAMP(3)) NOT NULL, `version` BLOB DEFAULT (X'00') NOT NULL, `contact_id` INT NOT NULL, `name` VARCHAR(64) NOT NULL, CONSTRAINT `tags_primary_key` PRIMARY KEY (`id`), CONSTRAINT `tags_uuid_unique_constraint` UNIQUE (`uuid`), CONSTRAINT `tags_contact_id_foreign_key` FOREIGN KEY (`contact_id`) REFERENCES `contacts` (`id`));
GO
CREATE TRIGGER `trg_tags_updated_at` BEFORE UPDATE ON `tags`
FOR EACH ROW SET NEW.`updated` = (UTC_TIMESTAMP(3));
