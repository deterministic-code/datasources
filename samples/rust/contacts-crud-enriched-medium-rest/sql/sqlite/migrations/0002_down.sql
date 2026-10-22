-- Rollback migration for sqlite
ALTER TABLE contacts ADD COLUMN "middle_name" VARCHAR(128);
CREATE TABLE "tags" ("id" INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL, "uuid" TEXT CONSTRAINT "tags_uuid_default_constraint" DEFAULT (lower(hex(randomblob(4))) || '-' || lower(hex(randomblob(2))) || '-4' || substr(lower(hex(randomblob(2))), 2) || '-' || substr('89ab', abs(random()) % 4 + 1, 1) || substr(lower(hex(randomblob(2))), 2) || '-' || lower(hex(randomblob(6)))) NOT NULL, "created" TEXT CONSTRAINT "tags_created_default_constraint" DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')) NOT NULL, "updated" TEXT CONSTRAINT "tags_updated_default_constraint" DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')) NOT NULL, "version" BLOB CONSTRAINT "tags_version_default_constraint" DEFAULT X'00' NOT NULL, "contact_id" TEXT NOT NULL, "name" VARCHAR(64) NOT NULL, CONSTRAINT "tags_uuid_unique_constraint" UNIQUE ("uuid"), CONSTRAINT "tags_contact_id_foreign_key" FOREIGN KEY ("contact_id") REFERENCES "contacts" ("id"));
CREATE TRIGGER "trg_tags_updated_at"
AFTER UPDATE ON "tags"
FOR EACH ROW
WHEN NEW."updated" IS OLD."updated"
BEGIN
  UPDATE "tags" SET "updated" = (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')) WHERE "id" = OLD."id";
END;
