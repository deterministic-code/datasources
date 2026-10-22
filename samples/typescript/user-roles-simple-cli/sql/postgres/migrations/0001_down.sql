-- Rollback migration for postgres
DROP TRIGGER IF EXISTS "trg_audit_logs_updated_at" ON "audit_logs";
DROP TABLE IF EXISTS "audit_logs";
DROP TRIGGER IF EXISTS "trg_sessions_updated_at" ON "sessions";
DROP TABLE IF EXISTS "sessions";
DROP TRIGGER IF EXISTS "trg_user_roles_updated_at" ON "user_roles";
DROP TABLE IF EXISTS "user_roles";
DROP TRIGGER IF EXISTS "trg_user_groups_updated_at" ON "user_groups";
DROP TABLE IF EXISTS "user_groups";
DROP TRIGGER IF EXISTS "trg_groups_updated_at" ON "groups";
DROP TABLE IF EXISTS "groups";
DROP TRIGGER IF EXISTS "trg_roles_updated_at" ON "roles";
DROP TABLE IF EXISTS "roles";
DROP TRIGGER IF EXISTS "trg_users_updated_at" ON "users";
DROP TABLE IF EXISTS "users";
