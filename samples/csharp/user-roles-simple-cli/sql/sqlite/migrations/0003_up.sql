-- Generated migration for sqlite
DROP TRIGGER IF EXISTS "trg_audit_logs_updated_at";
DROP TABLE IF EXISTS "audit_logs";
ALTER TABLE roles DROP COLUMN "description";
