import { CreateTableStatement, DefaultStatementTextRenderer, type IDriver } from "@deterministic-code/datasources";
import { MysqlDriver } from "@deterministic-code/datasources-mysql";
import { PostgresDriver } from "@deterministic-code/datasources-postgres";
import { SqliteDriver } from "@deterministic-code/datasources-sqlite";
import bcrypt from "bcryptjs";
import { describe, expect, it } from "vitest";
import { listQuery, names, parse, passwordHash, usage } from "./main.ts";
import { readFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import { tables } from "../../../../typescript/schema-spec.ts";

const driver = (provider: string): IDriver =>
  ({
    sqlite: () => new SqliteDriver(),
    mysql: () => new MysqlDriver(),
    postgres: () => new PostgresDriver(),
  }[provider] ??
    (() => {
      throw new Error(`unknown provider ${provider}`);
    }))();

describe("parse", () => {
  it("parses commands", () => {
    expect(parse(["add-user", "a", "b"])).toEqual({ kind: "add-user", values: ["a", "b"] });
    expect(parse(["add-role", "admin"])).toEqual({ kind: "add-role", values: ["admin"] });
    expect(parse(["add-group", "staff"])).toEqual({ kind: "add-group", values: ["staff"] });
    expect(parse(["add-user-role", "a", "admin"])).toEqual({ kind: "add-user-role", values: ["a", "admin"] });
    expect(parse(["add-user-group", "a", "staff"])).toEqual({ kind: "add-user-group", values: ["a", "staff"] });
    expect(parse(["enable-user", "a"])).toEqual({ kind: "enable-user", values: ["a"] });
    expect(parse(["disable-user", "a"])).toEqual({ kind: "disable-user", values: ["a"] });
    expect(parse(["list-users-roles-groups"])).toEqual({ kind: "list-users-roles-groups", values: [] });
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
  return types.statementRenderer!.render(
    new CreateTableStatement(
      "user",
      columns.map(([name, type, id]) => ({
        name,
        type: id ? `${types.columnType(type)} PRIMARY KEY` : types.columnType(type),
      })),
    ),
  );
};

describe("schema", () => {
  it("creates tables with foreign keys", () => {
    const catalog = tables("user-roles-cli", (id) => id);
    const sql = catalog.map((table) => DefaultStatementTextRenderer.instance.render(table)).join("\n");
    expect(sql).toContain("FOREIGN KEY (user_id) REFERENCES");
    expect(sql).toContain("FOREIGN KEY (bad_role_id) REFERENCES");
    expect(sql).toContain("ugly_name");
    expect(catalog[0]!.columns[0]!.identity).toBe(true);
    expect(DefaultStatementTextRenderer.instance.render(catalog[0]!)).toContain("CONSTRAINT pk_users PRIMARY KEY");
    expect(userTable("sqlite")).toBe(
      'CREATE TABLE "user" ("id" INTEGER PRIMARY KEY, "uuid" TEXT, "username" TEXT, "is_enabled" BOOLEAN, "balance" NUMERIC, "location" REAL, "created" TEXT, "version" BLOB)',
    );
    expect(userTable("mysql")).toBe(
      "CREATE TABLE `user` (`id` INT PRIMARY KEY, `uuid` CHAR(36), `username` TEXT, `is_enabled` TINYINT, `balance` DECIMAL(18,4), `location` DOUBLE, `created` TEXT, `version` BLOB)",
    );
    expect(userTable("postgres")).toBe(
      'CREATE TABLE "user" ("id" INTEGER PRIMARY KEY, "uuid" UUID, "username" TEXT, "is_enabled" BOOLEAN, "balance" NUMERIC, "location" DOUBLE PRECISION, "created" TIMESTAMPTZ, "version" BYTEA)',
    );
    expect(names.map("name")).toBe("ugly_name");
    expect(names.map("role_id")).toBe("bad_role_id");
    expect(names.map("username")).toBe("username");
    expect(names.reverseMap("ugly_name")).toBe("name");
    expect(names.reverseMap("bad_role_id")).toBe("role_id");
    expect(names.reverseMap("username")).toBe("username");
    expect(
      readFileSync(
        join(dirname(fileURLToPath(import.meta.url)), "../../../user-roles-simple-cli-shared/mysql-procedures.sql"),
        "utf8",
      ),
    ).toContain("INSERT INTO `groups` (ugly_name");
    expect(DefaultStatementTextRenderer.instance.render(listQuery())).toBe(
      "SELECT users.id AS id, users.username AS username, users.is_enabled AS is_enabled, users.created AS created, roles.id AS role_id, roles.name AS role_name, roles.is_enabled AS role_is_enabled, groups.id AS group_id, groups.ugly_name AS group_name, groups.is_enabled AS group_is_enabled FROM users LEFT JOIN user_roles ON users.id = user_roles.user_id LEFT JOIN roles ON user_roles.bad_role_id = roles.id LEFT JOIN user_groups ON users.id = user_groups.user_id LEFT JOIN groups ON user_groups.group_id = groups.id ORDER BY users.id ASC, roles.id ASC, groups.id ASC",
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
