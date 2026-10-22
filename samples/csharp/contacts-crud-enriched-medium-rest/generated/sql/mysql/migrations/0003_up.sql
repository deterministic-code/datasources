-- Generated migration for mysql
DROP TRIGGER IF EXISTS `trg_note_pads_updated_at`;
DROP TABLE IF EXISTS `note_pads`;
ALTER TABLE addresses DROP COLUMN `line3`;
