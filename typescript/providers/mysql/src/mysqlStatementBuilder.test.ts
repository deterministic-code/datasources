import { describe, expect, it } from "vitest";
import { DatasourceError, WhereBuilder } from "@datasources/core";
import { MysqlStatementBuilder } from "./mysqlStatementBuilder.ts";

const userColumns = [
  { name: "id", type: "int", id: true },
  { name: "uuid", type: "uuid" },
  { name: "username", type: "string" },
  { name: "is_enabled", type: "bool" },
  { name: "balance", type: "decimal" },
  { name: "location", type: "float" },
  { name: "created", type: "datetime" },
  { name: "version", type: "binary" },
];

describe("MysqlStatementBuilder", () => {
  it("quotes identifiers and maps canonical types", () => {
    expect(MysqlStatementBuilder.quote("user")).toBe("`user`");
    expect(MysqlStatementBuilder.quote("a`b")).toBe("`a``b`");
    expect(MysqlStatementBuilder.columnType("int")).toBe("INT");
    expect(() => MysqlStatementBuilder.columnType("xml")).toThrow(DatasourceError);
    expect(() => MysqlStatementBuilder.columnType("xml")).toThrow(/Unknown mysql type "xml"/);
  });

  it("creates the user table", () => {
    expect(MysqlStatementBuilder.createTable("user", userColumns).getText()).toBe(
      "CREATE TABLE `user` (`id` INT PRIMARY KEY, `uuid` CHAR(36), `username` TEXT, `is_enabled` TINYINT, `balance` DECIMAL(18,4), `location` DOUBLE, `created` TEXT, `version` BLOB)",
    );
  });

  it("quotes tables in DML and keeps parameter names", () => {
    const where = WhereBuilder.equals("id", 1);
    const fields = [
      { name: "id", value: 1 },
      { name: "username", value: "alice" },
    ];
    expect(MysqlStatementBuilder.createQuery("user", ["id", "username"], where).getText()).toBe(
      "SELECT `id`, `username` FROM `user` WHERE id = $id",
    );
    expect(MysqlStatementBuilder.createInsert("user", fields).getText()).toBe(
      "INSERT INTO `user` (id, username) VALUES ($id, $username)",
    );
    expect(MysqlStatementBuilder.createUpdate("user", fields, where).getText()).toBe(
      "UPDATE `user` SET id = $id, username = $username WHERE id = $id",
    );
    expect(MysqlStatementBuilder.createDelete("user", where).getText()).toBe(
      "DELETE FROM `user` WHERE id = $id",
    );
  });
});
