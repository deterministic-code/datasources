CREATE TABLE "contact_source" (
  "id" INTEGER PRIMARY KEY,
  "uuid" TEXT NOT NULL,
  "created" TEXT NOT NULL,
  "updated" TEXT NOT NULL,
  "version" BLOB NOT NULL,
  "name" VARCHAR(64) NOT NULL,
  "description" TEXT
);

CREATE TABLE "contact_type" (
  "id" INTEGER PRIMARY KEY,
  "uuid" TEXT NOT NULL,
  "created" TEXT NOT NULL,
  "updated" TEXT NOT NULL,
  "version" BLOB NOT NULL,
  "name" VARCHAR(64) NOT NULL,
  "description" TEXT
);

CREATE TABLE "contacts_base" (
  "id" INTEGER PRIMARY KEY,
  "uuid" TEXT NOT NULL,
  "created" TEXT NOT NULL,
  "updated" TEXT NOT NULL,
  "version" BLOB NOT NULL,
  "contact_source_id" INTEGER NOT NULL REFERENCES "contact_source" ("id"),
  "contact_type_id" INTEGER NOT NULL REFERENCES "contact_type" ("id"),
  "first_name" VARCHAR(128) NOT NULL,
  "last_name" VARCHAR(128) NOT NULL,
  "email" VARCHAR(256),
  "notes" TEXT
);

CREATE TABLE "address_type" (
  "id" INTEGER PRIMARY KEY,
  "uuid" TEXT NOT NULL,
  "created" TEXT NOT NULL,
  "updated" TEXT NOT NULL,
  "version" BLOB NOT NULL,
  "name" VARCHAR(64) NOT NULL,
  "description" TEXT
);

CREATE TABLE "phone_type" (
  "id" INTEGER PRIMARY KEY,
  "uuid" TEXT NOT NULL,
  "created" TEXT NOT NULL,
  "updated" TEXT NOT NULL,
  "version" BLOB NOT NULL,
  "name" VARCHAR(64) NOT NULL,
  "description" TEXT
);

CREATE TABLE "address" (
  "id" INTEGER PRIMARY KEY,
  "uuid" TEXT NOT NULL,
  "created" TEXT NOT NULL,
  "updated" TEXT NOT NULL,
  "version" BLOB NOT NULL,
  "contact_id" INTEGER NOT NULL REFERENCES "contacts_base" ("id"),
  "address_type_id" INTEGER NOT NULL REFERENCES "address_type" ("id"),
  "line1" VARCHAR(256) NOT NULL,
  "line2" VARCHAR(256),
  "city" VARCHAR(128) NOT NULL,
  "region" VARCHAR(128),
  "postal_code" VARCHAR(32),
  "country" VARCHAR(64)
);

CREATE TABLE "phone" (
  "id" INTEGER PRIMARY KEY,
  "uuid" TEXT NOT NULL,
  "created" TEXT NOT NULL,
  "updated" TEXT NOT NULL,
  "version" BLOB NOT NULL,
  "contact_id" INTEGER NOT NULL REFERENCES "contacts_base" ("id"),
  "phone_type_id" INTEGER NOT NULL REFERENCES "phone_type" ("id"),
  "number" VARCHAR(32) NOT NULL
);

CREATE TABLE "contact_groups_base" (
  "id" INTEGER PRIMARY KEY,
  "uuid" TEXT NOT NULL,
  "created" TEXT NOT NULL,
  "updated" TEXT NOT NULL,
  "version" BLOB NOT NULL,
  "name" VARCHAR(128) NOT NULL,
  "description" TEXT
);

CREATE TABLE "contact_group_member" (
  "id" INTEGER PRIMARY KEY,
  "uuid" TEXT NOT NULL,
  "created" TEXT NOT NULL,
  "updated" TEXT NOT NULL,
  "version" BLOB NOT NULL,
  "contact_id" INTEGER NOT NULL REFERENCES "contacts_base" ("id"),
  "contact_group_id" INTEGER NOT NULL REFERENCES "contact_groups_base" ("id"),
  UNIQUE ("contact_id", "contact_group_id")
);

CREATE TABLE "legacy_contact" (
  "key" VARCHAR(64) PRIMARY KEY,
  "first_name" VARCHAR(128) NOT NULL,
  "last_name" VARCHAR(128) NOT NULL,
  "email" VARCHAR(256),
  "imported_at" TEXT
);

CREATE TABLE "contact_change_log" (
  "id" INTEGER PRIMARY KEY,
  "uuid" TEXT NOT NULL,
  "created" TEXT NOT NULL,
  "updated" TEXT NOT NULL,
  "version" BLOB NOT NULL,
  "occurred_at" TEXT NOT NULL,
  "contact_id" INTEGER NOT NULL,
  "changed_by" VARCHAR(128) NOT NULL,
  "change_summary" TEXT NOT NULL
);

INSERT INTO "contact_source" ("id", "uuid", "created", "updated", "version", "name", "description") VALUES
(1, '11111111-1111-1111-1111-111111111111', '2024-01-01T00:00:00Z', '2024-01-01T00:00:00Z', X'', 'Web', 'Contacts entered from the web app'),
(2, '22222222-2222-2222-2222-222222222222', '2024-01-01T00:00:00Z', '2024-01-01T00:00:00Z', X'', 'Import', 'Bulk imported contacts'),
(3, '33333333-3333-3333-3333-333333333333', '2024-01-01T00:00:00Z', '2024-01-01T00:00:00Z', X'', 'Legacy', 'Migrated from legacy_contact');

INSERT INTO "contact_type" ("id", "uuid", "created", "updated", "version", "name", "description") VALUES
(1, '44444444-4444-4444-4444-444444444444', '2024-01-01T00:00:00Z', '2024-01-01T00:00:00Z', X'', 'Person', 'Individual contact'),
(2, '55555555-5555-5555-5555-555555555555', '2024-01-01T00:00:00Z', '2024-01-01T00:00:00Z', X'', 'Organization', 'Organization contact');

INSERT INTO "phone_type" ("id", "uuid", "created", "updated", "version", "name", "description") VALUES
(1, '66666666-6666-6666-6666-666666666666', '2024-01-01T00:00:00Z', '2024-01-01T00:00:00Z', X'', 'Mobile', 'Mobile phone'),
(2, '77777777-7777-7777-7777-777777777777', '2024-01-01T00:00:00Z', '2024-01-01T00:00:00Z', X'', 'Work', 'Work phone'),
(3, '88888888-8888-8888-8888-888888888888', '2024-01-01T00:00:00Z', '2024-01-01T00:00:00Z', X'', 'Home', 'Home phone');

INSERT INTO "address_type" ("id", "uuid", "created", "updated", "version", "name", "description") VALUES
(1, 'aaaaaaaa-aaaa-aaaa-aaaa-aaaaaaaaaaaa', '2024-01-01T00:00:00Z', '2024-01-01T00:00:00Z', X'', 'Home', 'Home address'),
(2, 'bbbbbbbb-bbbb-bbbb-bbbb-bbbbbbbbbbbb', '2024-01-01T00:00:00Z', '2024-01-01T00:00:00Z', X'', 'Work', 'Work address'),
(3, 'cccccccc-cccc-cccc-cccc-cccccccccccc', '2024-01-01T00:00:00Z', '2024-01-01T00:00:00Z', X'', 'Other', 'Other address');
