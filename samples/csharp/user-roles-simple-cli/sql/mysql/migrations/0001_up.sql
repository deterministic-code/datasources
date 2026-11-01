-- Generated migration for mysql
CREATE TABLE `users` (`id` INT AUTO_INCREMENT NOT NULL, `uuid` CHAR(36) DEFAULT (UUID()) NOT NULL, `created` DATETIME(3) NOT NULL, `updated` BLOB DEFAULT (X'00') NOT NULL, `username` TEXT NOT NULL, `password` TEXT NOT NULL, `is_enabled` TINYINT(1) DEFAULT 1 NOT NULL, CONSTRAINT `users_primary_key` PRIMARY KEY (`id`), CONSTRAINT `users_uuid_unique_constraint` UNIQUE (`uuid`));
GO
CREATE TRIGGER `trg_users_updated_at` BEFORE UPDATE ON `users`
FOR EACH ROW SET NEW.`updated` = (UTC_TIMESTAMP(3));
GO
CREATE TABLE `roles` (`id` INT AUTO_INCREMENT NOT NULL, `uuid` CHAR(36) DEFAULT (UUID()) NOT NULL, `created` DATETIME(3) NOT NULL, `updated` BLOB DEFAULT (X'00') NOT NULL, `name` TEXT NOT NULL, `description` TEXT DEFAULT ('') NOT NULL, `is_enabled` TINYINT(1) DEFAULT 1 NOT NULL, CONSTRAINT `roles_primary_key` PRIMARY KEY (`id`), CONSTRAINT `roles_uuid_unique_constraint` UNIQUE (`uuid`));
GO
CREATE TRIGGER `trg_roles_updated_at` BEFORE UPDATE ON `roles`
FOR EACH ROW SET NEW.`updated` = (UTC_TIMESTAMP(3));
GO
CREATE TABLE `groups` (`id` INT AUTO_INCREMENT NOT NULL, `uuid` CHAR(36) DEFAULT (UUID()) NOT NULL, `created` DATETIME(3) NOT NULL, `updated` BLOB DEFAULT (X'00') NOT NULL, `ugly_name` TEXT NOT NULL, `description` TEXT DEFAULT ('') NOT NULL, `is_enabled` TINYINT(1) DEFAULT 1 NOT NULL, CONSTRAINT `groups_primary_key` PRIMARY KEY (`id`), CONSTRAINT `groups_uuid_unique_constraint` UNIQUE (`uuid`));
GO
CREATE TRIGGER `trg_groups_updated_at` BEFORE UPDATE ON `groups`
FOR EACH ROW SET NEW.`updated` = (UTC_TIMESTAMP(3));
GO
CREATE TABLE `user_groups` (`id` INT AUTO_INCREMENT NOT NULL, `uuid` CHAR(36) DEFAULT (UUID()) NOT NULL, `created` DATETIME(3) NOT NULL, `updated` BLOB DEFAULT (X'00') NOT NULL, `user_id` INT NOT NULL, `group_id` INT NOT NULL, CONSTRAINT `user_groups_primary_key` PRIMARY KEY (`id`), CONSTRAINT `user_groups_uuid_unique_constraint` UNIQUE (`uuid`), CONSTRAINT `user_groups_user_id_foreign_key` FOREIGN KEY (`user_id`) REFERENCES `users` (`id`), CONSTRAINT `user_groups_group_id_foreign_key` FOREIGN KEY (`group_id`) REFERENCES `groups` (`id`));
GO
CREATE TRIGGER `trg_user_groups_updated_at` BEFORE UPDATE ON `user_groups`
FOR EACH ROW SET NEW.`updated` = (UTC_TIMESTAMP(3));
GO
CREATE TABLE `user_roles` (`id` INT AUTO_INCREMENT NOT NULL, `uuid` CHAR(36) DEFAULT (UUID()) NOT NULL, `created` DATETIME(3) NOT NULL, `updated` BLOB DEFAULT (X'00') NOT NULL, `user_id` INT NOT NULL, `bad_role_id` INT NOT NULL, CONSTRAINT `user_roles_primary_key` PRIMARY KEY (`id`), CONSTRAINT `user_roles_uuid_unique_constraint` UNIQUE (`uuid`), CONSTRAINT `user_roles_user_id_foreign_key` FOREIGN KEY (`user_id`) REFERENCES `users` (`id`), CONSTRAINT `user_roles_bad_role_id_foreign_key` FOREIGN KEY (`bad_role_id`) REFERENCES `roles` (`id`));
GO
CREATE TRIGGER `trg_user_roles_updated_at` BEFORE UPDATE ON `user_roles`
FOR EACH ROW SET NEW.`updated` = (UTC_TIMESTAMP(3));
