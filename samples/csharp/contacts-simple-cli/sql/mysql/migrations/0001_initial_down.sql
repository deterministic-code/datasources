-- Rollback initial migration for mysql

-- DROP tables (reverse dependency order)
DROP TABLE IF EXISTS `phone_labels`;
GO
DROP TABLE IF EXISTS `email_labels`;
GO
DROP TABLE IF EXISTS `address_labels`;
GO
DROP TABLE IF EXISTS `contact_labels`;
GO
DROP TABLE IF EXISTS `labels`;
GO
DROP TABLE IF EXISTS `phone_attributes`;
GO
DROP TABLE IF EXISTS `email_attributes`;
GO
DROP TABLE IF EXISTS `address_attributes`;
GO
DROP TABLE IF EXISTS `contact_attributes`;
GO
DROP TABLE IF EXISTS `addresses`;
GO
DROP TABLE IF EXISTS `address_types`;
GO
DROP TABLE IF EXISTS `phones`;
GO
DROP TABLE IF EXISTS `phone_types`;
GO
DROP TABLE IF EXISTS `emails`;
GO
DROP TABLE IF EXISTS `email_types`;
GO
DROP TABLE IF EXISTS `contacts`;
GO
DROP TABLE IF EXISTS `genders`;
