import { describe, expect, it } from "vitest";
import { DatasourceError, Join, WhereBuilder } from "@datasources/core";
import { SqliteStatementBuilder } from "./sqliteStatementBuilder.ts";

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

describe("SqliteStatementBuilder", () => {
  it("quotes identifiers and maps canonical types", () => {
    expect(SqliteStatementBuilder.quote("user")).toBe('"user"');
    expect(SqliteStatementBuilder.quote('a"b')).toBe('"a""b"');
    expect(SqliteStatementBuilder.columnType("int")).toBe("INTEGER");
    expect(() => SqliteStatementBuilder.columnType("xml")).toThrow(DatasourceError);
    expect(() => SqliteStatementBuilder.columnType("xml")).toThrow(/Unknown sqlite type "xml"/);
  });

  it("creates the user table", () => {
    expect(SqliteStatementBuilder.createTable("user", userColumns).getText()).toBe(
      'CREATE TABLE "user" ("id" INTEGER PRIMARY KEY, "uuid" TEXT, "username" TEXT, "is_enabled" INTEGER, "balance" NUMERIC, "location" REAL, "created" TEXT, "version" BLOB)',
    );
  });

  it("quotes tables in DML and keeps parameter names", () => {
    const where = WhereBuilder.equals("id", 1);
    const fields = [
      { name: "id", value: 1 },
      { name: "username", value: "alice" },
    ];
    expect(SqliteStatementBuilder.createQuery("user", ["id", "username"], where).getText()).toBe(
      'SELECT "id", "username" FROM "user" WHERE id = $id',
    );
    expect(SqliteStatementBuilder.createInsert("user", fields).getText()).toBe(
      'INSERT INTO "user" (id, username) VALUES ($id, $username)',
    );
    expect(SqliteStatementBuilder.createUpdate("user", fields, where).getText()).toBe(
      'UPDATE "user" SET id = $id, username = $username WHERE id = $id',
    );
    expect(SqliteStatementBuilder.createDelete("user", where).getText()).toBe(
      'DELETE FROM "user" WHERE id = $id',
    );
    expect(SqliteStatementBuilder.createStoredProcedure("save", fields).getText()).toBe(
      'CALL "save"($id, $username)',
    );
    expect(
      SqliteStatementBuilder.createQuery(
        "users",
        ["users.id", "orders.total"],
        WhereBuilder.equals("users.id", 1),
        [
          Join.inner("orders", "users.id", "orders.user_id"),
          Join.left("payments", "orders.id", "payments.order_id"),
        ],
      ).getText(),
    ).toBe(
      'SELECT "users"."id", "orders"."total" FROM "users" INNER JOIN "orders" ON "users"."id" = "orders"."user_id" LEFT JOIN "payments" ON "orders"."id" = "payments"."order_id" WHERE users.id = $users.id',
    );
  });
});
