-- Rollback migration for sqlite
ALTER TABLE addresses ADD COLUMN "line3" VARCHAR(256);
GO
CREATE TABLE "note_pads" ("id" INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL, "uuid" TEXT CONSTRAINT "note_pads_uuid_default_constraint" DEFAULT (lower(hex(randomblob(4))) || '-' || lower(hex(randomblob(2))) || '-4' || substr(lower(hex(randomblob(2))), 2) || '-' || substr('89ab', abs(random()) % 4 + 1, 1) || substr(lower(hex(randomblob(2))), 2) || '-' || lower(hex(randomblob(6)))) NOT NULL, "created" TEXT CONSTRAINT "note_pads_created_default_constraint" DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')) NOT NULL, "updated" TEXT CONSTRAINT "note_pads_updated_default_constraint" DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')) NOT NULL, "version" BLOB CONSTRAINT "note_pads_version_default_constraint" DEFAULT X'00' NOT NULL, "contact_id" INTEGER NOT NULL, "body" TEXT, CONSTRAINT "note_pads_uuid_unique_constraint" UNIQUE ("uuid"), CONSTRAINT "note_pads_contact_id_foreign_key" FOREIGN KEY ("contact_id") REFERENCES "contacts" ("id"));
GO
CREATE TRIGGER "trg_note_pads_updated_at"
AFTER UPDATE ON "note_pads"
FOR EACH ROW
WHEN NEW."updated" IS OLD."updated"
BEGIN
  UPDATE "note_pads" SET "updated" = (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')) WHERE "id" = OLD."id";
END;
