-- Generated migration for sqlite
DROP TRIGGER IF EXISTS "trg_sessions_updated_at";
DROP TABLE IF EXISTS "sessions";
ALTER TABLE users DROP COLUMN "nickname";
