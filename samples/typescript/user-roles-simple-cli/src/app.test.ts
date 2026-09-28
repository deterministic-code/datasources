import { MockCommand, MockConnection, MockReader } from "@datasources/core/mocks";
import { CreateTableStatement, FlatToNestedMapper, type IDriver } from "@datasources/core";
import { MysqlDriver } from "@datasources/mysql";
import { PostgresDriver } from "@datasources/postgres";
import { SqliteDriver } from "@datasources/sqlite";
import bcrypt from "bcryptjs";
import { describe, expect, it } from "vitest";
import { App } from "./app.ts";
import { asUserView } from "./as-user-view.ts";
import { columns } from "./columns.ts";
import { migrate, ifNotExists } from "./migrate.ts";
import { parse, usage } from "./parse.ts";
import { passwordHash } from "./password-hash.ts";
import { ident, mysqlProcedures, qualified, schema } from "./schema.ts";
import { usersRolesGroupsQuery } from "./users-roles-groups-query.ts";
import { usersView } from "./users-view.ts";

const driver = (provider: string): IDriver =>
  ({
    sqlite: () => new SqliteDriver(),
    mysql: () => new MysqlDriver(),
    postgres: () => new PostgresDriver(),
  }[provider] ??
    (() => {
      throw new Error(`unknown provider ${provider}`);
    }))();

const frozen = new Date("2026-09-22T00:00:00.000Z");
const token = () => new Uint8Array([1, 2, 3]);

const create = (provider: string, procedures = true) => {
  const connection = new MockConnection(undefined, provider);
  const command = new MockCommand(connection);
  connection.command = command;
  return { app: new App(command, () => frozen, (password) => `#${password}`, token, procedures), command };
};

describe("parse", () => {
  it("parses commands", () => {
    expect(parse(["add-user", "a", "b"])).toEqual({ kind: "add-user", username: "a", password: "b" });
    expect(parse(["add-role", "admin"])).toEqual({ kind: "add-role", name: "admin" });
    expect(parse(["add-group", "staff"])).toEqual({ kind: "add-group", name: "staff" });
    expect(parse(["add-user-role", "a", "admin"])).toEqual({
      kind: "add-user-role",
      username: "a",
      role: "admin",
    });
    expect(parse(["add-user-group", "a", "staff"])).toEqual({
      kind: "add-user-group",
      username: "a",
      group: "staff",
    });
    expect(parse(["enable-user", "a"])).toEqual({ kind: "enable-user", username: "a" });
    expect(parse(["disable-user", "a"])).toEqual({ kind: "disable-user", username: "a" });
    expect(parse(["list-users-roles-groups"])).toEqual({ kind: "list-users-roles-groups" });
  });

  it("rejects bad args", () => {
    for (const verb of [
      "add-user",
      "add-role",
      "add-group",
      "add-user-role",
      "add-user-group",
      "enable-user",
      "disable-user",
      "nope",
    ])
      expect(() => parse([verb])).toThrow();
    expect(() => parse([])).toThrow(usage);
    expect(() => parse(["add-user", "a", "b", "extra"])).toThrow();
    expect(() => parse(["add-role", "admin", "extra"])).toThrow();
    expect(() => parse(["add-group", "staff", "extra"])).toThrow();
    expect(() => parse(["add-user-role", "a", "admin", "extra"])).toThrow();
    expect(() => parse(["add-user-group", "a", "staff", "extra"])).toThrow();
    expect(() => parse(["enable-user", "a", "extra"])).toThrow();
    expect(() => parse(["disable-user", "a", "extra"])).toThrow();
    expect(() => parse(["list-users-roles-groups", "extra"])).toThrow();
  });
});

const userTable = (provider: string): string => {
  const columns = [
    ["id", "int", true],
    ["uuid", "uuid", false],
    ["username", "string", false],
    ["is_enabled", "bool", false],
    ["balance", "decimal", false],
    ["location", "float", false],
    ["created", "datetime", false],
    ["version", "binary", false],
  ] as const;
  const types = driver(provider);
  return new CreateTableStatement(
    ident(provider, "user"),
    columns.map(([name, type, id]) => ({
      name: ident(provider, name),
      type: id ? `${types.columnType(type)} PRIMARY KEY` : types.columnType(type),
    })),
  ).getText();
};

