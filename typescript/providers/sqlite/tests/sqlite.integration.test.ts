import { describe, it } from "vitest";
import {
  loadDialect,
  runSpec,
  specFiles,
} from "../../sql-spec-runner.ts";
import {
  loadBetterSqlite3Driver,
  SqliteCommand,
  SqliteConnection,
} from "../src/index.ts";

const dialect = loadDialect("sqlite");

describe("shared sql specs", () => {
  for (const file of specFiles()) {
    it(file.split("/").pop()!, async () => {
      await runSpec(file, dialect, {
        openSession: async (filename) => {
          const connection = new SqliteConnection({
            filename,
            driver: await loadBetterSqlite3Driver(filename),
          });
          return { connection, command: new SqliteCommand(connection) };
        },
        closedCommand: () =>
          new SqliteCommand(
            new SqliteConnection({
              driver: {
                open: async () => {},
                close: async () => {},
                execute: async () => ({ rowsAffected: 0 }),
                query: async () => [],
                begin: async () => {},
                commit: async () => {},
                rollback: async () => {},
              },
            }),
          ),
      });
    });
  }
});
