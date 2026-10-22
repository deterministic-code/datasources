-- Rollback migration for sqlite
ALTER TABLE users ADD COLUMN "nickname" TEXT;
CREATE TABLE "sessions" ("id" INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL, "user_id" TEXT NOT NULL, "token" TEXT NOT NULL, "created" TEXT NOT NULL, "updated" BLOB CONSTRAINT "sessions_updated_default_constraint" DEFAULT X'00' NOT NULL, CONSTRAINT "sessions_user_id_foreign_key" FOREIGN KEY ("user_id") REFERENCES "users" ("id"));
CREATE TRIGGER "trg_sessions_updated_at"
AFTER UPDATE ON "sessions"
FOR EACH ROW
WHEN NEW."updated" IS OLD."updated"
BEGIN
  UPDATE "sessions" SET "updated" = (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')) WHERE "id" = OLD."id";
END;