describe("schema", () => {
  it("creates tables with foreign keys", () => {
    for (const provider of ["sqlite", "mysql", "postgres"]) {
      const sql = schema(provider, driver(provider))
        .map((table) => table.getText())
        .join("\n");
      expect(sql).toContain(`FOREIGN KEY (${ident(provider, "user_id")}) REFERENCES`);
      expect(sql).toContain(`FOREIGN KEY (${ident(provider, "bad_role_id")}) REFERENCES`);
      expect(sql).toContain("ugly_name");
    }
    expect(schema("sqlite", driver("sqlite"))[0]!.columns[0]!.identity).toBe(true);
    expect(schema("sqlite", driver("sqlite"))[0]!.getText()).toContain("CONSTRAINT pk_users PRIMARY KEY");
    expect(schema("mysql", driver("mysql"))[0]!.columns[0]!.identity).toBe(true);
    expect(schema("postgres", driver("postgres"))[0]!.columns[0]!.identity).toBe(true);
    expect(ident("sqlite", "user")).toBe('"user"');
    expect(ident("postgres", 'a"b')).toBe('"a""b"');
    expect(ident("mysql", "a`b")).toBe("`a``b`");
    expect(qualified("sqlite", "users.id")).toBe('"users"."id"');
    expect(qualified("mysql", "users.id")).toBe("`users`.`id`");
    expect(userTable("sqlite")).toBe(
      'CREATE TABLE "user" ("id" INTEGER PRIMARY KEY, "uuid" TEXT, "username" TEXT, "is_enabled" INTEGER, "balance" NUMERIC, "location" REAL, "created" TEXT, "version" BLOB)',
    );
    expect(userTable("mysql")).toBe(
      "CREATE TABLE `user` (`id` INT PRIMARY KEY, `uuid` CHAR(36), `username` TEXT, `is_enabled` TINYINT, `balance` DECIMAL(18,4), `location` DOUBLE, `created` TEXT, `version` BLOB)",
    );
    expect(userTable("postgres")).toBe(
      'CREATE TABLE "user" ("id" INTEGER PRIMARY KEY, "uuid" UUID, "username" TEXT, "is_enabled" BOOLEAN, "balance" NUMERIC, "location" DOUBLE PRECISION, "created" TIMESTAMPTZ, "version" BYTEA)',
    );
    expect(ident("oracle", "groups")).toBe('"groups"');
    expect(columns.map("name")).toBe("ugly_name");
    expect(columns.map("role_id")).toBe("bad_role_id");
    expect(columns.map("username")).toBe("username");
    expect(columns.reverseMap("ugly_name")).toBe("name");
    expect(columns.reverseMap("bad_role_id")).toBe("role_id");
    expect(columns.reverseMap("username")).toBe("username");
    expect(ifNotExists("CREATE TABLE users (id INT)")).toBe("CREATE TABLE IF NOT EXISTS users (id INT)");
    expect(ifNotExists("SELECT 1")).toBe("SELECT 1");
    expect(mysqlProcedures().join("\n")).toContain("INSERT INTO `groups` (ugly_name");
    expect(usersRolesGroupsQuery("sqlite").getText()).toBe(
      'SELECT "users".id AS id, "users".username AS username, "users".is_enabled AS is_enabled, "users".created AS created, "roles".id AS role_id, "roles".name AS role_name, "roles".is_enabled AS role_is_enabled, "groups".id AS group_id, "groups".ugly_name AS group_name, "groups".is_enabled AS group_is_enabled FROM "users" LEFT JOIN "user_roles" ON "users".id = "user_roles".user_id LEFT JOIN "roles" ON "user_roles".bad_role_id = "roles".id LEFT JOIN "user_groups" ON "users".id = "user_groups".user_id LEFT JOIN "groups" ON "user_groups".group_id = "groups".id ORDER BY "users".id ASC, "roles".id ASC, "groups".id ASC',
    );
  });
});

describe("password", () => {
  it("hashes", () => {
    const hash = passwordHash("secret");
    expect(hash).not.toBe("secret");
    expect(bcrypt.compareSync("secret", hash)).toBe(true);
  });
});

describe("asUserView", () => {
  it("collapses join rows", () => {
    const users = asUserView(
      new FlatToNestedMapper(usersView).map([
        {
          id: 1,
          username: "alice",
          is_enabled: true,
          created: "2026-01-01T00:00:00.000Z",
          role_id: 2,
          role_name: "admin",
          role_is_enabled: 1,
          group_id: 3,
          group_name: "staff",
          group_is_enabled: false,
        },
        {
          id: 1,
          username: "alice",
          is_enabled: true,
          created: "2026-01-01T00:00:00.000Z",
          role_id: 4n,
          role_name: "ops",
          role_is_enabled: 0n,
          group_id: null,
          group_name: null,
          group_is_enabled: null,
        },
      ]),
    );
    expect(users).toHaveLength(1);
    expect(users[0]!.roles.map((role) => role.name)).toEqual(["admin", "ops"]);
    expect(users[0]!.groups.map((group) => group.name)).toEqual(["staff"]);
    expect(asUserView([{ id: "9", username: 1, is_enabled: null, created: new Date("2026-01-02T03:04:05.000Z"), roles: [], groups: [] } as never])[0]!.id).toBe(9);
    expect(asUserView([{ id: "x", username: null, is_enabled: "x", created: null, roles: undefined, groups: undefined } as never])[0]!.id).toBe(0);
  });
});

