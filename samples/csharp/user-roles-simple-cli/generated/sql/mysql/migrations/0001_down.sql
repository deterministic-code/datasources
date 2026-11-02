-- Rollback migration for mysql
DROP TRIGGER IF EXISTS trg_user_roles_updated_at;
GO
DROP TABLE IF EXISTS `user_roles`;
GO
DROP TRIGGER IF EXISTS trg_user_groups_updated_at;
GO
DROP TABLE IF EXISTS `user_groups`;
GO
DROP TRIGGER IF EXISTS trg_groups_updated_at;
GO
DROP TABLE IF EXISTS `groups`;
GO
DROP TRIGGER IF EXISTS trg_roles_updated_at;
GO
DROP TABLE IF EXISTS `roles`;
GO
DROP TRIGGER IF EXISTS trg_users_updated_at;
GO
DROP TABLE IF EXISTS `users`;
