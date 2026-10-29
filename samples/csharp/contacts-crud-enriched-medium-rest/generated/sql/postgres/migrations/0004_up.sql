-- Generated migration for postgres
-- Seed data
-- Seeds: address_types
GO
INSERT INTO "address_types" ("id", "uuid", "name") VALUES (2, '17222ad6-7a4a-547c-b7c7-083cf9664323', 'Work');
SELECT setval(pg_get_serial_sequence('address_types', 'id'), (SELECT MAX("id") FROM "address_types"));
