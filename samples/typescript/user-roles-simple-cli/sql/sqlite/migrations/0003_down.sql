-- Rollback migration for sqlite
ALTER TABLE roles ADD COLUMN "description" TEXT;
CREATE TABLE "audit_logs" ("id" INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL, "message" TEXT NOT NULL, "created" TEXT NOT NULL, "updated" BLOB CONSTRAINT "audit_logs_updated_default_constraint" DEFAULT X'00' NOT NULL);
CREATE TRIGGER "trg_audit_logs_updated_at"
AFTER UPDATE ON "audit_logs"
FOR EACH ROW
WHEN NEW."updated" IS OLD."updated"
BEGIN
  UPDATE "audit_logs" SET "updated" = (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')) WHERE "id" = OLD."id";
END;
