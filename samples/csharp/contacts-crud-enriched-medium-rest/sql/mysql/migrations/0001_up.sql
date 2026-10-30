-- Generated migration for mysql
CREATE TABLE `contact_sources` (`id` INT AUTO_INCREMENT NOT NULL, `uuid` CHAR(36) DEFAULT (UUID()) NOT NULL, `created` DATETIME(3) DEFAULT (UTC_TIMESTAMP(3)) NOT NULL, `updated` DATETIME(3) DEFAULT (UTC_TIMESTAMP(3)) NOT NULL, `version` BLOB DEFAULT (X'00') NOT NULL, `name` VARCHAR(64) NOT NULL, `description` TEXT, CONSTRAINT `contact_sources_primary_key` PRIMARY KEY (`id`), CONSTRAINT `contact_sources_uuid_unique_constraint` UNIQUE (`uuid`), CONSTRAINT `contact_sources_name_unique_constraint` UNIQUE (`name`));
GO
CREATE TRIGGER `trg_contact_sources_updated_at` BEFORE UPDATE ON `contact_sources`
FOR EACH ROW SET NEW.`updated` = (UTC_TIMESTAMP(3));
GO
CREATE TABLE `address_types` (`id` INT AUTO_INCREMENT NOT NULL, `uuid` CHAR(36) DEFAULT (UUID()) NOT NULL, `created` DATETIME(3) DEFAULT (UTC_TIMESTAMP(3)) NOT NULL, `updated` DATETIME(3) DEFAULT (UTC_TIMESTAMP(3)) NOT NULL, `version` BLOB DEFAULT (X'00') NOT NULL, `name` VARCHAR(64) NOT NULL, `description` TEXT, CONSTRAINT `address_types_primary_key` PRIMARY KEY (`id`), CONSTRAINT `address_types_uuid_unique_constraint` UNIQUE (`uuid`));
GO
CREATE TRIGGER `trg_address_types_updated_at` BEFORE UPDATE ON `address_types`
FOR EACH ROW SET NEW.`updated` = (UTC_TIMESTAMP(3));
GO
CREATE TABLE `phone_types` (`id` INT AUTO_INCREMENT NOT NULL, `uuid` CHAR(36) DEFAULT (UUID()) NOT NULL, `created` DATETIME(3) DEFAULT (UTC_TIMESTAMP(3)) NOT NULL, `updated` DATETIME(3) DEFAULT (UTC_TIMESTAMP(3)) NOT NULL, `version` BLOB DEFAULT (X'00') NOT NULL, `name` VARCHAR(64) NOT NULL, `description` TEXT, CONSTRAINT `phone_types_primary_key` PRIMARY KEY (`id`), CONSTRAINT `phone_types_uuid_unique_constraint` UNIQUE (`uuid`));
GO
CREATE TRIGGER `trg_phone_types_updated_at` BEFORE UPDATE ON `phone_types`
FOR EACH ROW SET NEW.`updated` = (UTC_TIMESTAMP(3));
GO
CREATE TABLE `contacts` (`id` INT AUTO_INCREMENT NOT NULL, `uuid` CHAR(36) DEFAULT (UUID()) NOT NULL, `created` DATETIME(3) DEFAULT (UTC_TIMESTAMP(3)) NOT NULL, `updated` DATETIME(3) DEFAULT (UTC_TIMESTAMP(3)) NOT NULL, `version` BLOB DEFAULT (X'00') NOT NULL, `contact_source_id` INT NOT NULL, `first_name` VARCHAR(128) NOT NULL, `last_name` VARCHAR(128) NOT NULL, `middle_name` VARCHAR(128), `email` VARCHAR(256), `notes` TEXT, CONSTRAINT `contacts_primary_key` PRIMARY KEY (`id`), CONSTRAINT `contacts_uuid_unique_constraint` UNIQUE (`uuid`), CONSTRAINT `contacts_contact_source_id_foreign_key` FOREIGN KEY (`contact_source_id`) REFERENCES `contact_sources` (`id`));
GO
CREATE TRIGGER `trg_contacts_updated_at` BEFORE UPDATE ON `contacts`
FOR EACH ROW SET NEW.`updated` = (UTC_TIMESTAMP(3));
GO
CREATE TABLE `addresses` (`id` INT AUTO_INCREMENT NOT NULL, `uuid` CHAR(36) DEFAULT (UUID()) NOT NULL, `created` DATETIME(3) DEFAULT (UTC_TIMESTAMP(3)) NOT NULL, `updated` DATETIME(3) DEFAULT (UTC_TIMESTAMP(3)) NOT NULL, `version` BLOB DEFAULT (X'00') NOT NULL, `contact_id` INT NOT NULL, `address_type_id` INT NOT NULL, `line1` VARCHAR(256) NOT NULL, `line2` VARCHAR(256), `line3` VARCHAR(256), `city` VARCHAR(128) NOT NULL, `region` VARCHAR(128), `postal_code` VARCHAR(32), `country` VARCHAR(64), CONSTRAINT `addresses_primary_key` PRIMARY KEY (`id`), CONSTRAINT `addresses_uuid_unique_constraint` UNIQUE (`uuid`), CONSTRAINT `addresses_contact_id_foreign_key` FOREIGN KEY (`contact_id`) REFERENCES `contacts` (`id`), CONSTRAINT `addresses_address_type_id_foreign_key` FOREIGN KEY (`address_type_id`) REFERENCES `address_types` (`id`));
GO
CREATE TRIGGER `trg_addresses_updated_at` BEFORE UPDATE ON `addresses`
FOR EACH ROW SET NEW.`updated` = (UTC_TIMESTAMP(3));
GO
CREATE TABLE `phones` (`id` INT AUTO_INCREMENT NOT NULL, `uuid` CHAR(36) DEFAULT (UUID()) NOT NULL, `created` DATETIME(3) DEFAULT (UTC_TIMESTAMP(3)) NOT NULL, `updated` DATETIME(3) DEFAULT (UTC_TIMESTAMP(3)) NOT NULL, `version` BLOB DEFAULT (X'00') NOT NULL, `contact_id` INT NOT NULL, `phone_type_id` INT NOT NULL, `number` VARCHAR(32) NOT NULL, `label` VARCHAR(32), CONSTRAINT `phones_primary_key` PRIMARY KEY (`id`), CONSTRAINT `phones_uuid_unique_constraint` UNIQUE (`uuid`), CONSTRAINT `phones_contact_id_foreign_key` FOREIGN KEY (`contact_id`) REFERENCES `contacts` (`id`), CONSTRAINT `phones_phone_type_id_foreign_key` FOREIGN KEY (`phone_type_id`) REFERENCES `phone_types` (`id`));
GO
CREATE TRIGGER `trg_phones_updated_at` BEFORE UPDATE ON `phones`
FOR EACH ROW SET NEW.`updated` = (UTC_TIMESTAMP(3));
GO
CREATE TABLE `tags` (`id` INT AUTO_INCREMENT NOT NULL, `uuid` CHAR(36) DEFAULT (UUID()) NOT NULL, `created` DATETIME(3) DEFAULT (UTC_TIMESTAMP(3)) NOT NULL, `updated` DATETIME(3) DEFAULT (UTC_TIMESTAMP(3)) NOT NULL, `version` BLOB DEFAULT (X'00') NOT NULL, `contact_id` INT NOT NULL, `name` VARCHAR(64) NOT NULL, CONSTRAINT `tags_primary_key` PRIMARY KEY (`id`), CONSTRAINT `tags_uuid_unique_constraint` UNIQUE (`uuid`), CONSTRAINT `tags_contact_id_foreign_key` FOREIGN KEY (`contact_id`) REFERENCES `contacts` (`id`));
GO
CREATE TRIGGER `trg_tags_updated_at` BEFORE UPDATE ON `tags`
FOR EACH ROW SET NEW.`updated` = (UTC_TIMESTAMP(3));
GO
CREATE TABLE `note_pads` (`id` INT AUTO_INCREMENT NOT NULL, `uuid` CHAR(36) DEFAULT (UUID()) NOT NULL, `created` DATETIME(3) DEFAULT (UTC_TIMESTAMP(3)) NOT NULL, `updated` DATETIME(3) DEFAULT (UTC_TIMESTAMP(3)) NOT NULL, `version` BLOB DEFAULT (X'00') NOT NULL, `contact_id` INT NOT NULL, `body` TEXT, CONSTRAINT `note_pads_primary_key` PRIMARY KEY (`id`), CONSTRAINT `note_pads_uuid_unique_constraint` UNIQUE (`uuid`), CONSTRAINT `note_pads_contact_id_foreign_key` FOREIGN KEY (`contact_id`) REFERENCES `contacts` (`id`));
GO
CREATE TRIGGER `trg_note_pads_updated_at` BEFORE UPDATE ON `note_pads`
FOR EACH ROW SET NEW.`updated` = (UTC_TIMESTAMP(3));
GO
CREATE TABLE `contact_groups` (`id` INT AUTO_INCREMENT NOT NULL, `uuid` CHAR(36) DEFAULT (UUID()) NOT NULL, `created` DATETIME(3) DEFAULT (UTC_TIMESTAMP(3)) NOT NULL, `updated` DATETIME(3) DEFAULT (UTC_TIMESTAMP(3)) NOT NULL, `version` BLOB DEFAULT (X'00') NOT NULL, `name` VARCHAR(128) NOT NULL, `description` TEXT, CONSTRAINT `contact_groups_primary_key` PRIMARY KEY (`id`), CONSTRAINT `contact_groups_uuid_unique_constraint` UNIQUE (`uuid`), CONSTRAINT `contact_groups_name_unique_constraint` UNIQUE (`name`));
GO
CREATE TRIGGER `trg_contact_groups_updated_at` BEFORE UPDATE ON `contact_groups`
FOR EACH ROW SET NEW.`updated` = (UTC_TIMESTAMP(3));
GO
CREATE TABLE `contact_settings` (`id` INT AUTO_INCREMENT NOT NULL, `uuid` CHAR(36) DEFAULT (UUID()) NOT NULL, `created` DATETIME(3) DEFAULT (UTC_TIMESTAMP(3)) NOT NULL, `updated` DATETIME(3) DEFAULT (UTC_TIMESTAMP(3)) NOT NULL, `version` BLOB DEFAULT (X'00') NOT NULL, `contact_id` INT NOT NULL, `key` VARCHAR(64) NOT NULL, `value` TEXT NOT NULL, CONSTRAINT `contact_settings_primary_key` PRIMARY KEY (`id`), CONSTRAINT `contact_settings_uuid_unique_constraint` UNIQUE (`uuid`), CONSTRAINT `contact_settings_contact_id_foreign_key` FOREIGN KEY (`contact_id`) REFERENCES `contacts` (`id`));
GO
CREATE UNIQUE INDEX `contact_settings_owner_key_index` ON `contact_settings` (`contact_id`, `key`);
GO
CREATE TRIGGER `trg_contact_settings_updated_at` BEFORE UPDATE ON `contact_settings`
FOR EACH ROW SET NEW.`updated` = (UTC_TIMESTAMP(3));
GO
CREATE TABLE `contact_preferences` (`id` INT AUTO_INCREMENT NOT NULL, `uuid` CHAR(36) DEFAULT (UUID()) NOT NULL, `created` DATETIME(3) DEFAULT (UTC_TIMESTAMP(3)) NOT NULL, `updated` DATETIME(3) DEFAULT (UTC_TIMESTAMP(3)) NOT NULL, `version` BLOB DEFAULT (X'00') NOT NULL, `contact_id` INT NOT NULL, `key` VARCHAR(64) NOT NULL, `locale` VARCHAR(16) NOT NULL, `timezone` VARCHAR(64) NOT NULL, CONSTRAINT `contact_preferences_primary_key` PRIMARY KEY (`id`), CONSTRAINT `contact_preferences_uuid_unique_constraint` UNIQUE (`uuid`), CONSTRAINT `contact_preferences_contact_id_foreign_key` FOREIGN KEY (`contact_id`) REFERENCES `contacts` (`id`));
GO
CREATE UNIQUE INDEX `contact_preferences_owner_key_index` ON `contact_preferences` (`contact_id`, `key`);
GO
CREATE TRIGGER `trg_contact_preferences_updated_at` BEFORE UPDATE ON `contact_preferences`
FOR EACH ROW SET NEW.`updated` = (UTC_TIMESTAMP(3));
GO
CREATE TABLE `contact_group_members` (`id` INT AUTO_INCREMENT NOT NULL, `uuid` CHAR(36) DEFAULT (UUID()) NOT NULL, `created` DATETIME(3) DEFAULT (UTC_TIMESTAMP(3)) NOT NULL, `updated` DATETIME(3) DEFAULT (UTC_TIMESTAMP(3)) NOT NULL, `version` BLOB DEFAULT (X'00') NOT NULL, `contact_id` INT NOT NULL, `contact_group_id` INT NOT NULL, CONSTRAINT `contact_group_members_primary_key` PRIMARY KEY (`id`), CONSTRAINT `contact_group_members_uuid_unique_constraint` UNIQUE (`uuid`), CONSTRAINT `contact_group_members_contact_id_foreign_key` FOREIGN KEY (`contact_id`) REFERENCES `contacts` (`id`), CONSTRAINT `contact_group_members_contact_group_id_foreign_key` FOREIGN KEY (`contact_group_id`) REFERENCES `contact_groups` (`id`));
GO
CREATE TRIGGER `trg_contact_group_members_updated_at` BEFORE UPDATE ON `contact_group_members`
FOR EACH ROW SET NEW.`updated` = (UTC_TIMESTAMP(3));
GO
CREATE TABLE `OldContactsTbl` (`CntID` VARCHAR(64) NOT NULL, `FirstNm` VARCHAR(128) NOT NULL, `LastNm` VARCHAR(128) NOT NULL, `EmailAddr` VARCHAR(256), `ImpDate` DATETIME(3), CONSTRAINT `OldContactsTbl_primary_key` PRIMARY KEY (`CntID`), CONSTRAINT `OldContactsTbl_CntID_unique_constraint` UNIQUE (`CntID`));
GO
CREATE TABLE `contact_change_logs` (`id` INT NOT NULL, `occurred_at` DATETIME(3) NOT NULL, `contact_id` INT NOT NULL, `changed_by` VARCHAR(128) NOT NULL, `change_summary` TEXT NOT NULL, CONSTRAINT `contact_change_logs_primary_key` PRIMARY KEY (`id`));
-- Seed data
-- Seeds: contact_sources
GO
INSERT INTO `contact_sources` (`id`, `uuid`, `name`) VALUES (1, '1d3e841c-6a99-5e61-8283-879f22e8d932', 'Manual');
GO
INSERT INTO `contact_sources` (`id`, `uuid`, `name`, `description`) VALUES (2, '38dd404e-9605-5681-a425-678204a8ae48', 'Imported', 'Loaded from a CSV or vCard import.');
GO
INSERT INTO `contact_sources` (`id`, `uuid`, `name`, `description`) VALUES (3, 'ef86312e-ba98-502a-90c7-96af9357a65b', 'OAuth', 'Synced from a third-party provider.');
GO
-- Seeds: address_types
GO
INSERT INTO `address_types` (`id`, `uuid`, `name`) VALUES (1, '1e75343a-f291-5115-a555-47c1f64a2c69', 'Home');
GO
-- Seeds: phone_types
GO
INSERT INTO `phone_types` (`id`, `uuid`, `name`) VALUES (1, '775f24d4-e8cb-505c-9671-1d53c44f2780', 'Mobile');
GO
-- Seeds: contact_groups
GO
INSERT INTO `contact_groups` (`id`, `uuid`, `name`, `description`) VALUES (1, 'd02c4d97-b6bf-5ac2-b735-9721f41896f3', 'Friends', 'Friends');
GO
INSERT INTO `contact_groups` (`id`, `uuid`, `name`, `description`) VALUES (2, '102bdd01-31a2-5663-b368-561bb4101044', 'Work', 'Work contacts');
