-- Generated migration for postgres
DROP TRIGGER IF EXISTS "trg_note_pads_updated_at" ON "note_pads";
DROP TABLE IF EXISTS "note_pads";
ALTER TABLE addresses DROP COLUMN "line3";
