-- Generated migration for sqlite
CREATE TABLE "contact_sources" ("id" INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL, "uuid" TEXT CONSTRAINT "contact_sources_uuid_default_constraint" DEFAULT (lower(hex(randomblob(4))) || '-' || lower(hex(randomblob(2))) || '-4' || substr(lower(hex(randomblob(2))), 2) || '-' || substr('89ab', abs(random()) % 4 + 1, 1) || substr(lower(hex(randomblob(2))), 2) || '-' || lower(hex(randomblob(6)))) NOT NULL, "created" TEXT CONSTRAINT "contact_sources_created_default_constraint" DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')) NOT NULL, "updated" TEXT CONSTRAINT "contact_sources_updated_default_constraint" DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')) NOT NULL, "version" BLOB CONSTRAINT "contact_sources_version_default_constraint" DEFAULT X'00' NOT NULL, "name" VARCHAR(64) NOT NULL, "description" TEXT, CONSTRAINT "contact_sources_uuid_unique_constraint" UNIQUE ("uuid"), CONSTRAINT "contact_sources_name_unique_constraint" UNIQUE ("name"));
CREATE TRIGGER "trg_contact_sources_updated_at"
AFTER UPDATE ON "contact_sources"
FOR EACH ROW
WHEN NEW."updated" IS OLD."updated"
BEGIN
  UPDATE "contact_sources" SET "updated" = (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')) WHERE "id" = OLD."id";
END;
CREATE TABLE "address_types" ("id" INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL, "uuid" TEXT CONSTRAINT "address_types_uuid_default_constraint" DEFAULT (lower(hex(randomblob(4))) || '-' || lower(hex(randomblob(2))) || '-4' || substr(lower(hex(randomblob(2))), 2) || '-' || substr('89ab', abs(random()) % 4 + 1, 1) || substr(lower(hex(randomblob(2))), 2) || '-' || lower(hex(randomblob(6)))) NOT NULL, "created" TEXT CONSTRAINT "address_types_created_default_constraint" DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')) NOT NULL, "updated" TEXT CONSTRAINT "address_types_updated_default_constraint" DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')) NOT NULL, "version" BLOB CONSTRAINT "address_types_version_default_constraint" DEFAULT X'00' NOT NULL, "name" VARCHAR(64) NOT NULL, "description" TEXT, CONSTRAINT "address_types_uuid_unique_constraint" UNIQUE ("uuid"));
CREATE TRIGGER "trg_address_types_updated_at"
AFTER UPDATE ON "address_types"
FOR EACH ROW
WHEN NEW."updated" IS OLD."updated"
BEGIN
  UPDATE "address_types" SET "updated" = (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')) WHERE "id" = OLD."id";
END;
CREATE TABLE "phone_types" ("id" INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL, "uuid" TEXT CONSTRAINT "phone_types_uuid_default_constraint" DEFAULT (lower(hex(randomblob(4))) || '-' || lower(hex(randomblob(2))) || '-4' || substr(lower(hex(randomblob(2))), 2) || '-' || substr('89ab', abs(random()) % 4 + 1, 1) || substr(lower(hex(randomblob(2))), 2) || '-' || lower(hex(randomblob(6)))) NOT NULL, "created" TEXT CONSTRAINT "phone_types_created_default_constraint" DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')) NOT NULL, "updated" TEXT CONSTRAINT "phone_types_updated_default_constraint" DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')) NOT NULL, "version" BLOB CONSTRAINT "phone_types_version_default_constraint" DEFAULT X'00' NOT NULL, "name" VARCHAR(64) NOT NULL, "description" TEXT, CONSTRAINT "phone_types_uuid_unique_constraint" UNIQUE ("uuid"));
CREATE TRIGGER "trg_phone_types_updated_at"
AFTER UPDATE ON "phone_types"
FOR EACH ROW
WHEN NEW."updated" IS OLD."updated"
BEGIN
  UPDATE "phone_types" SET "updated" = (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')) WHERE "id" = OLD."id";
END;
CREATE TABLE "contacts" ("id" INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL, "uuid" TEXT CONSTRAINT "contacts_uuid_default_constraint" DEFAULT (lower(hex(randomblob(4))) || '-' || lower(hex(randomblob(2))) || '-4' || substr(lower(hex(randomblob(2))), 2) || '-' || substr('89ab', abs(random()) % 4 + 1, 1) || substr(lower(hex(randomblob(2))), 2) || '-' || lower(hex(randomblob(6)))) NOT NULL, "created" TEXT CONSTRAINT "contacts_created_default_constraint" DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')) NOT NULL, "updated" TEXT CONSTRAINT "contacts_updated_default_constraint" DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')) NOT NULL, "version" BLOB CONSTRAINT "contacts_version_default_constraint" DEFAULT X'00' NOT NULL, "contact_source_id" TEXT NOT NULL, "first_name" VARCHAR(128) NOT NULL, "last_name" VARCHAR(128) NOT NULL, "middle_name" VARCHAR(128), "email" VARCHAR(256), "notes" TEXT, CONSTRAINT "contacts_uuid_unique_constraint" UNIQUE ("uuid"), CONSTRAINT "contacts_contact_source_id_foreign_key" FOREIGN KEY ("contact_source_id") REFERENCES "contact_sources" ("id"));
CREATE TRIGGER "trg_contacts_updated_at"
AFTER UPDATE ON "contacts"
FOR EACH ROW
WHEN NEW."updated" IS OLD."updated"
BEGIN
  UPDATE "contacts" SET "updated" = (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')) WHERE "id" = OLD."id";
END;
CREATE TABLE "addresses" ("id" INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL, "uuid" TEXT CONSTRAINT "addresses_uuid_default_constraint" DEFAULT (lower(hex(randomblob(4))) || '-' || lower(hex(randomblob(2))) || '-4' || substr(lower(hex(randomblob(2))), 2) || '-' || substr('89ab', abs(random()) % 4 + 1, 1) || substr(lower(hex(randomblob(2))), 2) || '-' || lower(hex(randomblob(6)))) NOT NULL, "created" TEXT CONSTRAINT "addresses_created_default_constraint" DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')) NOT NULL, "updated" TEXT CONSTRAINT "addresses_updated_default_constraint" DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')) NOT NULL, "version" BLOB CONSTRAINT "addresses_version_default_constraint" DEFAULT X'00' NOT NULL, "contact_id" TEXT NOT NULL, "address_type_id" TEXT NOT NULL, "line1" VARCHAR(256) NOT NULL, "line2" VARCHAR(256), "line3" VARCHAR(256), "city" VARCHAR(128) NOT NULL, "region" VARCHAR(128), "postal_code" VARCHAR(32), "country" VARCHAR(64), CONSTRAINT "addresses_uuid_unique_constraint" UNIQUE ("uuid"), CONSTRAINT "addresses_contact_id_foreign_key" FOREIGN KEY ("contact_id") REFERENCES "contacts" ("id"), CONSTRAINT "addresses_address_type_id_foreign_key" FOREIGN KEY ("address_type_id") REFERENCES "address_types" ("id"));
CREATE TRIGGER "trg_addresses_updated_at"
AFTER UPDATE ON "addresses"
FOR EACH ROW
WHEN NEW."updated" IS OLD."updated"
BEGIN
  UPDATE "addresses" SET "updated" = (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')) WHERE "id" = OLD."id";
END;
CREATE TABLE "phones" ("id" INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL, "uuid" TEXT CONSTRAINT "phones_uuid_default_constraint" DEFAULT (lower(hex(randomblob(4))) || '-' || lower(hex(randomblob(2))) || '-4' || substr(lower(hex(randomblob(2))), 2) || '-' || substr('89ab', abs(random()) % 4 + 1, 1) || substr(lower(hex(randomblob(2))), 2) || '-' || lower(hex(randomblob(6)))) NOT NULL, "created" TEXT CONSTRAINT "phones_created_default_constraint" DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')) NOT NULL, "updated" TEXT CONSTRAINT "phones_updated_default_constraint" DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')) NOT NULL, "version" BLOB CONSTRAINT "phones_version_default_constraint" DEFAULT X'00' NOT NULL, "contact_id" TEXT NOT NULL, "phone_type_id" TEXT NOT NULL, "number" VARCHAR(32) NOT NULL, "label" VARCHAR(32), CONSTRAINT "phones_uuid_unique_constraint" UNIQUE ("uuid"), CONSTRAINT "phones_contact_id_foreign_key" FOREIGN KEY ("contact_id") REFERENCES "contacts" ("id"), CONSTRAINT "phones_phone_type_id_foreign_key" FOREIGN KEY ("phone_type_id") REFERENCES "phone_types" ("id"));
CREATE TRIGGER "trg_phones_updated_at"
AFTER UPDATE ON "phones"
FOR EACH ROW
WHEN NEW."updated" IS OLD."updated"
BEGIN
  UPDATE "phones" SET "updated" = (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')) WHERE "id" = OLD."id";
END;
CREATE TABLE "tags" ("id" INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL, "uuid" TEXT CONSTRAINT "tags_uuid_default_constraint" DEFAULT (lower(hex(randomblob(4))) || '-' || lower(hex(randomblob(2))) || '-4' || substr(lower(hex(randomblob(2))), 2) || '-' || substr('89ab', abs(random()) % 4 + 1, 1) || substr(lower(hex(randomblob(2))), 2) || '-' || lower(hex(randomblob(6)))) NOT NULL, "created" TEXT CONSTRAINT "tags_created_default_constraint" DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')) NOT NULL, "updated" TEXT CONSTRAINT "tags_updated_default_constraint" DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')) NOT NULL, "version" BLOB CONSTRAINT "tags_version_default_constraint" DEFAULT X'00' NOT NULL, "contact_id" TEXT NOT NULL, "name" VARCHAR(64) NOT NULL, CONSTRAINT "tags_uuid_unique_constraint" UNIQUE ("uuid"), CONSTRAINT "tags_contact_id_foreign_key" FOREIGN KEY ("contact_id") REFERENCES "contacts" ("id"));
CREATE TRIGGER "trg_tags_updated_at"
AFTER UPDATE ON "tags"
FOR EACH ROW
WHEN NEW."updated" IS OLD."updated"
BEGIN
  UPDATE "tags" SET "updated" = (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')) WHERE "id" = OLD."id";
END;
CREATE TABLE "note_pads" ("id" INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL, "uuid" TEXT CONSTRAINT "note_pads_uuid_default_constraint" DEFAULT (lower(hex(randomblob(4))) || '-' || lower(hex(randomblob(2))) || '-4' || substr(lower(hex(randomblob(2))), 2) || '-' || substr('89ab', abs(random()) % 4 + 1, 1) || substr(lower(hex(randomblob(2))), 2) || '-' || lower(hex(randomblob(6)))) NOT NULL, "created" TEXT CONSTRAINT "note_pads_created_default_constraint" DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')) NOT NULL, "updated" TEXT CONSTRAINT "note_pads_updated_default_constraint" DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')) NOT NULL, "version" BLOB CONSTRAINT "note_pads_version_default_constraint" DEFAULT X'00' NOT NULL, "contact_id" TEXT NOT NULL, "body" TEXT, CONSTRAINT "note_pads_uuid_unique_constraint" UNIQUE ("uuid"), CONSTRAINT "note_pads_contact_id_foreign_key" FOREIGN KEY ("contact_id") REFERENCES "contacts" ("id"));
CREATE TRIGGER "trg_note_pads_updated_at"
AFTER UPDATE ON "note_pads"
FOR EACH ROW
WHEN NEW."updated" IS OLD."updated"
BEGIN
  UPDATE "note_pads" SET "updated" = (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')) WHERE "id" = OLD."id";
END;
CREATE TABLE "contact_groups" ("id" INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL, "uuid" TEXT CONSTRAINT "contact_groups_uuid_default_constraint" DEFAULT (lower(hex(randomblob(4))) || '-' || lower(hex(randomblob(2))) || '-4' || substr(lower(hex(randomblob(2))), 2) || '-' || substr('89ab', abs(random()) % 4 + 1, 1) || substr(lower(hex(randomblob(2))), 2) || '-' || lower(hex(randomblob(6)))) NOT NULL, "created" TEXT CONSTRAINT "contact_groups_created_default_constraint" DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')) NOT NULL, "updated" TEXT CONSTRAINT "contact_groups_updated_default_constraint" DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')) NOT NULL, "version" BLOB CONSTRAINT "contact_groups_version_default_constraint" DEFAULT X'00' NOT NULL, "name" VARCHAR(128) NOT NULL, "description" TEXT, CONSTRAINT "contact_groups_uuid_unique_constraint" UNIQUE ("uuid"), CONSTRAINT "contact_groups_name_unique_constraint" UNIQUE ("name"));
CREATE TRIGGER "trg_contact_groups_updated_at"
AFTER UPDATE ON "contact_groups"
FOR EACH ROW
WHEN NEW."updated" IS OLD."updated"
BEGIN
  UPDATE "contact_groups" SET "updated" = (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')) WHERE "id" = OLD."id";
END;
CREATE TABLE "contact_settings" ("id" INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL, "uuid" TEXT CONSTRAINT "contact_settings_uuid_default_constraint" DEFAULT (lower(hex(randomblob(4))) || '-' || lower(hex(randomblob(2))) || '-4' || substr(lower(hex(randomblob(2))), 2) || '-' || substr('89ab', abs(random()) % 4 + 1, 1) || substr(lower(hex(randomblob(2))), 2) || '-' || lower(hex(randomblob(6)))) NOT NULL, "created" TEXT CONSTRAINT "contact_settings_created_default_constraint" DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')) NOT NULL, "updated" TEXT CONSTRAINT "contact_settings_updated_default_constraint" DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')) NOT NULL, "version" BLOB CONSTRAINT "contact_settings_version_default_constraint" DEFAULT X'00' NOT NULL, "contact_id" TEXT NOT NULL, "key" VARCHAR(64) NOT NULL, "value" TEXT NOT NULL, CONSTRAINT "contact_settings_uuid_unique_constraint" UNIQUE ("uuid"), CONSTRAINT "contact_settings_contact_id_foreign_key" FOREIGN KEY ("contact_id") REFERENCES "contacts" ("id"));
CREATE UNIQUE INDEX "contact_settings_owner_key_index" ON "contact_settings" ("contact_id", "key");
CREATE TRIGGER "trg_contact_settings_updated_at"
AFTER UPDATE ON "contact_settings"
FOR EACH ROW
WHEN NEW."updated" IS OLD."updated"
BEGIN
  UPDATE "contact_settings" SET "updated" = (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')) WHERE "id" = OLD."id";
END;
CREATE TABLE "contact_preferences" ("id" INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL, "uuid" TEXT CONSTRAINT "contact_preferences_uuid_default_constraint" DEFAULT (lower(hex(randomblob(4))) || '-' || lower(hex(randomblob(2))) || '-4' || substr(lower(hex(randomblob(2))), 2) || '-' || substr('89ab', abs(random()) % 4 + 1, 1) || substr(lower(hex(randomblob(2))), 2) || '-' || lower(hex(randomblob(6)))) NOT NULL, "created" TEXT CONSTRAINT "contact_preferences_created_default_constraint" DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')) NOT NULL, "updated" TEXT CONSTRAINT "contact_preferences_updated_default_constraint" DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')) NOT NULL, "version" BLOB CONSTRAINT "contact_preferences_version_default_constraint" DEFAULT X'00' NOT NULL, "contact_id" TEXT NOT NULL, "key" VARCHAR(64) NOT NULL, "locale" VARCHAR(16) NOT NULL, "timezone" VARCHAR(64) NOT NULL, CONSTRAINT "contact_preferences_uuid_unique_constraint" UNIQUE ("uuid"), CONSTRAINT "contact_preferences_contact_id_foreign_key" FOREIGN KEY ("contact_id") REFERENCES "contacts" ("id"));
CREATE UNIQUE INDEX "contact_preferences_owner_key_index" ON "contact_preferences" ("contact_id", "key");
CREATE TRIGGER "trg_contact_preferences_updated_at"
AFTER UPDATE ON "contact_preferences"
FOR EACH ROW
WHEN NEW."updated" IS OLD."updated"
BEGIN
  UPDATE "contact_preferences" SET "updated" = (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')) WHERE "id" = OLD."id";
END;
CREATE TABLE "contact_group_members" ("id" INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL, "uuid" TEXT CONSTRAINT "contact_group_members_uuid_default_constraint" DEFAULT (lower(hex(randomblob(4))) || '-' || lower(hex(randomblob(2))) || '-4' || substr(lower(hex(randomblob(2))), 2) || '-' || substr('89ab', abs(random()) % 4 + 1, 1) || substr(lower(hex(randomblob(2))), 2) || '-' || lower(hex(randomblob(6)))) NOT NULL, "created" TEXT CONSTRAINT "contact_group_members_created_default_constraint" DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')) NOT NULL, "updated" TEXT CONSTRAINT "contact_group_members_updated_default_constraint" DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')) NOT NULL, "version" BLOB CONSTRAINT "contact_group_members_version_default_constraint" DEFAULT X'00' NOT NULL, "contact_id" TEXT NOT NULL, "contact_group_id" TEXT NOT NULL, CONSTRAINT "contact_group_members_uuid_unique_constraint" UNIQUE ("uuid"), CONSTRAINT "contact_group_members_contact_id_foreign_key" FOREIGN KEY ("contact_id") REFERENCES "contacts" ("id"), CONSTRAINT "contact_group_members_contact_group_id_foreign_key" FOREIGN KEY ("contact_group_id") REFERENCES "contact_groups" ("id"));
CREATE TRIGGER "trg_contact_group_members_updated_at"
AFTER UPDATE ON "contact_group_members"
FOR EACH ROW
WHEN NEW."updated" IS OLD."updated"
BEGIN
  UPDATE "contact_group_members" SET "updated" = (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')) WHERE "id" = OLD."id";
END;
CREATE TABLE "OldContactsTbl" ("CntID" VARCHAR(64) NOT NULL, "FirstNm" VARCHAR(128) NOT NULL, "LastNm" VARCHAR(128) NOT NULL, "EmailAddr" VARCHAR(256), "ImpDate" TEXT, CONSTRAINT "OldContactsTbl_primary_key" PRIMARY KEY ("CntID"), CONSTRAINT "OldContactsTbl_CntID_unique_constraint" UNIQUE ("CntID"));
CREATE TABLE "contact_change_logs" ("id" INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL, "occurred_at" TEXT NOT NULL, "contact_id" INTEGER NOT NULL, "changed_by" VARCHAR(128) NOT NULL, "change_summary" TEXT NOT NULL);
-- Seed data
-- Seeds: contact_sources
INSERT INTO "contact_sources" ("id", "uuid", "name") VALUES (1, '1d3e841c-6a99-5e61-8283-879f22e8d932', 'Manual');
INSERT INTO "contact_sources" ("id", "uuid", "name", "description") VALUES (2, '38dd404e-9605-5681-a425-678204a8ae48', 'Imported', 'Loaded from a CSV or vCard import.');
INSERT INTO "contact_sources" ("id", "uuid", "name", "description") VALUES (3, 'ef86312e-ba98-502a-90c7-96af9357a65b', 'OAuth', 'Synced from a third-party provider.');
-- Seeds: address_types
INSERT INTO "address_types" ("id", "uuid", "name") VALUES (1, '1e75343a-f291-5115-a555-47c1f64a2c69', 'Home');
-- Seeds: phone_types
INSERT INTO "phone_types" ("id", "uuid", "name") VALUES (1, '775f24d4-e8cb-505c-9671-1d53c44f2780', 'Mobile');
-- Seeds: contact_groups
INSERT INTO "contact_groups" ("id", "uuid", "name", "description") VALUES (1, 'd02c4d97-b6bf-5ac2-b735-9721f41896f3', 'Friends', 'Friends');
INSERT INTO "contact_groups" ("id", "uuid", "name", "description") VALUES (2, '102bdd01-31a2-5663-b368-561bb4101044', 'Work', 'Work contacts');