describe("app", () => {
  it("writes sqlite inserts and lists", async () => {
    const { app, command } = create("sqlite");
    command.reader = new MockReader([{ id: 7 }]);
    await app.run({ kind: "add-user", username: "alice", password: "secret" });
    await app.run({ kind: "add-role", name: "admin" });
    await app.run({ kind: "add-group", name: "staff" });
    command.reader = new MockReader([{ id: 1 }, { id: 2 }]);
    await app.run({ kind: "add-user-role", username: "alice", role: "admin" });
    command.reader = new MockReader([{ id: 1 }, { id: 3 }]);
    await app.run({ kind: "add-user-group", username: "alice", group: "staff" });
    command.reader = new MockReader([{ id: 1 }]);
    await app.run({ kind: "disable-user", username: "alice" });
    expect(command.executeCalls.some((call) => call.text.includes("INSERT INTO") && call.text.includes("users"))).toBe(true);
    expect(command.executeCalls.some((call) => call.text.includes("ugly_name"))).toBe(true);
    expect(command.executeCalls.some((call) => call.text.includes("bad_role_id"))).toBe(true);
    command.reader = new MockReader([
      undefined as never,
      {
        id: 1,
        username: "alice",
        is_enabled: false,
        created: "2026-09-22T00:00:00.000Z",
        role_id: 2,
        role_name: "admin",
        role_is_enabled: true,
        group_id: 3,
        group_name: "staff",
        group_is_enabled: true,
      },
    ]);
    const json = await app.run({ kind: "list-users-roles-groups" });
    expect(json).toContain('"username":"alice"');
    expect(json).not.toContain("password");
    expect(json).not.toContain("ugly_name");
  });

  it("writes mysql procedures", async () => {
    const { app, command } = create("mysql");
    command.reader = new MockReader([{ id: 1 }]);
    await app.run({ kind: "add-user", username: "alice", password: "secret" });
    await app.run({ kind: "add-role", name: "admin" });
    await app.run({ kind: "add-group", name: "staff" });
    command.reader = new MockReader([{ id: 1 }, { id: 2 }]);
    await app.run({ kind: "add-user-role", username: "alice", role: "admin" });
    command.reader = new MockReader([{ id: 1 }, { id: 3 }]);
    await app.run({ kind: "add-user-group", username: "alice", group: "staff" });
    await app.run({ kind: "enable-user", username: "alice" });
    await app.run({ kind: "disable-user", username: "alice" });
    expect(command.executeCalls.every((call) => call.text.startsWith("CALL "))).toBe(true);
  });

  it("mysql sql path inserts and updates", async () => {
    const { app, command } = create("mysql", false);
    await app.run({ kind: "add-user", username: "alice", password: "secret" });
    command.reader = new MockReader([{ id: 1 }]);
    await app.run({ kind: "enable-user", username: "alice" });
    expect(command.executeCalls.some((call) => call.text.includes("INSERT INTO"))).toBe(true);
    expect(command.executeCalls.some((call) => call.text.includes("UPDATE"))).toBe(true);
    expect(command.executeCalls.some((call) => call.text.startsWith("CALL "))).toBe(false);
  });

  it("uses default clock and hash", async () => {
    const connection = new MockConnection(undefined, "sqlite");
    const command = new MockCommand(connection);
    connection.command = command;
    await new App(command).run({ kind: "add-user", username: "alice", password: "secret" });
    expect(command.executeCalls.some((call) => call.text.includes("INSERT INTO") && call.text.includes("users"))).toBe(true);
  });

  it("migrates and rejects missing rows", async () => {
    const sqlite = new MockConnection(undefined, "sqlite");
    const sqliteCmd = new MockCommand(sqlite);
    sqlite.command = sqliteCmd;
    await migrate(sqliteCmd, new SqliteDriver());
    expect(sqliteCmd.executeCalls.every((call) => call.text.startsWith("CREATE TABLE IF NOT EXISTS"))).toBe(true);
    const mysql = new MockConnection(undefined, "mysql");
    const mysqlCmd = new MockCommand(mysql);
    mysql.command = mysqlCmd;
    await migrate(mysqlCmd, new MysqlDriver());
    expect(mysqlCmd.executeCalls.some((call) => call.text.includes("CREATE PROCEDURE add_user"))).toBe(true);
    const { app, command } = create("sqlite");
    command.reader = new MockReader([]);
    await expect(app.run({ kind: "enable-user", username: "missing" })).rejects.toThrow("not found");
    command.reader = new MockReader([{ id: "x" }]);
    await expect(app.run({ kind: "enable-user", username: "bad" })).rejects.toThrow("not found");
    await expect(app.run({ kind: "nope" } as never)).rejects.toThrow("empty command");
  });
});
