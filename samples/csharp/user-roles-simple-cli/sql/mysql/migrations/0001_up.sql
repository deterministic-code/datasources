-- Generated migration for mysql
CREATE TABLE `users` (`id` INT AUTO_INCREMENT NOT NULL, `username` TEXT NOT NULL, `password` TEXT NOT NULL, `nickname` TEXT, `is_enabled` TINYINT(1) CONSTRAINT `users_is_enabled_default_constraint` DEFAULT 1 NOT NULL, `created` DATETIME(3) NOT NULL, `updated` BLOB CONSTRAINT `users_updated_default_constraint` DEFAULT X'00' NOT NULL, CONSTRAINT `users_primary_key` PRIMARY KEY (`id`));
CREATE TRIGGER `trg_users_updated_at` BEFORE UPDATE ON `users`
FOR EACH ROW SET NEW.`updated` = (UTC_TIMESTAMP(3));
CREATE TABLE `roles` (`id` INT AUTO_INCREMENT NOT NULL, `name` TEXT NOT NULL, `description` TEXT, `is_enabled` TINYINT(1) CONSTRAINT `roles_is_enabled_default_constraint` DEFAULT 1 NOT NULL, `created` DATETIME(3) NOT NULL, `updated` BLOB CONSTRAINT `roles_updated_default_constraint` DEFAULT X'00' NOT NULL, CONSTRAINT `roles_primary_key` PRIMARY KEY (`id`));
CREATE TRIGGER `trg_roles_updated_at` BEFORE UPDATE ON `roles`
FOR EACH ROW SET NEW.`updated` = (UTC_TIMESTAMP(3));
CREATE TABLE `groups` (`id` INT AUTO_INCREMENT NOT NULL, `ugly_name` TEXT NOT NULL, `is_enabled` TINYINT(1) CONSTRAINT `groups_is_enabled_default_constraint` DEFAULT 1 NOT NULL, `created` DATETIME(3) NOT NULL, `updated` BLOB CONSTRAINT `groups_updated_default_constraint` DEFAULT X'00' NOT NULL, CONSTRAINT `groups_primary_key` PRIMARY KEY (`id`));
CREATE TRIGGER `trg_groups_updated_at` BEFORE UPDATE ON `groups`
FOR EACH ROW SET NEW.`updated` = (UTC_TIMESTAMP(3));
CREATE TABLE `user_groups` (`id` INT AUTO_INCREMENT NOT NULL, `user_id` TEXT NOT NULL, `group_id` TEXT NOT NULL, `created` DATETIME(3) NOT NULL, `updated` BLOB CONSTRAINT `user_groups_updated_default_constraint` DEFAULT X'00' NOT NULL, CONSTRAINT `user_groups_primary_key` PRIMARY KEY (`id`), CONSTRAINT `user_groups_user_id_foreign_key` FOREIGN KEY (`user_id`) REFERENCES `users` (`id`), CONSTRAINT `user_groups_group_id_foreign_key` FOREIGN KEY (`group_id`) REFERENCES `groups` (`id`));
CREATE TRIGGER `trg_user_groups_updated_at` BEFORE UPDATE ON `user_groups`
FOR EACH ROW SET NEW.`updated` = (UTC_TIMESTAMP(3));
CREATE TABLE `user_roles` (`id` INT AUTO_INCREMENT NOT NULL, `user_id` TEXT NOT NULL, `bad_role_id` TEXT NOT NULL, `created` DATETIME(3) NOT NULL, `updated` BLOB CONSTRAINT `user_roles_updated_default_constraint` DEFAULT X'00' NOT NULL, CONSTRAINT `user_roles_primary_key` PRIMARY KEY (`id`), CONSTRAINT `user_roles_user_id_foreign_key` FOREIGN KEY (`user_id`) REFERENCES `users` (`id`), CONSTRAINT `user_roles_bad_role_id_foreign_key` FOREIGN KEY (`bad_role_id`) REFERENCES `roles` (`id`));
CREATE TRIGGER `trg_user_roles_updated_at` BEFORE UPDATE ON `user_roles`
FOR EACH ROW SET NEW.`updated` = (UTC_TIMESTAMP(3));
CREATE TABLE `sessions` (`id` INT AUTO_INCREMENT NOT NULL, `user_id` TEXT NOT NULL, `token` TEXT NOT NULL, `created` DATETIME(3) NOT NULL, `updated` BLOB CONSTRAINT `sessions_updated_default_constraint` DEFAULT X'00' NOT NULL, CONSTRAINT `sessions_primary_key` PRIMARY KEY (`id`), CONSTRAINT `sessions_user_id_foreign_key` FOREIGN KEY (`user_id`) REFERENCES `users` (`id`));
CREATE TRIGGER `trg_sessions_updated_at` BEFORE UPDATE ON `sessions`
FOR EACH ROW SET NEW.`updated` = (UTC_TIMESTAMP(3));
CREATE TABLE `audit_logs` (`id` INT AUTO_INCREMENT NOT NULL, `message` TEXT NOT NULL, `created` DATETIME(3) NOT NULL, `updated` BLOB CONSTRAINT `audit_logs_updated_default_constraint` DEFAULT X'00' NOT NULL, CONSTRAINT `audit_logs_primary_key` PRIMARY KEY (`id`));
CREATE TRIGGER `trg_audit_logs_updated_at` BEFORE UPDATE ON `audit_logs`
FOR EACH ROW SET NEW.`updated` = (UTC_TIMESTAMP(3));
