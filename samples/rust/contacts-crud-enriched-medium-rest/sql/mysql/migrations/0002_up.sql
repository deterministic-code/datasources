-- Generated migration for mysql
DROP TRIGGER IF EXISTS `trg_tags_updated_at`;
DROP TABLE IF EXISTS `tags`;
ALTER TABLE contacts DROP COLUMN `middle_name`;
