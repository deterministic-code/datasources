import { describe, it } from "vitest";
import {
  repositorySpecFiles,
  runRepositorySpec,
} from "../../repository-spec-runner.ts";
import { CreateTableStatement } from "@datasources/core";
import { SqliteCommand } from "../src/command.ts";
import { SqliteConnection } from "../src/connection.ts";
import { SqliteRowTypeConverters } from "../src/converters/SqliteRowTypeConverters.ts";
import { loadBetterSqlite3Driver } from "../src/driver.ts";
import type { RepositoryDialect } from "../../repository-spec-runner.ts";

const quote = (id: string): string => `"${id.replaceAll('"', '""')}"`;

const map = (canonical: string): string => {
  switch (canonical) {
    case "int": return "INTEGER";
    case "uuid":
    case "string":
    case "datetime": return "TEXT";
    case "bool": return "INTEGER";
    case "decimal": return "NUMERIC";
    case "float": return "REAL";
    case "binary": return "BLOB";
    default: throw new Error(`Unknown sqlite type "${canonical}"`);
  }
};

const dialect: RepositoryDialect = {
  quote,
  createTable: (table, columns) =>
    new CreateTableStatement(
      quote(table),
      columns.map((column) => ({
        name: quote(column.name),
        type: column.id ? `${map(column.type)} PRIMARY KEY` : map(column.type),
      })),
    ),
};

describe("repository specs", () => {
  for (const file of repositorySpecFiles()) {
    it(file.split("/").pop()!, async () => {
      const connection = new SqliteConnection({
        filename: ":memory:",
        driver: await loadBetterSqlite3Driver(":memory:"),
      });
      await connection.open();
      const command = new SqliteCommand(connection);
      await runRepositorySpec(file, connection, (sql) => command.execute(sql), dialect, SqliteRowTypeConverters);
    });
  }
});
