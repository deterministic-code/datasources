-- Rollback migration for sqlite
ALTER TABLE contacts ADD COLUMN "middle_name" VARCHAR(128);
CREATE TABLE "tags" ("id" INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL, "uuid" TEXT NOT NULL, "created" TEXT CONSTRAINT "tags_created_default_constraint" DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')) NOT NULL, "updated" TEXT CONSTRAINT "tags_updated_default_constraint" DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')) NOT NULL, "version" BLOB NOT NULL, "contact_id" TEXT NOT NULL, "name" VARCHAR(64) NOT NULL, CONSTRAINT "tags_uuid_unique_constraint" UNIQUE ("uuid"), CONSTRAINT "tags_contact_id_foreign_key" FOREIGN KEY ("contact_id") REFERENCES "contacts" ("id"));
CREATE TRIGGER "trg_tags_updated_at"
AFTER UPDATE ON "tags"
FOR EACH ROW
WHEN NEW."updated" IS OLD."updated"
BEGIN
  UPDATE "tags" SET "updated" = (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')) WHERE "id" = OLD."id";
END;
