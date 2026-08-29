import { describe, it } from "vitest";
import {
  repositorySpecFiles,
  runRepositorySpec,
} from "../../../repository-spec-runner.ts";
import {
  loadBetterSqlite3Driver,
  SqliteCommand,
  SqliteConnection,
} from "./index.ts";

describe("repository specs", () => {
  for (const file of repositorySpecFiles()) {
    it(file.split("/").pop()!, async () => {
      const connection = new SqliteConnection({
        filename: ":memory:",
        driver: await loadBetterSqlite3Driver(":memory:"),
      });
      await connection.open();
      const command = new SqliteCommand(connection);
      await runRepositorySpec(file, connection, (sql) => command.execute(sql));
    });
  }
});
