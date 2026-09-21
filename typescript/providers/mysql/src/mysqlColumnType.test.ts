import { describe, expect, it } from "vitest";
import { CreateTableStatement, DatasourceError } from "@datasources/core";
import { MysqlColumnType } from "./mysqlColumnType.ts";
import { MysqlQuote } from "./mysqlQuote.ts";
import type { MysqlColumn } from "./mysqlColumn.ts";

const userColumns: MysqlColumn[] = [
  { name: "id", type: "int", id: true },
  { name: "uuid", type: "uuid" },
  { name: "username", type: "string" },
  { name: "is_enabled", type: "bool" },
  { name: "balance", type: "decimal" },
  { name: "location", type: "float" },
  { name: "created", type: "datetime" },
  { name: "version", type: "binary" },
];

describe("MysqlColumnType", () => {
  it("maps known and unknown types", () => {
    expect(MysqlColumnType.map("int")).toBe("INT");
    expect(() => MysqlColumnType.map("xml")).toThrow(DatasourceError);
    expect(() => MysqlColumnType.map("xml")).toThrow(/Unknown mysql type "xml"/);
  });

  it("creates the user table", () => {
    expect(
      new CreateTableStatement(
        MysqlQuote.quote("user"),
        userColumns.map((column) => ({
          name: MysqlQuote.quote(column.name),
          type: column.id
            ? `${MysqlColumnType.map(column.type)} PRIMARY KEY`
            : MysqlColumnType.map(column.type),
        })),
      ).getText(),
    ).toBe(
      "CREATE TABLE `user` (`id` INT PRIMARY KEY, `uuid` CHAR(36), `username` TEXT, `is_enabled` TINYINT, `balance` DECIMAL(18,4), `location` DOUBLE, `created` TEXT, `version` BLOB)",
    );
  });
});
