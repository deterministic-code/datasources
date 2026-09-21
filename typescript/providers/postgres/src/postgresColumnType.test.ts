import { describe, expect, it } from "vitest";
import { CreateTableStatement, DatasourceError } from "@datasources/core";
import { PostgresColumnType } from "./postgresColumnType.ts";
import { PostgresQuote } from "./postgresQuote.ts";
import type { PostgresColumn } from "./postgresColumn.ts";

const userColumns: PostgresColumn[] = [
  { name: "id", type: "int", id: true },
  { name: "uuid", type: "uuid" },
  { name: "username", type: "string" },
  { name: "is_enabled", type: "bool" },
  { name: "balance", type: "decimal" },
  { name: "location", type: "float" },
  { name: "created", type: "datetime" },
  { name: "version", type: "binary" },
];

describe("PostgresColumnType", () => {
  it("maps known and unknown types", () => {
    expect(PostgresColumnType.map("int")).toBe("INTEGER");
    expect(() => PostgresColumnType.map("xml")).toThrow(DatasourceError);
    expect(() => PostgresColumnType.map("xml")).toThrow(/Unknown postgres type "xml"/);
  });

  it("creates the user table", () => {
    expect(
      new CreateTableStatement(
        PostgresQuote.quote("user"),
        userColumns.map((column) => ({
          name: PostgresQuote.quote(column.name),
          type: column.id
            ? `${PostgresColumnType.map(column.type)} PRIMARY KEY`
            : PostgresColumnType.map(column.type),
        })),
      ).getText(),
    ).toBe(
      'CREATE TABLE "user" ("id" INTEGER PRIMARY KEY, "uuid" UUID, "username" TEXT, "is_enabled" BOOLEAN, "balance" NUMERIC, "location" DOUBLE PRECISION, "created" TIMESTAMPTZ, "version" BYTEA)',
    );
  });
});
