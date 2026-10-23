INSERT INTO "contact_source" ("id", "uuid", "created", "updated", "version", "name", "description") VALUES
(1, '11111111-1111-1111-1111-111111111111', '2024-01-01 00:00:00', '2024-01-01 00:00:00', X'', 'Web', 'Contacts entered from the web app'),
(2, '22222222-2222-2222-2222-222222222222', '2024-01-01 00:00:00', '2024-01-01 00:00:00', X'', 'Import', 'Bulk imported contacts'),
(3, '33333333-3333-3333-3333-333333333333', '2024-01-01 00:00:00', '2024-01-01 00:00:00', X'', 'Legacy', 'Migrated from legacy_contact');

INSERT INTO "contact_type" ("id", "uuid", "created", "updated", "version", "name", "description") VALUES
(1, '44444444-4444-4444-4444-444444444444', '2024-01-01 00:00:00', '2024-01-01 00:00:00', X'', 'Person', 'Individual contact'),
(2, '55555555-5555-5555-5555-555555555555', '2024-01-01 00:00:00', '2024-01-01 00:00:00', X'', 'Organization', 'Organization contact');

INSERT INTO "phone_type" ("id", "uuid", "created", "updated", "version", "name", "description") VALUES
(1, '66666666-6666-6666-6666-666666666666', '2024-01-01 00:00:00', '2024-01-01 00:00:00', X'', 'Mobile', 'Mobile phone'),
(2, '77777777-7777-7777-7777-777777777777', '2024-01-01 00:00:00', '2024-01-01 00:00:00', X'', 'Work', 'Work phone'),
(3, '88888888-8888-8888-8888-888888888888', '2024-01-01 00:00:00', '2024-01-01 00:00:00', X'', 'Home', 'Home phone');

INSERT INTO "address_type" ("id", "uuid", "created", "updated", "version", "name", "description") VALUES
(1, 'aaaaaaaa-aaaa-aaaa-aaaa-aaaaaaaaaaaa', '2024-01-01 00:00:00', '2024-01-01 00:00:00', X'', 'Home', 'Home address'),
(2, 'bbbbbbbb-bbbb-bbbb-bbbb-bbbbbbbbbbbb', '2024-01-01 00:00:00', '2024-01-01 00:00:00', X'', 'Work', 'Work address'),
(3, 'cccccccc-cccc-cccc-cccc-cccccccccccc', '2024-01-01 00:00:00', '2024-01-01 00:00:00', X'', 'Other', 'Other address');
