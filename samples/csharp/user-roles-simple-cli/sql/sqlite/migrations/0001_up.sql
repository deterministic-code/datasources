-- Generated migration for sqlite
CREATE TABLE "users" ("id" INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL, "username" TEXT NOT NULL, "password" TEXT NOT NULL, "nickname" TEXT, "is_enabled" BOOLEAN CONSTRAINT "users_is_enabled_default_constraint" DEFAULT 1 NOT NULL, "created" TEXT NOT NULL, "updated" BLOB CONSTRAINT "users_updated_default_constraint" DEFAULT X'00' NOT NULL);
CREATE TRIGGER "trg_users_updated_at"
AFTER UPDATE ON "users"
FOR EACH ROW
WHEN NEW."updated" IS OLD."updated"
BEGIN
  UPDATE "users" SET "updated" = (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')) WHERE "id" = OLD."id";
END;
CREATE TABLE "roles" ("id" INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL, "name" TEXT NOT NULL, "description" TEXT, "is_enabled" BOOLEAN CONSTRAINT "roles_is_enabled_default_constraint" DEFAULT 1 NOT NULL, "created" TEXT NOT NULL, "updated" BLOB CONSTRAINT "roles_updated_default_constraint" DEFAULT X'00' NOT NULL);
CREATE TRIGGER "trg_roles_updated_at"
AFTER UPDATE ON "roles"
FOR EACH ROW
WHEN NEW."updated" IS OLD."updated"
BEGIN
  UPDATE "roles" SET "updated" = (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')) WHERE "id" = OLD."id";
END;
CREATE TABLE "groups" ("id" INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL, "ugly_name" TEXT NOT NULL, "is_enabled" BOOLEAN CONSTRAINT "groups_is_enabled_default_constraint" DEFAULT 1 NOT NULL, "created" TEXT NOT NULL, "updated" BLOB CONSTRAINT "groups_updated_default_constraint" DEFAULT X'00' NOT NULL);
CREATE TRIGGER "trg_groups_updated_at"
AFTER UPDATE ON "groups"
FOR EACH ROW
WHEN NEW."updated" IS OLD."updated"
BEGIN
  UPDATE "groups" SET "updated" = (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')) WHERE "id" = OLD."id";
END;
CREATE TABLE "user_groups" ("id" INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL, "user_id" TEXT NOT NULL, "group_id" TEXT NOT NULL, "created" TEXT NOT NULL, "updated" BLOB CONSTRAINT "user_groups_updated_default_constraint" DEFAULT X'00' NOT NULL, CONSTRAINT "user_groups_user_id_foreign_key" FOREIGN KEY ("user_id") REFERENCES "users" ("id"), CONSTRAINT "user_groups_group_id_foreign_key" FOREIGN KEY ("group_id") REFERENCES "groups" ("id"));
CREATE TRIGGER "trg_user_groups_updated_at"
AFTER UPDATE ON "user_groups"
FOR EACH ROW
WHEN NEW."updated" IS OLD."updated"
BEGIN
  UPDATE "user_groups" SET "updated" = (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')) WHERE "id" = OLD."id";
END;
CREATE TABLE "user_roles" ("id" INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL, "user_id" TEXT NOT NULL, "bad_role_id" TEXT NOT NULL, "created" TEXT NOT NULL, "updated" BLOB CONSTRAINT "user_roles_updated_default_constraint" DEFAULT X'00' NOT NULL, CONSTRAINT "user_roles_user_id_foreign_key" FOREIGN KEY ("user_id") REFERENCES "users" ("id"), CONSTRAINT "user_roles_bad_role_id_foreign_key" FOREIGN KEY ("bad_role_id") REFERENCES "roles" ("id"));
CREATE TRIGGER "trg_user_roles_updated_at"
AFTER UPDATE ON "user_roles"
FOR EACH ROW
WHEN NEW."updated" IS OLD."updated"
BEGIN
  UPDATE "user_roles" SET "updated" = (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')) WHERE "id" = OLD."id";
END;
CREATE TABLE "sessions" ("id" INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL, "user_id" TEXT NOT NULL, "token" TEXT NOT NULL, "created" TEXT NOT NULL, "updated" BLOB CONSTRAINT "sessions_updated_default_constraint" DEFAULT X'00' NOT NULL, CONSTRAINT "sessions_user_id_foreign_key" FOREIGN KEY ("user_id") REFERENCES "users" ("id"));
CREATE TRIGGER "trg_sessions_updated_at"
AFTER UPDATE ON "sessions"
FOR EACH ROW
WHEN NEW."updated" IS OLD."updated"
BEGIN
  UPDATE "sessions" SET "updated" = (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')) WHERE "id" = OLD."id";
END;
CREATE TABLE "audit_logs" ("id" INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL, "message" TEXT NOT NULL, "created" TEXT NOT NULL, "updated" BLOB CONSTRAINT "audit_logs_updated_default_constraint" DEFAULT X'00' NOT NULL);
CREATE TRIGGER "trg_audit_logs_updated_at"
AFTER UPDATE ON "audit_logs"
FOR EACH ROW
WHEN NEW."updated" IS OLD."updated"
BEGIN
  UPDATE "audit_logs" SET "updated" = (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')) WHERE "id" = OLD."id";
END;
