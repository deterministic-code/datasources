import { describe, expect, it } from "vitest";
import { CreateTableStatement, DatasourceError } from "@datasources/core";
import { SqliteColumnType } from "./sqliteColumnType.ts";
import { SqliteQuote } from "./sqliteQuote.ts";
import type { SqliteColumn } from "./sqliteColumn.ts";

const userColumns: SqliteColumn[] = [
  { name: "id", type: "int", id: true },
  { name: "uuid", type: "uuid" },
  { name: "username", type: "string" },
  { name: "is_enabled", type: "bool" },
  { name: "balance", type: "decimal" },
  { name: "location", type: "float" },
  { name: "created", type: "datetime" },
  { name: "version", type: "binary" },
];

describe("SqliteColumnType", () => {
  it("maps known and unknown types", () => {
    expect(SqliteColumnType.map("int")).toBe("INTEGER");
    expect(() => SqliteColumnType.map("xml")).toThrow(DatasourceError);
    expect(() => SqliteColumnType.map("xml")).toThrow(/Unknown sqlite type "xml"/);
  });

  it("creates the user table", () => {
    expect(
      new CreateTableStatement(
        SqliteQuote.quote("user"),
        userColumns.map((column) => ({
          name: SqliteQuote.quote(column.name),
          type: column.id
            ? `${SqliteColumnType.map(column.type)} PRIMARY KEY`
            : SqliteColumnType.map(column.type),
        })),
      ).getText(),
    ).toBe(
      'CREATE TABLE "user" ("id" INTEGER PRIMARY KEY, "uuid" TEXT, "username" TEXT, "is_enabled" INTEGER, "balance" NUMERIC, "location" REAL, "created" TEXT, "version" BLOB)',
    );
  });
});
