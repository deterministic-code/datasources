import { describe, it } from "vitest";
import {
  repositorySpecFiles,
  runRepositorySpec,
} from "../../repository-spec-runner.ts";
import { SqliteCommand } from "../src/command.ts";
import { SqliteConnection } from "../src/connection.ts";
import { SqliteRowTypeConverters } from "../src/converters/SqliteRowTypeConverters.ts";
import type { RepositoryDialect } from "../../repository-spec-runner.ts";

const quote = (id: string): string => `"${id.replaceAll('"', '""')}"`;

const dialect: RepositoryDialect = { quote };

describe("repository specs", () => {
  for (const file of repositorySpecFiles()) {
    it(file.split("/").pop()!, async () => {
      const connection = SqliteConnection.from({ filename: ":memory:" });
      await connection.open();
      const command = new SqliteCommand(connection);
      await runRepositorySpec(file, connection, (sql) => command.execute(sql), dialect, SqliteRowTypeConverters);
    });
  }
});
