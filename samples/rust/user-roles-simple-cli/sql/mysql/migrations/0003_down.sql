-- Rollback migration for mysql
ALTER TABLE roles ADD COLUMN `description` TEXT;
CREATE TABLE `audit_logs` (`id` INT AUTO_INCREMENT NOT NULL, `message` TEXT NOT NULL, `created` DATETIME(3) NOT NULL, `updated` BLOB CONSTRAINT `audit_logs_updated_default_constraint` DEFAULT X'00' NOT NULL, CONSTRAINT `audit_logs_primary_key` PRIMARY KEY (`id`));
CREATE TRIGGER `trg_audit_logs_updated_at` BEFORE UPDATE ON `audit_logs`
FOR EACH ROW SET NEW.`updated` = (UTC_TIMESTAMP(3));
