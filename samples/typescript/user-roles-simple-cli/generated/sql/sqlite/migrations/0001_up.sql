-- Generated migration for sqlite
CREATE TABLE "users" ("id" INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL, "uuid" TEXT CONSTRAINT "users_uuid_default_constraint" DEFAULT (lower(hex(randomblob(4))) || '-' || lower(hex(randomblob(2))) || '-4' || substr(lower(hex(randomblob(2))), 2) || '-' || substr('89ab', abs(random()) % 4 + 1, 1) || substr(lower(hex(randomblob(2))), 2) || '-' || lower(hex(randomblob(6)))) NOT NULL, "created" TEXT NOT NULL, "updated" BLOB CONSTRAINT "users_updated_default_constraint" DEFAULT X'00' NOT NULL, "username" TEXT NOT NULL, "password" TEXT NOT NULL, "is_enabled" BOOLEAN CONSTRAINT "users_is_enabled_default_constraint" DEFAULT 1 NOT NULL, CONSTRAINT "users_uuid_unique_constraint" UNIQUE ("uuid"));
GO
CREATE TRIGGER "trg_users_updated_at"
AFTER UPDATE ON "users"
FOR EACH ROW
WHEN NEW."updated" IS OLD."updated"
BEGIN
  UPDATE "users" SET "updated" = (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')) WHERE "id" = OLD."id";
END;
GO
CREATE TABLE "roles" ("id" INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL, "uuid" TEXT CONSTRAINT "roles_uuid_default_constraint" DEFAULT (lower(hex(randomblob(4))) || '-' || lower(hex(randomblob(2))) || '-4' || substr(lower(hex(randomblob(2))), 2) || '-' || substr('89ab', abs(random()) % 4 + 1, 1) || substr(lower(hex(randomblob(2))), 2) || '-' || lower(hex(randomblob(6)))) NOT NULL, "created" TEXT NOT NULL, "updated" BLOB CONSTRAINT "roles_updated_default_constraint" DEFAULT X'00' NOT NULL, "name" TEXT NOT NULL, "description" TEXT CONSTRAINT "roles_description_default_constraint" DEFAULT '' NOT NULL, "is_enabled" BOOLEAN CONSTRAINT "roles_is_enabled_default_constraint" DEFAULT 1 NOT NULL, CONSTRAINT "roles_uuid_unique_constraint" UNIQUE ("uuid"));
GO
CREATE TRIGGER "trg_roles_updated_at"
AFTER UPDATE ON "roles"
FOR EACH ROW
WHEN NEW."updated" IS OLD."updated"
BEGIN
  UPDATE "roles" SET "updated" = (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')) WHERE "id" = OLD."id";
END;
GO
CREATE TABLE "groups" ("id" INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL, "uuid" TEXT CONSTRAINT "groups_uuid_default_constraint" DEFAULT (lower(hex(randomblob(4))) || '-' || lower(hex(randomblob(2))) || '-4' || substr(lower(hex(randomblob(2))), 2) || '-' || substr('89ab', abs(random()) % 4 + 1, 1) || substr(lower(hex(randomblob(2))), 2) || '-' || lower(hex(randomblob(6)))) NOT NULL, "created" TEXT NOT NULL, "updated" BLOB CONSTRAINT "groups_updated_default_constraint" DEFAULT X'00' NOT NULL, "ugly_name" TEXT NOT NULL, "description" TEXT CONSTRAINT "groups_description_default_constraint" DEFAULT '' NOT NULL, "is_enabled" BOOLEAN CONSTRAINT "groups_is_enabled_default_constraint" DEFAULT 1 NOT NULL, CONSTRAINT "groups_uuid_unique_constraint" UNIQUE ("uuid"));
GO
CREATE TRIGGER "trg_groups_updated_at"
AFTER UPDATE ON "groups"
FOR EACH ROW
WHEN NEW."updated" IS OLD."updated"
BEGIN
  UPDATE "groups" SET "updated" = (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')) WHERE "id" = OLD."id";
END;
GO
CREATE TABLE "user_groups" ("id" INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL, "uuid" TEXT CONSTRAINT "user_groups_uuid_default_constraint" DEFAULT (lower(hex(randomblob(4))) || '-' || lower(hex(randomblob(2))) || '-4' || substr(lower(hex(randomblob(2))), 2) || '-' || substr('89ab', abs(random()) % 4 + 1, 1) || substr(lower(hex(randomblob(2))), 2) || '-' || lower(hex(randomblob(6)))) NOT NULL, "created" TEXT NOT NULL, "updated" BLOB CONSTRAINT "user_groups_updated_default_constraint" DEFAULT X'00' NOT NULL, "user_id" INTEGER NOT NULL, "group_id" INTEGER NOT NULL, CONSTRAINT "user_groups_uuid_unique_constraint" UNIQUE ("uuid"), CONSTRAINT "user_groups_user_id_foreign_key" FOREIGN KEY ("user_id") REFERENCES "users" ("id"), CONSTRAINT "user_groups_group_id_foreign_key" FOREIGN KEY ("group_id") REFERENCES "groups" ("id"));
GO
CREATE TRIGGER "trg_user_groups_updated_at"
AFTER UPDATE ON "user_groups"
FOR EACH ROW
WHEN NEW."updated" IS OLD."updated"
BEGIN
  UPDATE "user_groups" SET "updated" = (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')) WHERE "id" = OLD."id";
END;
GO
CREATE TABLE "user_roles" ("id" INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL, "uuid" TEXT CONSTRAINT "user_roles_uuid_default_constraint" DEFAULT (lower(hex(randomblob(4))) || '-' || lower(hex(randomblob(2))) || '-4' || substr(lower(hex(randomblob(2))), 2) || '-' || substr('89ab', abs(random()) % 4 + 1, 1) || substr(lower(hex(randomblob(2))), 2) || '-' || lower(hex(randomblob(6)))) NOT NULL, "created" TEXT NOT NULL, "updated" BLOB CONSTRAINT "user_roles_updated_default_constraint" DEFAULT X'00' NOT NULL, "user_id" INTEGER NOT NULL, "bad_role_id" INTEGER NOT NULL, CONSTRAINT "user_roles_uuid_unique_constraint" UNIQUE ("uuid"), CONSTRAINT "user_roles_user_id_foreign_key" FOREIGN KEY ("user_id") REFERENCES "users" ("id"), CONSTRAINT "user_roles_bad_role_id_foreign_key" FOREIGN KEY ("bad_role_id") REFERENCES "roles" ("id"));
GO
CREATE TRIGGER "trg_user_roles_updated_at"
AFTER UPDATE ON "user_roles"
FOR EACH ROW
WHEN NEW."updated" IS OLD."updated"
BEGIN
  UPDATE "user_roles" SET "updated" = (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')) WHERE "id" = OLD."id";
END;
