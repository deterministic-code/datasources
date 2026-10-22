-- Rollback migration for mysql
ALTER TABLE users ADD COLUMN `nickname` TEXT;
CREATE TABLE `sessions` (`id` INT AUTO_INCREMENT NOT NULL, `user_id` TEXT NOT NULL, `token` TEXT NOT NULL, `created` DATETIME(3) NOT NULL, `updated` BLOB CONSTRAINT `sessions_updated_default_constraint` DEFAULT X'00' NOT NULL, CONSTRAINT `sessions_primary_key` PRIMARY KEY (`id`), CONSTRAINT `sessions_user_id_foreign_key` FOREIGN KEY (`user_id`) REFERENCES `users` (`id`));
CREATE TRIGGER `trg_sessions_updated_at` BEFORE UPDATE ON `sessions`
FOR EACH ROW SET NEW.`updated` = (UTC_TIMESTAMP(3));
