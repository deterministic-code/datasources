-- Rollback initial migration for sqlite

-- DROP tables (reverse dependency order)
DROP TABLE IF EXISTS "contact_change_logs";
DROP TABLE IF EXISTS "OldContactsTbl";
DROP TABLE IF EXISTS "contact_group_members";
DROP TABLE IF EXISTS "contact_preferences";
DROP TABLE IF EXISTS "contact_settings";
DROP TABLE IF EXISTS "contact_groups";
DROP TABLE IF EXISTS "phones";
DROP TABLE IF EXISTS "addresses";
DROP TABLE IF EXISTS "contacts";
DROP TABLE IF EXISTS "phone_types";
DROP TABLE IF EXISTS "address_types";
DROP TABLE IF EXISTS "contact_sources";
