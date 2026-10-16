-- Rollback migration for sqlite
ALTER TABLE addresses ADD COLUMN "line3" VARCHAR(256);
CREATE TABLE "note_pads" ("id" INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL, "uuid" TEXT NOT NULL, "created" TEXT CONSTRAINT "note_pads_created_default_constraint" DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')) NOT NULL, "updated" TEXT CONSTRAINT "note_pads_updated_default_constraint" DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')) NOT NULL, "version" BLOB NOT NULL, "contact_id" TEXT NOT NULL, "body" TEXT, CONSTRAINT "note_pads_uuid_unique_constraint" UNIQUE ("uuid"), CONSTRAINT "note_pads_contact_id_foreign_key" FOREIGN KEY ("contact_id") REFERENCES "contacts" ("id"));
CREATE TRIGGER "trg_note_pads_updated_at"
AFTER UPDATE ON "note_pads"
FOR EACH ROW
WHEN NEW."updated" IS OLD."updated"
BEGIN
  UPDATE "note_pads" SET "updated" = (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')) WHERE "id" = OLD."id";
END;
