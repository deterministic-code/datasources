-- Generated migration for postgres
DROP TRIGGER IF EXISTS "trg_tags_updated_at" ON "tags";
GO
DROP TABLE IF EXISTS "tags";
GO
ALTER TABLE contacts DROP COLUMN "middle_name";
