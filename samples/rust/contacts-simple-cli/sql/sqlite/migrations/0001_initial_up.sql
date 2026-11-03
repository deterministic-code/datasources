-- Generated initial migration for sqlite

-- Create added tables
CREATE TABLE "genders" ("id" INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL, "uuid" TEXT CONSTRAINT "genders_uuid_default_constraint" DEFAULT (lower(hex(randomblob(4))) || '-' || lower(hex(randomblob(2))) || '-4' || substr(lower(hex(randomblob(2))), 2) || '-' || substr('89ab', abs(random()) % 4 + 1, 1) || substr(lower(hex(randomblob(2))), 2) || '-' || lower(hex(randomblob(6)))) NOT NULL, "created" TEXT CONSTRAINT "genders_created_default_constraint" DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')) NOT NULL, "updated" TEXT CONSTRAINT "genders_updated_default_constraint" DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')) NOT NULL, "version" BLOB CONSTRAINT "genders_version_default_constraint" DEFAULT X'00' NOT NULL, "name" VARCHAR(128) NOT NULL, "description" TEXT CONSTRAINT "genders_description_default_constraint" DEFAULT '' NOT NULL, CONSTRAINT "genders_uuid_unique_constraint" UNIQUE ("uuid"), CONSTRAINT "genders_name_unique_constraint" UNIQUE ("name"));
GO
CREATE TRIGGER "trg_genders_updated_at"
AFTER UPDATE ON "genders"
FOR EACH ROW
WHEN NEW."updated" IS OLD."updated"
BEGIN
  UPDATE "genders" SET "updated" = (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')) WHERE "id" = OLD."id";
END;
GO
CREATE TABLE "contacts" ("id" INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL, "uuid" TEXT CONSTRAINT "contacts_uuid_default_constraint" DEFAULT (lower(hex(randomblob(4))) || '-' || lower(hex(randomblob(2))) || '-4' || substr(lower(hex(randomblob(2))), 2) || '-' || substr('89ab', abs(random()) % 4 + 1, 1) || substr(lower(hex(randomblob(2))), 2) || '-' || lower(hex(randomblob(6)))) NOT NULL, "created" TEXT CONSTRAINT "contacts_created_default_constraint" DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')) NOT NULL, "updated" TEXT CONSTRAINT "contacts_updated_default_constraint" DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')) NOT NULL, "version" BLOB CONSTRAINT "contacts_version_default_constraint" DEFAULT X'00' NOT NULL, "name" VARCHAR(512) NOT NULL, "given_name" VARCHAR(512) NOT NULL, "additional_name" VARCHAR(512) CONSTRAINT "contacts_additional_name_default_constraint" DEFAULT '' NOT NULL, "pronunciation" VARCHAR(512) CONSTRAINT "contacts_pronunciation_default_constraint" DEFAULT '' NOT NULL, "family_name" VARCHAR(512) CONSTRAINT "contacts_family_name_default_constraint" DEFAULT '' NOT NULL, "nick_name" VARCHAR(512) CONSTRAINT "contacts_nick_name_default_constraint" DEFAULT '' NOT NULL, "job_title" VARCHAR(512) CONSTRAINT "contacts_job_title_default_constraint" DEFAULT '' NOT NULL, "gender_type_id" INTEGER NOT NULL, "organization" VARCHAR(512) CONSTRAINT "contacts_organization_default_constraint" DEFAULT '' NOT NULL, "is_starred" BOOLEAN NOT NULL, "is_favorite" BOOLEAN NOT NULL, "is_deleted" BOOLEAN NOT NULL, CONSTRAINT "contacts_uuid_unique_constraint" UNIQUE ("uuid"), CONSTRAINT "contacts_gender_type_id_foreign_key" FOREIGN KEY ("gender_type_id") REFERENCES "genders" ("id"));
GO
CREATE TRIGGER "trg_contacts_updated_at"
AFTER UPDATE ON "contacts"
FOR EACH ROW
WHEN NEW."updated" IS OLD."updated"
BEGIN
  UPDATE "contacts" SET "updated" = (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')) WHERE "id" = OLD."id";
END;
GO
CREATE TABLE "email_types" ("id" INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL, "uuid" TEXT CONSTRAINT "email_types_uuid_default_constraint" DEFAULT (lower(hex(randomblob(4))) || '-' || lower(hex(randomblob(2))) || '-4' || substr(lower(hex(randomblob(2))), 2) || '-' || substr('89ab', abs(random()) % 4 + 1, 1) || substr(lower(hex(randomblob(2))), 2) || '-' || lower(hex(randomblob(6)))) NOT NULL, "created" TEXT CONSTRAINT "email_types_created_default_constraint" DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')) NOT NULL, "updated" TEXT CONSTRAINT "email_types_updated_default_constraint" DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')) NOT NULL, "version" BLOB CONSTRAINT "email_types_version_default_constraint" DEFAULT X'00' NOT NULL, "name" VARCHAR(128) NOT NULL, "description" TEXT CONSTRAINT "email_types_description_default_constraint" DEFAULT '' NOT NULL, CONSTRAINT "email_types_uuid_unique_constraint" UNIQUE ("uuid"), CONSTRAINT "email_types_name_unique_constraint" UNIQUE ("name"));
GO
CREATE TRIGGER "trg_email_types_updated_at"
AFTER UPDATE ON "email_types"
FOR EACH ROW
WHEN NEW."updated" IS OLD."updated"
BEGIN
  UPDATE "email_types" SET "updated" = (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')) WHERE "id" = OLD."id";
END;
GO
CREATE TABLE "emails" ("id" INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL, "uuid" TEXT CONSTRAINT "emails_uuid_default_constraint" DEFAULT (lower(hex(randomblob(4))) || '-' || lower(hex(randomblob(2))) || '-4' || substr(lower(hex(randomblob(2))), 2) || '-' || substr('89ab', abs(random()) % 4 + 1, 1) || substr(lower(hex(randomblob(2))), 2) || '-' || lower(hex(randomblob(6)))) NOT NULL, "created" TEXT CONSTRAINT "emails_created_default_constraint" DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')) NOT NULL, "updated" TEXT CONSTRAINT "emails_updated_default_constraint" DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')) NOT NULL, "version" BLOB CONSTRAINT "emails_version_default_constraint" DEFAULT X'00' NOT NULL, "email_type_id" INTEGER NOT NULL, "contact_id" INTEGER NOT NULL, "address" VARCHAR(320) CONSTRAINT "emails_address_default_constraint" DEFAULT '' NOT NULL, "display_name" VARCHAR(256) CONSTRAINT "emails_display_name_default_constraint" DEFAULT '' NOT NULL, "is_primary" BOOLEAN NOT NULL, CONSTRAINT "emails_uuid_unique_constraint" UNIQUE ("uuid"), CONSTRAINT "emails_email_type_id_foreign_key" FOREIGN KEY ("email_type_id") REFERENCES "email_types" ("id"), CONSTRAINT "emails_contact_id_foreign_key" FOREIGN KEY ("contact_id") REFERENCES "contacts" ("id"));
GO
CREATE UNIQUE INDEX "emails_contact_id_email_type_id_index_index" ON "emails" ("contact_id", "email_type_id");
GO
CREATE TRIGGER "trg_emails_updated_at"
AFTER UPDATE ON "emails"
FOR EACH ROW
WHEN NEW."updated" IS OLD."updated"
BEGIN
  UPDATE "emails" SET "updated" = (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')) WHERE "id" = OLD."id";
END;
GO
CREATE TABLE "phone_types" ("id" INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL, "uuid" TEXT CONSTRAINT "phone_types_uuid_default_constraint" DEFAULT (lower(hex(randomblob(4))) || '-' || lower(hex(randomblob(2))) || '-4' || substr(lower(hex(randomblob(2))), 2) || '-' || substr('89ab', abs(random()) % 4 + 1, 1) || substr(lower(hex(randomblob(2))), 2) || '-' || lower(hex(randomblob(6)))) NOT NULL, "created" TEXT CONSTRAINT "phone_types_created_default_constraint" DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')) NOT NULL, "updated" TEXT CONSTRAINT "phone_types_updated_default_constraint" DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')) NOT NULL, "version" BLOB CONSTRAINT "phone_types_version_default_constraint" DEFAULT X'00' NOT NULL, "name" VARCHAR(128) NOT NULL, "description" TEXT CONSTRAINT "phone_types_description_default_constraint" DEFAULT '' NOT NULL, CONSTRAINT "phone_types_uuid_unique_constraint" UNIQUE ("uuid"), CONSTRAINT "phone_types_name_unique_constraint" UNIQUE ("name"));
GO
CREATE TRIGGER "trg_phone_types_updated_at"
AFTER UPDATE ON "phone_types"
FOR EACH ROW
WHEN NEW."updated" IS OLD."updated"
BEGIN
  UPDATE "phone_types" SET "updated" = (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')) WHERE "id" = OLD."id";
END;
GO
CREATE TABLE "phones" ("id" INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL, "uuid" TEXT CONSTRAINT "phones_uuid_default_constraint" DEFAULT (lower(hex(randomblob(4))) || '-' || lower(hex(randomblob(2))) || '-4' || substr(lower(hex(randomblob(2))), 2) || '-' || substr('89ab', abs(random()) % 4 + 1, 1) || substr(lower(hex(randomblob(2))), 2) || '-' || lower(hex(randomblob(6)))) NOT NULL, "created" TEXT CONSTRAINT "phones_created_default_constraint" DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')) NOT NULL, "updated" TEXT CONSTRAINT "phones_updated_default_constraint" DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')) NOT NULL, "version" BLOB CONSTRAINT "phones_version_default_constraint" DEFAULT X'00' NOT NULL, "phone_type_id" INTEGER NOT NULL, "contact_id" INTEGER NOT NULL, "number" VARCHAR(20) NOT NULL, "display_name" VARCHAR(256) NOT NULL, "extention" VARCHAR(10) NOT NULL, "is_primary" BOOLEAN NOT NULL, "is_verified" BOOLEAN CONSTRAINT "phones_is_verified_default_constraint" DEFAULT 0 NOT NULL, CONSTRAINT "phones_uuid_unique_constraint" UNIQUE ("uuid"), CONSTRAINT "phones_phone_type_id_foreign_key" FOREIGN KEY ("phone_type_id") REFERENCES "phone_types" ("id"), CONSTRAINT "phones_contact_id_foreign_key" FOREIGN KEY ("contact_id") REFERENCES "contacts" ("id"));
GO
CREATE UNIQUE INDEX "phones_contact_id_phone_type_id_index_index" ON "phones" ("contact_id", "phone_type_id");
GO
CREATE TRIGGER "trg_phones_updated_at"
AFTER UPDATE ON "phones"
FOR EACH ROW
WHEN NEW."updated" IS OLD."updated"
BEGIN
  UPDATE "phones" SET "updated" = (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')) WHERE "id" = OLD."id";
END;
GO
CREATE TABLE "address_types" ("id" INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL, "uuid" TEXT CONSTRAINT "address_types_uuid_default_constraint" DEFAULT (lower(hex(randomblob(4))) || '-' || lower(hex(randomblob(2))) || '-4' || substr(lower(hex(randomblob(2))), 2) || '-' || substr('89ab', abs(random()) % 4 + 1, 1) || substr(lower(hex(randomblob(2))), 2) || '-' || lower(hex(randomblob(6)))) NOT NULL, "created" TEXT CONSTRAINT "address_types_created_default_constraint" DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')) NOT NULL, "updated" TEXT CONSTRAINT "address_types_updated_default_constraint" DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')) NOT NULL, "version" BLOB CONSTRAINT "address_types_version_default_constraint" DEFAULT X'00' NOT NULL, "name" VARCHAR(128) NOT NULL, "description" TEXT CONSTRAINT "address_types_description_default_constraint" DEFAULT '' NOT NULL, CONSTRAINT "address_types_uuid_unique_constraint" UNIQUE ("uuid"));
GO
CREATE TRIGGER "trg_address_types_updated_at"
AFTER UPDATE ON "address_types"
FOR EACH ROW
WHEN NEW."updated" IS OLD."updated"
BEGIN
  UPDATE "address_types" SET "updated" = (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')) WHERE "id" = OLD."id";
END;
GO
CREATE TABLE "addresses" ("id" INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL, "uuid" TEXT CONSTRAINT "addresses_uuid_default_constraint" DEFAULT (lower(hex(randomblob(4))) || '-' || lower(hex(randomblob(2))) || '-4' || substr(lower(hex(randomblob(2))), 2) || '-' || substr('89ab', abs(random()) % 4 + 1, 1) || substr(lower(hex(randomblob(2))), 2) || '-' || lower(hex(randomblob(6)))) NOT NULL, "created" TEXT CONSTRAINT "addresses_created_default_constraint" DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')) NOT NULL, "updated" TEXT CONSTRAINT "addresses_updated_default_constraint" DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')) NOT NULL, "version" BLOB CONSTRAINT "addresses_version_default_constraint" DEFAULT X'00' NOT NULL, "contact_id" INTEGER NOT NULL, "address_type_id" INTEGER NOT NULL, "display_name" VARCHAR(255) CONSTRAINT "addresses_display_name_default_constraint" DEFAULT '' NOT NULL, "line_1" VARCHAR(255) NOT NULL, "line_2" VARCHAR(255) CONSTRAINT "addresses_line_2_default_constraint" DEFAULT '' NOT NULL, "line_3" VARCHAR(255) CONSTRAINT "addresses_line_3_default_constraint" DEFAULT '' NOT NULL, "neighborhood" VARCHAR(100) CONSTRAINT "addresses_neighborhood_default_constraint" DEFAULT '' NOT NULL, "district" VARCHAR(100) CONSTRAINT "addresses_district_default_constraint" DEFAULT '' NOT NULL, "city" VARCHAR(100) NOT NULL, "state_province" VARCHAR(100) CONSTRAINT "addresses_state_province_default_constraint" DEFAULT '' NOT NULL, "postal_code" VARCHAR(20) CONSTRAINT "addresses_postal_code_default_constraint" DEFAULT '' NOT NULL, "country_code" VARCHAR(2) NOT NULL, "formatted_address" TEXT CONSTRAINT "addresses_formatted_address_default_constraint" DEFAULT '' NOT NULL, "latitude" INTEGER, "longitude" INTEGER, "is_primary" BOOLEAN CONSTRAINT "addresses_is_primary_default_constraint" DEFAULT 0 NOT NULL, CONSTRAINT "addresses_uuid_unique_constraint" UNIQUE ("uuid"), CONSTRAINT "addresses_contact_id_foreign_key" FOREIGN KEY ("contact_id") REFERENCES "contacts" ("id"), CONSTRAINT "addresses_address_type_id_foreign_key" FOREIGN KEY ("address_type_id") REFERENCES "address_types" ("id"));
GO
CREATE UNIQUE INDEX "addresses_address_contact_id_address_type_id_index_index" ON "addresses" ("contact_id", "address_type_id");
GO
CREATE TRIGGER "trg_addresses_updated_at"
AFTER UPDATE ON "addresses"
FOR EACH ROW
WHEN NEW."updated" IS OLD."updated"
BEGIN
  UPDATE "addresses" SET "updated" = (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')) WHERE "id" = OLD."id";
END;
GO
CREATE TABLE "contact_attributes" ("id" INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL, "uuid" TEXT CONSTRAINT "contact_attributes_uuid_default_constraint" DEFAULT (lower(hex(randomblob(4))) || '-' || lower(hex(randomblob(2))) || '-4' || substr(lower(hex(randomblob(2))), 2) || '-' || substr('89ab', abs(random()) % 4 + 1, 1) || substr(lower(hex(randomblob(2))), 2) || '-' || lower(hex(randomblob(6)))) NOT NULL, "created" TEXT CONSTRAINT "contact_attributes_created_default_constraint" DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')) NOT NULL, "updated" TEXT CONSTRAINT "contact_attributes_updated_default_constraint" DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')) NOT NULL, "version" BLOB CONSTRAINT "contact_attributes_version_default_constraint" DEFAULT X'00' NOT NULL, "contact_id" INTEGER NOT NULL, "key" VARCHAR(255) NOT NULL, "value" TEXT NOT NULL, CONSTRAINT "contact_attributes_uuid_unique_constraint" UNIQUE ("uuid"), CONSTRAINT "contact_attributes_contact_id_foreign_key" FOREIGN KEY ("contact_id") REFERENCES "contacts" ("id"));
GO
CREATE INDEX "contact_attributes_contact_attribute_contact_id_name_index_index" ON "contact_attributes" ("contact_id", "key");
GO
CREATE UNIQUE INDEX "contact_attributes_owner_key_index" ON "contact_attributes" ("contact_id", "key");
GO
CREATE TRIGGER "trg_contact_attributes_updated_at"
AFTER UPDATE ON "contact_attributes"
FOR EACH ROW
WHEN NEW."updated" IS OLD."updated"
BEGIN
  UPDATE "contact_attributes" SET "updated" = (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')) WHERE "id" = OLD."id";
END;
GO
CREATE TABLE "address_attributes" ("id" INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL, "uuid" TEXT CONSTRAINT "address_attributes_uuid_default_constraint" DEFAULT (lower(hex(randomblob(4))) || '-' || lower(hex(randomblob(2))) || '-4' || substr(lower(hex(randomblob(2))), 2) || '-' || substr('89ab', abs(random()) % 4 + 1, 1) || substr(lower(hex(randomblob(2))), 2) || '-' || lower(hex(randomblob(6)))) NOT NULL, "created" TEXT CONSTRAINT "address_attributes_created_default_constraint" DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')) NOT NULL, "updated" TEXT CONSTRAINT "address_attributes_updated_default_constraint" DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')) NOT NULL, "version" BLOB CONSTRAINT "address_attributes_version_default_constraint" DEFAULT X'00' NOT NULL, "address_id" INTEGER NOT NULL, "key" VARCHAR(255) NOT NULL, "value" TEXT NOT NULL, CONSTRAINT "address_attributes_uuid_unique_constraint" UNIQUE ("uuid"), CONSTRAINT "address_attributes_address_id_foreign_key" FOREIGN KEY ("address_id") REFERENCES "addresses" ("id"));
GO
CREATE UNIQUE INDEX "address_attributes_owner_key_index" ON "address_attributes" ("address_id", "key");
GO
CREATE TRIGGER "trg_address_attributes_updated_at"
AFTER UPDATE ON "address_attributes"
FOR EACH ROW
WHEN NEW."updated" IS OLD."updated"
BEGIN
  UPDATE "address_attributes" SET "updated" = (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')) WHERE "id" = OLD."id";
END;
GO
CREATE TABLE "email_attributes" ("id" INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL, "uuid" TEXT CONSTRAINT "email_attributes_uuid_default_constraint" DEFAULT (lower(hex(randomblob(4))) || '-' || lower(hex(randomblob(2))) || '-4' || substr(lower(hex(randomblob(2))), 2) || '-' || substr('89ab', abs(random()) % 4 + 1, 1) || substr(lower(hex(randomblob(2))), 2) || '-' || lower(hex(randomblob(6)))) NOT NULL, "created" TEXT CONSTRAINT "email_attributes_created_default_constraint" DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')) NOT NULL, "updated" TEXT CONSTRAINT "email_attributes_updated_default_constraint" DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')) NOT NULL, "version" BLOB CONSTRAINT "email_attributes_version_default_constraint" DEFAULT X'00' NOT NULL, "email_id" INTEGER NOT NULL, "key" VARCHAR(255) NOT NULL, "value" TEXT NOT NULL, CONSTRAINT "email_attributes_uuid_unique_constraint" UNIQUE ("uuid"), CONSTRAINT "email_attributes_email_id_foreign_key" FOREIGN KEY ("email_id") REFERENCES "emails" ("id"));
GO
CREATE UNIQUE INDEX "email_attributes_owner_key_index" ON "email_attributes" ("email_id", "key");
GO
CREATE TRIGGER "trg_email_attributes_updated_at"
AFTER UPDATE ON "email_attributes"
FOR EACH ROW
WHEN NEW."updated" IS OLD."updated"
BEGIN
  UPDATE "email_attributes" SET "updated" = (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')) WHERE "id" = OLD."id";
END;
GO
CREATE TABLE "phone_attributes" ("id" INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL, "uuid" TEXT CONSTRAINT "phone_attributes_uuid_default_constraint" DEFAULT (lower(hex(randomblob(4))) || '-' || lower(hex(randomblob(2))) || '-4' || substr(lower(hex(randomblob(2))), 2) || '-' || substr('89ab', abs(random()) % 4 + 1, 1) || substr(lower(hex(randomblob(2))), 2) || '-' || lower(hex(randomblob(6)))) NOT NULL, "created" TEXT CONSTRAINT "phone_attributes_created_default_constraint" DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')) NOT NULL, "updated" TEXT CONSTRAINT "phone_attributes_updated_default_constraint" DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')) NOT NULL, "version" BLOB CONSTRAINT "phone_attributes_version_default_constraint" DEFAULT X'00' NOT NULL, "phone_id" INTEGER NOT NULL, "key" VARCHAR(255) NOT NULL, "value" TEXT NOT NULL, CONSTRAINT "phone_attributes_uuid_unique_constraint" UNIQUE ("uuid"), CONSTRAINT "phone_attributes_phone_id_foreign_key" FOREIGN KEY ("phone_id") REFERENCES "phones" ("id"));
GO
CREATE UNIQUE INDEX "phone_attributes_owner_key_index" ON "phone_attributes" ("phone_id", "key");
GO
CREATE TRIGGER "trg_phone_attributes_updated_at"
AFTER UPDATE ON "phone_attributes"
FOR EACH ROW
WHEN NEW."updated" IS OLD."updated"
BEGIN
  UPDATE "phone_attributes" SET "updated" = (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')) WHERE "id" = OLD."id";
END;
GO
CREATE TABLE "labels" ("id" INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL, "uuid" TEXT CONSTRAINT "labels_uuid_default_constraint" DEFAULT (lower(hex(randomblob(4))) || '-' || lower(hex(randomblob(2))) || '-4' || substr(lower(hex(randomblob(2))), 2) || '-' || substr('89ab', abs(random()) % 4 + 1, 1) || substr(lower(hex(randomblob(2))), 2) || '-' || lower(hex(randomblob(6)))) NOT NULL, "created" TEXT CONSTRAINT "labels_created_default_constraint" DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')) NOT NULL, "updated" TEXT CONSTRAINT "labels_updated_default_constraint" DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')) NOT NULL, "version" BLOB CONSTRAINT "labels_version_default_constraint" DEFAULT X'00' NOT NULL, "name" VARCHAR(255) NOT NULL, "description" TEXT CONSTRAINT "labels_description_default_constraint" DEFAULT '' NOT NULL, CONSTRAINT "labels_uuid_unique_constraint" UNIQUE ("uuid"), CONSTRAINT "labels_name_unique_constraint" UNIQUE ("name"));
GO
CREATE TRIGGER "trg_labels_updated_at"
AFTER UPDATE ON "labels"
FOR EACH ROW
WHEN NEW."updated" IS OLD."updated"
BEGIN
  UPDATE "labels" SET "updated" = (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')) WHERE "id" = OLD."id";
END;
GO
CREATE TABLE "contact_labels" ("id" INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL, "uuid" TEXT CONSTRAINT "contact_labels_uuid_default_constraint" DEFAULT (lower(hex(randomblob(4))) || '-' || lower(hex(randomblob(2))) || '-4' || substr(lower(hex(randomblob(2))), 2) || '-' || substr('89ab', abs(random()) % 4 + 1, 1) || substr(lower(hex(randomblob(2))), 2) || '-' || lower(hex(randomblob(6)))) NOT NULL, "created" TEXT CONSTRAINT "contact_labels_created_default_constraint" DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')) NOT NULL, "updated" TEXT CONSTRAINT "contact_labels_updated_default_constraint" DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')) NOT NULL, "version" BLOB CONSTRAINT "contact_labels_version_default_constraint" DEFAULT X'00' NOT NULL, "contact_id" INTEGER NOT NULL, "label_id" INTEGER NOT NULL, CONSTRAINT "contact_labels_uuid_unique_constraint" UNIQUE ("uuid"), CONSTRAINT "contact_labels_contact_id_foreign_key" FOREIGN KEY ("contact_id") REFERENCES "contacts" ("id"), CONSTRAINT "contact_labels_label_id_foreign_key" FOREIGN KEY ("label_id") REFERENCES "labels" ("id"));
GO
CREATE UNIQUE INDEX "contact_labels_contact_label_contact_id_label_id_index_index" ON "contact_labels" ("contact_id", "label_id");
GO
CREATE INDEX "contact_labels_contact_label_label_id_index_index" ON "contact_labels" ("label_id");
GO
CREATE TRIGGER "trg_contact_labels_updated_at"
AFTER UPDATE ON "contact_labels"
FOR EACH ROW
WHEN NEW."updated" IS OLD."updated"
BEGIN
  UPDATE "contact_labels" SET "updated" = (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')) WHERE "id" = OLD."id";
END;
GO
CREATE TABLE "address_labels" ("id" INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL, "uuid" TEXT CONSTRAINT "address_labels_uuid_default_constraint" DEFAULT (lower(hex(randomblob(4))) || '-' || lower(hex(randomblob(2))) || '-4' || substr(lower(hex(randomblob(2))), 2) || '-' || substr('89ab', abs(random()) % 4 + 1, 1) || substr(lower(hex(randomblob(2))), 2) || '-' || lower(hex(randomblob(6)))) NOT NULL, "created" TEXT CONSTRAINT "address_labels_created_default_constraint" DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')) NOT NULL, "updated" TEXT CONSTRAINT "address_labels_updated_default_constraint" DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')) NOT NULL, "version" BLOB CONSTRAINT "address_labels_version_default_constraint" DEFAULT X'00' NOT NULL, "address_id" INTEGER NOT NULL, "label_id" INTEGER NOT NULL, CONSTRAINT "address_labels_uuid_unique_constraint" UNIQUE ("uuid"), CONSTRAINT "address_labels_address_id_foreign_key" FOREIGN KEY ("address_id") REFERENCES "addresses" ("id"), CONSTRAINT "address_labels_label_id_foreign_key" FOREIGN KEY ("label_id") REFERENCES "labels" ("id"));
GO
CREATE TRIGGER "trg_address_labels_updated_at"
AFTER UPDATE ON "address_labels"
FOR EACH ROW
WHEN NEW."updated" IS OLD."updated"
BEGIN
  UPDATE "address_labels" SET "updated" = (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')) WHERE "id" = OLD."id";
END;
GO
CREATE TABLE "email_labels" ("id" INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL, "uuid" TEXT CONSTRAINT "email_labels_uuid_default_constraint" DEFAULT (lower(hex(randomblob(4))) || '-' || lower(hex(randomblob(2))) || '-4' || substr(lower(hex(randomblob(2))), 2) || '-' || substr('89ab', abs(random()) % 4 + 1, 1) || substr(lower(hex(randomblob(2))), 2) || '-' || lower(hex(randomblob(6)))) NOT NULL, "created" TEXT CONSTRAINT "email_labels_created_default_constraint" DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')) NOT NULL, "updated" TEXT CONSTRAINT "email_labels_updated_default_constraint" DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')) NOT NULL, "version" BLOB CONSTRAINT "email_labels_version_default_constraint" DEFAULT X'00' NOT NULL, "email_id" INTEGER NOT NULL, "label_id" INTEGER NOT NULL, CONSTRAINT "email_labels_uuid_unique_constraint" UNIQUE ("uuid"), CONSTRAINT "email_labels_email_id_foreign_key" FOREIGN KEY ("email_id") REFERENCES "emails" ("id"), CONSTRAINT "email_labels_label_id_foreign_key" FOREIGN KEY ("label_id") REFERENCES "labels" ("id"));
GO
CREATE TRIGGER "trg_email_labels_updated_at"
AFTER UPDATE ON "email_labels"
FOR EACH ROW
WHEN NEW."updated" IS OLD."updated"
BEGIN
  UPDATE "email_labels" SET "updated" = (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')) WHERE "id" = OLD."id";
END;
GO
CREATE TABLE "phone_labels" ("id" INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL, "uuid" TEXT CONSTRAINT "phone_labels_uuid_default_constraint" DEFAULT (lower(hex(randomblob(4))) || '-' || lower(hex(randomblob(2))) || '-4' || substr(lower(hex(randomblob(2))), 2) || '-' || substr('89ab', abs(random()) % 4 + 1, 1) || substr(lower(hex(randomblob(2))), 2) || '-' || lower(hex(randomblob(6)))) NOT NULL, "created" TEXT CONSTRAINT "phone_labels_created_default_constraint" DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')) NOT NULL, "updated" TEXT CONSTRAINT "phone_labels_updated_default_constraint" DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')) NOT NULL, "version" BLOB CONSTRAINT "phone_labels_version_default_constraint" DEFAULT X'00' NOT NULL, "phone_id" INTEGER NOT NULL, "label_id" INTEGER NOT NULL, CONSTRAINT "phone_labels_uuid_unique_constraint" UNIQUE ("uuid"), CONSTRAINT "phone_labels_phone_id_foreign_key" FOREIGN KEY ("phone_id") REFERENCES "phones" ("id"), CONSTRAINT "phone_labels_label_id_foreign_key" FOREIGN KEY ("label_id") REFERENCES "labels" ("id"));
GO
CREATE TRIGGER "trg_phone_labels_updated_at"
AFTER UPDATE ON "phone_labels"
FOR EACH ROW
WHEN NEW."updated" IS OLD."updated"
BEGIN
  UPDATE "phone_labels" SET "updated" = (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')) WHERE "id" = OLD."id";
END;
GO
-- Seed data
-- Seeds: genders
GO
INSERT INTO "genders" ("id", "uuid", "name", "description") VALUES (1, '9a81c98b-b74f-55fc-ba2c-02365a89ea86', 'Male', 'Male');
GO
INSERT INTO "genders" ("id", "uuid", "name", "description") VALUES (2, 'e35b9efd-f2e0-5b76-a6df-521469c97c1b', 'Female', 'Female');
GO
INSERT INTO "genders" ("id", "uuid", "name", "description") VALUES (3, '259d0d77-76c9-50f7-a197-f7b0064b161c', 'Other', 'Other');
GO
INSERT INTO "genders" ("id", "uuid", "name", "description") VALUES (4, '7d42e061-bac3-5e43-aa4c-fd0a7924fa5b', 'Agender', 'A person who does not identify themselves with or experience any gender. Also called null-gender, genderless, gendervoid, or neutral gender.');
GO
INSERT INTO "genders" ("id", "uuid", "name", "description") VALUES (5, '6ec4e277-333d-5e40-9695-844289209c2d', 'Abimegender', 'Associated with being profound, deep, and infinite. May be used alone or in combination with other genders.');
GO
INSERT INTO "genders" ("id", "uuid", "name", "description") VALUES (6, '14f0a0f0-2495-5375-9c45-eaca1b180651', 'Adamas gender', 'A gender that is indefinable or indomitable. People identifying with this gender refuse to be categorized in any particular gender identity.');
GO
INSERT INTO "genders" ("id", "uuid", "name", "description") VALUES (7, '91d2c24a-1896-5772-87ce-bee6175cfb1f', 'Aerogender', 'Also called evaisgender, this gender identity changes according to one''s surroundings.');
GO
INSERT INTO "genders" ("id", "uuid", "name", "description") VALUES (8, 'c2970dfe-44be-539c-ab82-cab1c8412fd0', 'Aesthetigender', 'Also called aesthetgender, a type of gender identity derived from aesthetics.');
GO
INSERT INTO "genders" ("id", "uuid", "name", "description") VALUES (9, '7f51ee52-a2fb-5478-abb4-17f2d828ddee', 'Affectugender', 'Based on the person''s mood swings or fluctuations.');
GO
INSERT INTO "genders" ("id", "uuid", "name", "description") VALUES (10, 'af40649a-59ee-5626-8839-4d7fcc4e3bdd', 'Agenderflux', 'Mostly agender with brief shifts of belonging to other gender types.');
GO
INSERT INTO "genders" ("id", "uuid", "name", "description") VALUES (11, 'b3304b19-4885-5647-b486-eff531d360cf', 'Alexigender', 'A fluid gender identity between more than one type of gender although the person cannot name the genders they feel fluid in.');
GO
INSERT INTO "genders" ("id", "uuid", "name", "description") VALUES (12, '551c0b54-2fdb-5027-a583-1943118eda3d', 'Aliusgender', 'Stands apart from existing social gender constructs. A strong specific gender identity that is neither male nor female.');
GO
INSERT INTO "genders" ("id", "uuid", "name", "description") VALUES (13, 'aaf0ed50-15cc-50c5-b0a3-45ad541f77de', 'Amaregender', 'A gender identity that changes depending on the person one is emotionally attached to.');
GO
INSERT INTO "genders" ("id", "uuid", "name", "description") VALUES (14, '262a80da-4a72-54ff-a73a-a8b9d30a3aa4', 'Ambigender', 'Two specific gender identities simultaneously without any fluidity or fluctuations.');
GO
INSERT INTO "genders" ("id", "uuid", "name", "description") VALUES (15, 'df35e5ba-7e81-5b4b-8ead-1c8174011f5a', 'Ambonec', 'The person identifies as both man and woman and yet does not belong to either.');
GO
INSERT INTO "genders" ("id", "uuid", "name", "description") VALUES (16, 'dc3b2bba-09f9-5558-8857-509b7674e77b', 'Amicagender', 'A gender-fluid identity where a person changes their gender depending on the friends they have.');
GO
INSERT INTO "genders" ("id", "uuid", "name", "description") VALUES (17, 'c0529ffa-75e1-5cf4-94ae-e10f530798ba', 'Androgyne', 'A combination of feminine and masculine genders.');
GO
INSERT INTO "genders" ("id", "uuid", "name", "description") VALUES (18, 'e77df1e5-2d00-5e0f-ae9e-6571f62e4cf7', 'Anesigender', 'Feeling close to a specific type of gender despite being more comfortable identifying with another gender.');
GO
INSERT INTO "genders" ("id", "uuid", "name", "description") VALUES (19, '6f9fcd02-935f-57e0-be3b-8a953784ee6f', 'Angenital', 'Desiring to be without any primary sexual characteristics without identifying as genderless.');
GO
INSERT INTO "genders" ("id", "uuid", "name", "description") VALUES (20, 'bd47f1f7-5f2b-5606-bb12-f5661ac577ae', 'Anogender', 'Gender identity fades in and out in intensity but always comes back to the same gendered feeling.');
GO
INSERT INTO "genders" ("id", "uuid", "name", "description") VALUES (21, 'ffcad1cc-18af-5bac-8423-a58071dce22c', 'Anongender', 'Having a gender identity but not labeling it or preferring not to have a label.');
GO
INSERT INTO "genders" ("id", "uuid", "name", "description") VALUES (22, 'd992d30b-6063-56c9-8f3c-9f8e961e471c', 'Antegender', 'A protean gender that can be anything but is formless and motionless.');
GO
INSERT INTO "genders" ("id", "uuid", "name", "description") VALUES (23, '4d6f3cdb-4378-5d98-a885-b3090ebc0c0d', 'Anxiegender', 'A gender identity with anxiety as its prominent characteristic.');
GO
INSERT INTO "genders" ("id", "uuid", "name", "description") VALUES (24, '2f2d2aa4-14b1-58fe-9966-a0612f4913da', 'Apagender', 'Apathy or a lack of feelings toward one''s gender identity.');
GO
INSERT INTO "genders" ("id", "uuid", "name", "description") VALUES (25, 'd8080e89-b7de-52d7-847c-4fcb3a7cc4e2', 'Apconsugender', 'Knowing what are not the characteristics of gender but not knowing what are its characteristics.');
GO
INSERT INTO "genders" ("id", "uuid", "name", "description") VALUES (26, '1f1a5492-361e-5750-9621-d27eb533468d', 'Astergender', 'A bright and celestial gender identity.');
GO
INSERT INTO "genders" ("id", "uuid", "name", "description") VALUES (27, '5b98e88c-592f-5706-a82f-aa5826d6a988', 'Astral gender', 'A gender identity that feels related to space.');
GO
INSERT INTO "genders" ("id", "uuid", "name", "description") VALUES (28, 'e839b579-22ba-589e-8c7f-0a37cf3c635e', 'Autigender', 'A gender identity that feels closely related to being autistic.');
GO
INSERT INTO "genders" ("id", "uuid", "name", "description") VALUES (29, '657e3c23-2fcf-564e-bb00-e1be6e66eb45', 'Autogender', 'A gender experience that is deeply connected and personal to oneself.');
GO
INSERT INTO "genders" ("id", "uuid", "name", "description") VALUES (30, '557017db-086b-5a07-9f31-45c9dbf457cf', 'Axigender', 'Between the two extremes of agender and any other type of gender, experienced one at a time without overlapping.');
GO
INSERT INTO "genders" ("id", "uuid", "name", "description") VALUES (31, '30897e5a-80e4-5cce-b249-b84b59f66262', 'Bigender', 'Two gender identities at the same or different times.');
GO
INSERT INTO "genders" ("id", "uuid", "name", "description") VALUES (32, 'd1ce4717-7367-5ebf-95d7-a201820664ad', 'Biogender', 'A gender closely related to nature.');
GO
INSERT INTO "genders" ("id", "uuid", "name", "description") VALUES (33, 'f7d7aa3a-92e0-5967-bcad-0b95ea95661c', 'Blurgender', 'More than one gender identity that blur into each other so no particular type is clear. Also called gender fuss.');
GO
INSERT INTO "genders" ("id", "uuid", "name", "description") VALUES (34, '8b579b30-90f2-5f6e-a3ea-9e7df80009da', 'Boyflux', 'Identifies as male but experiences varying degrees of male identity, ranging from agender to completely male.');
GO
INSERT INTO "genders" ("id", "uuid", "name", "description") VALUES (35, 'd0de2d1b-8d6f-5c4e-ba3f-f039c1549e8c', 'Burstgender', 'Frequent bursts of intense feelings quickly move to the initial calm stage.');
GO
INSERT INTO "genders" ("id", "uuid", "name", "description") VALUES (36, 'bf99c30c-039c-57d6-94dd-6541349ff5e8', 'Caelgender', 'Shares the qualities or aesthetics of outer space.');
GO
INSERT INTO "genders" ("id", "uuid", "name", "description") VALUES (37, 'a2506b84-21ad-52dc-a6b8-afa02f0f9956', 'Cassgender', 'Associated with considering gender irrelevant or unimportant.');
GO
INSERT INTO "genders" ("id", "uuid", "name", "description") VALUES (38, '332d3550-2054-5c0b-8608-d99690a3e26d', 'Cassflux', 'A fluctuating intensity of irrelevance toward gender.');
GO
INSERT INTO "genders" ("id", "uuid", "name", "description") VALUES (39, 'f6af310c-a362-5bed-8bda-da005c14eece', 'Cavusgender', 'Feeling close to one gender when depressed and to another when not depressed.');
GO
INSERT INTO "genders" ("id", "uuid", "name", "description") VALUES (40, 'ab557c88-9992-5e16-a9ac-4a0be0154cfa', 'Cendgender', 'Gender identity changes from one gender to its opposite.');
GO
INSERT INTO "genders" ("id", "uuid", "name", "description") VALUES (41, 'c13e2aed-f124-529a-82f4-aacfc7893fec', 'Ceterogender', 'A nonbinary gender with specific masculine, feminine, or neutral feelings.');
GO
INSERT INTO "genders" ("id", "uuid", "name", "description") VALUES (42, '5ceb107c-3891-5aae-8af1-47d0c7fbe30f', 'Ceterofluid', 'A ceterogender whose identity keeps fluctuating between different genders.');
GO
INSERT INTO "genders" ("id", "uuid", "name", "description") VALUES (43, '9abd716e-bee3-5df6-807e-83399c02cf17', 'Cisgender', 'Closely related to the gender assigned at birth during the entire life.');
GO
INSERT INTO "genders" ("id", "uuid", "name", "description") VALUES (44, '2614df92-cf09-5fa7-923f-2d38088513cf', 'Cloudgender', 'Gender cannot be comprehended due to depersonalization and derealization disorder.');
GO
INSERT INTO "genders" ("id", "uuid", "name", "description") VALUES (45, '723becfb-3143-556c-8917-efaecdb73876', 'Collgender', 'Various genders present at the same time.');
GO
INSERT INTO "genders" ("id", "uuid", "name", "description") VALUES (46, '2ee58eba-e067-55d0-b2c9-302527efcaef', 'Colorgender', 'Colors are used to describe gender, e.g., pink gender or black gender.');
GO
INSERT INTO "genders" ("id", "uuid", "name", "description") VALUES (47, '8d856103-ee2f-5c70-85b1-9aaedf62c5ff', 'Commogender', 'Knows they are not cisgender yet continues to identify as one for a while.');
GO
INSERT INTO "genders" ("id", "uuid", "name", "description") VALUES (48, '2ef9d325-1df7-531c-b450-7502d3d42e34', 'Condigender', 'Feels their gender only under specific circumstances.');
GO
INSERT INTO "genders" ("id", "uuid", "name", "description") VALUES (49, 'bd129e22-40ff-56e3-92e4-8928f17a69b2', 'Deliciagender', 'Having multiple genders but preferring one over the other.');
GO
INSERT INTO "genders" ("id", "uuid", "name", "description") VALUES (50, '7efeb822-2613-5ebd-b84f-37fb10e630b3', 'Demifluid', 'Multiple genders, some fluid while others are static.');
GO
INSERT INTO "genders" ("id", "uuid", "name", "description") VALUES (51, '3527671a-ce52-52a7-8094-2adb82b3122c', 'Demiflux', 'Multiple genders with some static and others fluctuating in intensity.');
GO
INSERT INTO "genders" ("id", "uuid", "name", "description") VALUES (52, 'b198b34e-58a6-5377-a9d3-a86372cd4a8e', 'Demigender', 'Partial traits of one gender and the rest of the other gender.');
GO
INSERT INTO "genders" ("id", "uuid", "name", "description") VALUES (53, 'e8c75270-9ee0-5b9c-8f93-d70279e979b5', 'Domgender', 'Multiple genders with one dominating over the rest.');
GO
INSERT INTO "genders" ("id", "uuid", "name", "description") VALUES (54, 'cfde6341-f526-5644-93c9-2e3b77ee11f0', 'Duragender', 'More than one gender with one lasting longer than the others.');
GO
INSERT INTO "genders" ("id", "uuid", "name", "description") VALUES (55, '91c662e3-2d01-56c5-8884-25016970f781', 'Egogender', 'A personal type of gender identified by the individual alone, based on experience within the self.');
GO
INSERT INTO "genders" ("id", "uuid", "name", "description") VALUES (56, 'be6dea08-2d97-5480-9066-594280b46778', 'Epicene', 'A strong feeling of not being able to relate to any or both of the binary gender characteristics.');
GO
INSERT INTO "genders" ("id", "uuid", "name", "description") VALUES (57, 'e94171d6-5084-592e-9e5d-19ac857e1d1c', 'Esspigender', 'Gender identity related to spirits.');
GO
INSERT INTO "genders" ("id", "uuid", "name", "description") VALUES (58, '8a217618-a207-506f-9cb8-66a31e90e7da', 'Exgender', 'Denial to identify with any gender on the gender spectrum.');
GO
INSERT INTO "genders" ("id", "uuid", "name", "description") VALUES (59, '02371de9-1693-5415-9066-40ea8dc60f4b', 'Existigender', 'Gender identity exists only when the person makes conscious efforts to realize it.');
GO
INSERT INTO "genders" ("id", "uuid", "name", "description") VALUES (60, '6a38bad5-c61e-5204-b552-86aca7b013ef', 'Femfluid', 'Fluid or fluctuating regarding the feminine genders.');
GO
INSERT INTO "genders" ("id", "uuid", "name", "description") VALUES (61, 'c50e4548-a542-5b35-acc3-284b750b93df', 'Femgender', 'A nonbinary gender identity that is feminine.');
GO
INSERT INTO "genders" ("id", "uuid", "name", "description") VALUES (62, '6d049426-e583-5467-85ea-ed132cc85a1a', 'Fluidflux', 'Fluid between two or more genders with fluctuation in intensity of those genders.');
GO
INSERT INTO "genders" ("id", "uuid", "name", "description") VALUES (63, 'a7991187-ad26-543b-a1b5-0401094e2507', 'Gemigender', 'Two genders that are opposite yet flux and work together.');
GO
INSERT INTO "genders" ("id", "uuid", "name", "description") VALUES (64, 'afeec458-0b85-514d-9f43-1c5cf88f68bb', 'Genderblank', 'Closely related to a blank space.');
GO
INSERT INTO "genders" ("id", "uuid", "name", "description") VALUES (65, '431ebf68-70eb-5d26-8bb7-c0f630a070df', 'Genderflow', 'Gender identity is fluid between infinite feelings.');
GO
INSERT INTO "genders" ("id", "uuid", "name", "description") VALUES (66, '6a745a08-87a1-5ac9-a655-bcc84f4e6a1c', 'Genderfluid', 'Does not consistently adhere to one fixed gender and may have many genders.');
GO
INSERT INTO "genders" ("id", "uuid", "name", "description") VALUES (67, '2b2ccbdf-4e3d-515c-a54e-5c4931968191', 'Genderfuzz', 'More than one gender is blurred together.');
GO
INSERT INTO "genders" ("id", "uuid", "name", "description") VALUES (68, '56237acd-1c7f-59ed-b427-b2bf2c9480b9', 'Genderflux', 'Gender fluctuates in intensity.');
GO
INSERT INTO "genders" ("id", "uuid", "name", "description") VALUES (69, '08c2e05b-222d-57db-9bc7-a171466db27f', 'Genderpuck', 'Resists fitting in societal norms concerning genders.');
GO
INSERT INTO "genders" ("id", "uuid", "name", "description") VALUES (70, 'd7351b7a-097c-5883-9810-ef24e1b135e6', 'Genderqueer', 'Blurs the preconceived boundaries of gender in relation to the gender binary or having just one gender type.');
GO
INSERT INTO "genders" ("id", "uuid", "name", "description") VALUES (71, 'e60a3461-2c40-5b56-96ec-5c94e83ee75a', 'Gender witched', 'Inclined toward having one gender but does not know which.');
GO
INSERT INTO "genders" ("id", "uuid", "name", "description") VALUES (72, 'e03c40fb-a4fb-5a74-9639-9b1753d6a71a', 'Girlflux', 'Identifies as female but with varying intensities of female identity.');
GO
INSERT INTO "genders" ("id", "uuid", "name", "description") VALUES (73, '7d1e424f-540b-5996-bcda-a53f6fd24a0f', 'Healgender', 'A gender identity that gives the person peace, calm, and positivity.');
GO
INSERT INTO "genders" ("id", "uuid", "name", "description") VALUES (74, 'b49fcbb7-82df-5f35-9958-fa942df1ea0a', 'Mirrorgender', 'Changing one''s gender type based on the people surrounding.');
GO
INSERT INTO "genders" ("id", "uuid", "name", "description") VALUES (75, 'd0fbbea2-8e5b-56df-93e8-d6ae49a53219', 'Omnigender', 'Having or experiencing all genders.');
GO
-- Seeds: email_types
GO
INSERT INTO "email_types" ("id", "uuid", "name", "description") VALUES (1, 'ba9d1120-def1-5f64-9481-3d6ffe83e080', 'Work', 'Work');
GO
INSERT INTO "email_types" ("id", "uuid", "name", "description") VALUES (2, '89988ebb-6d06-5558-b58c-fb63bf1cb1b3', 'Personal', 'Personal');
GO
INSERT INTO "email_types" ("id", "uuid", "name", "description") VALUES (3, 'e7696dd3-2a8c-50e7-80ae-50f66c51d085', 'Billing', 'Billing');
GO
INSERT INTO "email_types" ("id", "uuid", "name", "description") VALUES (4, '520c4e2b-1554-5edf-9132-d9906c2e1f0e', 'Admin', 'Admin');
GO
INSERT INTO "email_types" ("id", "uuid", "name", "description") VALUES (5, 'd48b9dfd-dff4-5c95-9c26-86743fa5f93e', 'Temp', 'Temp');
GO
INSERT INTO "email_types" ("id", "uuid", "name", "description") VALUES (6, '059fe89b-9db7-5b0c-9d48-10e1b22a9895', 'Other', 'Other');
GO
-- Seeds: phone_types
GO
INSERT INTO "phone_types" ("id", "uuid", "name", "description") VALUES (1, '775f24d4-e8cb-505c-9671-1d53c44f2780', 'Mobile', 'Mobile');
GO
INSERT INTO "phone_types" ("id", "uuid", "name", "description") VALUES (2, 'f5400d5e-2d9c-5f28-855a-3c893af2e951', 'Work', 'Work');
GO
INSERT INTO "phone_types" ("id", "uuid", "name", "description") VALUES (3, '3341543b-9c7c-5006-9b77-527ea9521221', 'Home', 'Home');
GO
INSERT INTO "phone_types" ("id", "uuid", "name", "description") VALUES (4, 'c86ffd75-2e88-56c0-81a7-a6bf8fbbccf7', 'Fax', 'Fax');
GO
INSERT INTO "phone_types" ("id", "uuid", "name", "description") VALUES (5, '6a107d46-427f-5501-935a-7b2d7a1f7361', 'Other', 'Other');
GO
-- Seeds: address_types
GO
INSERT INTO "address_types" ("id", "uuid", "name", "description") VALUES (1, '1e75343a-f291-5115-a555-47c1f64a2c69', 'Home', 'Home');
GO
INSERT INTO "address_types" ("id", "uuid", "name", "description") VALUES (2, '17222ad6-7a4a-547c-b7c7-083cf9664323', 'Work', 'Work');
GO
INSERT INTO "address_types" ("id", "uuid", "name", "description") VALUES (3, 'd393ea16-d09f-5ce4-943c-96d390ae888c', 'Billing', 'Billing');
GO
INSERT INTO "address_types" ("id", "uuid", "name", "description") VALUES (4, 'c3723ff5-17fb-5b95-97bb-6222e5fa1b6e', 'Shipping', 'Shipping');
GO
INSERT INTO "address_types" ("id", "uuid", "name", "description") VALUES (5, 'fa48e29e-cfbc-50d9-b1b1-a9021e9725fb', 'Other', 'Other');
GO
-- Seeds: labels
GO
INSERT INTO "labels" ("id", "uuid", "name", "description") VALUES (1, '62b21d4c-39ec-585d-8461-9352d97bc56d', 'Family', 'Family members and relatives');
GO
INSERT INTO "labels" ("id", "uuid", "name", "description") VALUES (2, 'ae2f4fca-f8b6-544f-903c-06487fb1aefc', 'Friends', 'Personal friends');
GO
INSERT INTO "labels" ("id", "uuid", "name", "description") VALUES (3, '9bd502c5-a97d-5b2e-9b6e-571e62d814f5', 'Work', 'Work and professional contacts');
