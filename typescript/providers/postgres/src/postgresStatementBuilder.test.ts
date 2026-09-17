import { describe, expect, it } from "vitest";
import { DatasourceError, WhereBuilder } from "@datasources/core";
import { PostgresStatementBuilder } from "./postgresStatementBuilder.ts";

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

describe("PostgresStatementBuilder", () => {
  it("quotes identifiers and maps canonical types", () => {
    expect(PostgresStatementBuilder.quote("user")).toBe('"user"');
    expect(PostgresStatementBuilder.quote('a"b')).toBe('"a""b"');
    expect(PostgresStatementBuilder.columnType("int")).toBe("INTEGER");
    expect(() => PostgresStatementBuilder.columnType("xml")).toThrow(DatasourceError);
    expect(() => PostgresStatementBuilder.columnType("xml")).toThrow(/Unknown postgres type "xml"/);
  });

  it("creates the user table", () => {
    expect(PostgresStatementBuilder.createTable("user", userColumns).getText()).toBe(
      'CREATE TABLE "user" ("id" INTEGER PRIMARY KEY, "uuid" UUID, "username" TEXT, "is_enabled" BOOLEAN, "balance" NUMERIC, "location" DOUBLE PRECISION, "created" TIMESTAMPTZ, "version" BYTEA)',
    );
  });

  it("quotes tables in DML and keeps parameter names", () => {
    const where = WhereBuilder.equals("id", 1);
    const fields = [
      { name: "id", value: 1 },
      { name: "username", value: "alice" },
    ];
    expect(PostgresStatementBuilder.createQuery("user", ["id", "username"], where).getText()).toBe(
      'SELECT "id", "username" FROM "user" WHERE id = $id',
    );
    expect(PostgresStatementBuilder.createInsert("user", fields).getText()).toBe(
      'INSERT INTO "user" (id, username) VALUES ($id, $username)',
    );
    expect(PostgresStatementBuilder.createUpdate("user", fields, where).getText()).toBe(
      'UPDATE "user" SET id = $id, username = $username WHERE id = $id',
    );
    expect(PostgresStatementBuilder.createDelete("user", where).getText()).toBe(
      'DELETE FROM "user" WHERE id = $id',
    );
    expect(PostgresStatementBuilder.createStoredProcedure("save", fields).getText()).toBe(
      'CALL "save"($id, $username)',
    );
  });
});
