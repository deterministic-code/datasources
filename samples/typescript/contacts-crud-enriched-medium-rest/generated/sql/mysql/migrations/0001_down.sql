-- Rollback migration for mysql
DROP TABLE IF EXISTS `contact_change_logs`;
GO
DROP TABLE IF EXISTS `OldContactsTbl`;
GO
DROP TRIGGER IF EXISTS `trg_contact_group_members_updated_at`;
GO
DROP TABLE IF EXISTS `contact_group_members`;
GO
DROP TRIGGER IF EXISTS `trg_contact_preferences_updated_at`;
GO
DROP INDEX `contact_preferences_owner_key_index` ON `contact_preferences`;
GO
DROP TABLE IF EXISTS `contact_preferences`;
GO
DROP TRIGGER IF EXISTS `trg_contact_settings_updated_at`;
GO
DROP INDEX `contact_settings_owner_key_index` ON `contact_settings`;
GO
DROP TABLE IF EXISTS `contact_settings`;
GO
DROP TRIGGER IF EXISTS `trg_contact_groups_updated_at`;
GO
DROP TABLE IF EXISTS `contact_groups`;
GO
DROP TRIGGER IF EXISTS `trg_note_pads_updated_at`;
GO
DROP TABLE IF EXISTS `note_pads`;
GO
DROP TRIGGER IF EXISTS `trg_tags_updated_at`;
GO
DROP TABLE IF EXISTS `tags`;
GO
DROP TRIGGER IF EXISTS `trg_phones_updated_at`;
GO
DROP TABLE IF EXISTS `phones`;
GO
DROP TRIGGER IF EXISTS `trg_addresses_updated_at`;
GO
DROP TABLE IF EXISTS `addresses`;
GO
DROP TRIGGER IF EXISTS `trg_contacts_updated_at`;
GO
DROP TABLE IF EXISTS `contacts`;
GO
DROP TRIGGER IF EXISTS `trg_phone_types_updated_at`;
GO
DROP TABLE IF EXISTS `phone_types`;
GO
DROP TRIGGER IF EXISTS `trg_address_types_updated_at`;
GO
DROP TABLE IF EXISTS `address_types`;
GO
DROP TRIGGER IF EXISTS `trg_contact_sources_updated_at`;
GO
DROP TABLE IF EXISTS `contact_sources`;
