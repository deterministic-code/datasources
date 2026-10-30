-- Generated migration for mysql
DROP TRIGGER IF EXISTS `trg_tags_updated_at`;
GO
DROP TABLE IF EXISTS `tags`;
GO
ALTER TABLE contacts DROP COLUMN `middle_name`;
