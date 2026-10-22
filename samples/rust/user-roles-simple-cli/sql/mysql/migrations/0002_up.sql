-- Generated migration for mysql
DROP TRIGGER IF EXISTS `trg_sessions_updated_at`;
DROP TABLE IF EXISTS `sessions`;
ALTER TABLE users DROP COLUMN `nickname`;
