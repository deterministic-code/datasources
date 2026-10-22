-- Generated migration for postgres
DROP TRIGGER IF EXISTS "trg_audit_logs_updated_at" ON "audit_logs";
DROP TABLE IF EXISTS "audit_logs";
ALTER TABLE roles DROP COLUMN "description";
