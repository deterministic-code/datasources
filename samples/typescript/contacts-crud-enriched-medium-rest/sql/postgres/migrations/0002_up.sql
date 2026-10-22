-- Generated migration for postgres
DROP TRIGGER IF EXISTS "trg_tags_updated_at" ON "tags";
DROP TABLE IF EXISTS "tags";
ALTER TABLE contacts DROP COLUMN "middle_name";
