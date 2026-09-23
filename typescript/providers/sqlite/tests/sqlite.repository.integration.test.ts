import { describe, it } from "vitest";
import {
  repositorySpecFiles,
  runRepositorySpec,
} from "../../repository-spec-runner.ts";
import { CreateTableStatement } from "@datasources/core";
import {
  loadBetterSqlite3Driver,
  SqliteColumnType,
  SqliteCommand,
  SqliteConnection,
  SqliteQuote,
  SqliteRowTypeConverters,
} from "../src/index.ts";
import type { RepositoryDialect } from "../../repository-spec-runner.ts";

const dialect: RepositoryDialect = {
  quote: SqliteQuote.quote,
  createTable: (table, columns) =>
    new CreateTableStatement(
      SqliteQuote.quote(table),
      columns.map((column) => ({
        name: SqliteQuote.quote(column.name),
        type: column.id
          ? `${SqliteColumnType.map(column.type)} PRIMARY KEY`
          : SqliteColumnType.map(column.type),
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
