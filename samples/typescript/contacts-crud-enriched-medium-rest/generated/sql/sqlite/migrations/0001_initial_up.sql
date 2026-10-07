-- Generated initial migration for sqlite

-- Create added tables
CREATE TABLE "bases" (
  "id" INTEGER CONSTRAINT "bases_primary_key" PRIMARY KEY AUTOINCREMENT,
  "uuid" VARCHAR(36) NOT NULL,
  "created" TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')),
  "updated" TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')),
  "version" BLOB NOT NULL,
  CONSTRAINT "bases_uuid_unique_constraint" UNIQUE ("uuid")
);
CREATE TRIGGER "trg_bases_updated_at"
AFTER UPDATE ON "bases"
FOR EACH ROW
WHEN NEW."updated" IS OLD."updated"
BEGIN
  UPDATE "bases" SET "updated" = (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')) WHERE "id" = OLD."id";
END;

CREATE TABLE "contact_sources" (
  "id" INTEGER CONSTRAINT "contact_sources_primary_key" PRIMARY KEY AUTOINCREMENT,
  "uuid" VARCHAR(36) NOT NULL,
  "created" TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')),
  "updated" TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')),
  "version" BLOB NOT NULL,
  "name" VARCHAR(64) NOT NULL,
  "description" TEXT,
  CONSTRAINT "contact_sources_uuid_unique_constraint" UNIQUE ("uuid"),
  CONSTRAINT "contact_sources_name_unique_constraint" UNIQUE ("name")
);
CREATE TRIGGER "trg_contact_sources_updated_at"
AFTER UPDATE ON "contact_sources"
FOR EACH ROW
WHEN NEW."updated" IS OLD."updated"
BEGIN
  UPDATE "contact_sources" SET "updated" = (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')) WHERE "id" = OLD."id";
END;

CREATE TABLE "contacts" (
  "id" INTEGER CONSTRAINT "contacts_primary_key" PRIMARY KEY AUTOINCREMENT,
  "uuid" VARCHAR(36) NOT NULL,
  "created" TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')),
  "updated" TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')),
  "version" BLOB NOT NULL,
  "contact_source_id" TEXT NOT NULL,
  "first_name" VARCHAR(128) NOT NULL,
  "last_name" VARCHAR(128) NOT NULL,
  "email" VARCHAR(256),
  "notes" TEXT,
  CONSTRAINT "contacts_uuid_unique_constraint" UNIQUE ("uuid"),
  CONSTRAINT "contacts_contact_source_id_foreign_key" FOREIGN KEY ("contact_source_id") REFERENCES "contact_sources"("id")
);
CREATE TRIGGER "trg_contacts_updated_at"
AFTER UPDATE ON "contacts"
FOR EACH ROW
WHEN NEW."updated" IS OLD."updated"
BEGIN
  UPDATE "contacts" SET "updated" = (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')) WHERE "id" = OLD."id";
END;

CREATE TABLE "addresses" (
  "id" INTEGER CONSTRAINT "addresses_primary_key" PRIMARY KEY AUTOINCREMENT,
  "uuid" VARCHAR(36) NOT NULL,
  "created" TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')),
  "updated" TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')),
  "version" BLOB NOT NULL,
  "contact_id" TEXT NOT NULL,
  "line1" VARCHAR(256) NOT NULL,
  "line2" VARCHAR(256),
  "city" VARCHAR(128) NOT NULL,
  "region" VARCHAR(128),
  "postal_code" VARCHAR(32),
  "country" VARCHAR(64),
  CONSTRAINT "addresses_uuid_unique_constraint" UNIQUE ("uuid"),
  CONSTRAINT "addresses_contact_id_foreign_key" FOREIGN KEY ("contact_id") REFERENCES "contacts"("id")
);
CREATE TRIGGER "trg_addresses_updated_at"
AFTER UPDATE ON "addresses"
FOR EACH ROW
WHEN NEW."updated" IS OLD."updated"
BEGIN
  UPDATE "addresses" SET "updated" = (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')) WHERE "id" = OLD."id";
END;

CREATE TABLE "phones" (
  "id" INTEGER CONSTRAINT "phones_primary_key" PRIMARY KEY AUTOINCREMENT,
  "uuid" VARCHAR(36) NOT NULL,
  "created" TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')),
  "updated" TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')),
  "version" BLOB NOT NULL,
  "contact_id" TEXT NOT NULL,
  "number" VARCHAR(32) NOT NULL,
  "label" VARCHAR(32),
  CONSTRAINT "phones_uuid_unique_constraint" UNIQUE ("uuid"),
  CONSTRAINT "phones_contact_id_foreign_key" FOREIGN KEY ("contact_id") REFERENCES "contacts"("id")
);
CREATE TRIGGER "trg_phones_updated_at"
AFTER UPDATE ON "phones"
FOR EACH ROW
WHEN NEW."updated" IS OLD."updated"
BEGIN
  UPDATE "phones" SET "updated" = (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')) WHERE "id" = OLD."id";
END;

CREATE TABLE "contact_groups" (
  "id" INTEGER CONSTRAINT "contact_groups_primary_key" PRIMARY KEY AUTOINCREMENT,
  "uuid" VARCHAR(36) NOT NULL,
  "created" TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')),
  "updated" TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')),
  "version" BLOB NOT NULL,
  "name" VARCHAR(128) NOT NULL,
  "description" TEXT,
  CONSTRAINT "contact_groups_uuid_unique_constraint" UNIQUE ("uuid"),
  CONSTRAINT "contact_groups_name_unique_constraint" UNIQUE ("name")
);
CREATE TRIGGER "trg_contact_groups_updated_at"
AFTER UPDATE ON "contact_groups"
FOR EACH ROW
WHEN NEW."updated" IS OLD."updated"
BEGIN
  UPDATE "contact_groups" SET "updated" = (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')) WHERE "id" = OLD."id";
END;

CREATE TABLE "contact_group_members" (
  "id" INTEGER CONSTRAINT "contact_group_members_primary_key" PRIMARY KEY AUTOINCREMENT,
  "uuid" VARCHAR(36) NOT NULL,
  "created" TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')),
  "updated" TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')),
  "version" BLOB NOT NULL,
  "contact_id" TEXT NOT NULL,
  "contact_group_id" TEXT NOT NULL,
  CONSTRAINT "contact_group_members_uuid_unique_constraint" UNIQUE ("uuid"),
  CONSTRAINT "contact_group_members_contact_id_foreign_key" FOREIGN KEY ("contact_id") REFERENCES "contacts"("id"),
  CONSTRAINT "contact_group_members_contact_group_id_foreign_key" FOREIGN KEY ("contact_group_id") REFERENCES "contact_groups"("id")
);
CREATE TRIGGER "trg_contact_group_members_updated_at"
AFTER UPDATE ON "contact_group_members"
FOR EACH ROW
WHEN NEW."updated" IS OLD."updated"
BEGIN
  UPDATE "contact_group_members" SET "updated" = (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')) WHERE "id" = OLD."id";
END;

CREATE TABLE "OldContactsTbl" (
  "CntID" VARCHAR(64) NOT NULL CONSTRAINT "OldContactsTbl_primary_key" PRIMARY KEY,
  "FirstNm" VARCHAR(128) NOT NULL,
  "LastNm" VARCHAR(128) NOT NULL,
  "EmailAddr" VARCHAR(256),
  "ImpDate" TEXT,
  CONSTRAINT "OldContactsTbl_CntID_unique_constraint" UNIQUE ("CntID")
);

CREATE TABLE "contact_change_logs" (
  "id" INTEGER CONSTRAINT "contact_change_logs_primary_key" PRIMARY KEY AUTOINCREMENT,
  "occurred_at" TEXT NOT NULL,
  "contact_id" INTEGER NOT NULL,
  "changed_by" VARCHAR(128) NOT NULL,
  "change_summary" TEXT NOT NULL
);

CREATE TABLE "import_contacts_requests" (
  "source" VARCHAR(64) NOT NULL,
  "payload" TEXT NOT NULL
);

CREATE TABLE "import_contacts_results" (
  "imported_count" INTEGER NOT NULL,
  "skipped_count" INTEGER NOT NULL,
  "failed_count" INTEGER NOT NULL
);

CREATE TABLE "legacy_migration_results" (
  "migrated_count" INTEGER NOT NULL,
  "skipped_count" INTEGER NOT NULL,
  "completed_at" TEXT
);

-- Seed data
-- Seeds: contact_sources
INSERT INTO "contact_sources" ("id", "uuid", "name") VALUES (1, '1d3e841c-6a99-5e61-8283-879f22e8d932', 'Manual');
INSERT INTO "contact_sources" ("id", "uuid", "name", "description") VALUES (2, '38dd404e-9605-5681-a425-678204a8ae48', 'Imported', 'Loaded from a CSV or vCard import.');
INSERT INTO "contact_sources" ("id", "uuid", "name", "description") VALUES (3, 'ef86312e-ba98-502a-90c7-96af9357a65b', 'OAuth', 'Synced from a third-party provider.');

-- Seeds: contact_groups
INSERT INTO "contact_groups" ("id", "uuid", "name", "description") VALUES (1, 'd02c4d97-b6bf-5ac2-b735-9721f41896f3', 'Friends', 'Friends');
INSERT INTO "contact_groups" ("id", "uuid", "name", "description") VALUES (2, '102bdd01-31a2-5663-b368-561bb4101044', 'Work', 'Work contacts');

