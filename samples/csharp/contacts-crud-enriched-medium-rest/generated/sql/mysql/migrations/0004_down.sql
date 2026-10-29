-- Rollback migration for mysql
-- Seed data
-- Seeds: address_types
GO
DELETE FROM `address_types` WHERE `id` = 2;
