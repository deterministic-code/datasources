-- Generated migration for postgres
DROP TRIGGER IF EXISTS "trg_sessions_updated_at" ON "sessions";
DROP TABLE IF EXISTS "sessions";
ALTER TABLE users DROP COLUMN "nickname";
