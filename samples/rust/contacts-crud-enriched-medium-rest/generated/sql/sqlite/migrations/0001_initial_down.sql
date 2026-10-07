-- Rollback initial migration for sqlite

-- DROP tables (reverse dependency order)
DROP TABLE IF EXISTS "legacy_migration_results";
DROP TABLE IF EXISTS "import_contacts_results";
DROP TABLE IF EXISTS "import_contacts_requests";
DROP TABLE IF EXISTS "contact_change_logs";
DROP TABLE IF EXISTS "OldContactsTbl";
DROP TABLE IF EXISTS "contact_group_members";
DROP TABLE IF EXISTS "contact_groups";
DROP TABLE IF EXISTS "phones";
DROP TABLE IF EXISTS "addresses";
DROP TABLE IF EXISTS "contacts";
DROP TABLE IF EXISTS "contact_sources";
DROP TABLE IF EXISTS "bases";
