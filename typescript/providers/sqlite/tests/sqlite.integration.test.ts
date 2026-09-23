import { describe, expect, it } from "vitest";
import { ReturnStatement } from "@datasources/core";
import {
  loadDialect,
  runSpec,
  specFiles,
} from "../../sql-spec-runner.ts";
import { SqliteCommand } from "../src/command.ts";
import { SqliteConnection } from "../src/connection.ts";

const dialect = loadDialect("sqlite");

describe("shared sql specs", () => {
  for (const file of specFiles()) {
    it(file.split("/").pop()!, async () => {
      await runSpec(file, dialect, {
        openSession: async (filename) => {
          const connection = SqliteConnection.from({ filename });
          return { connection, command: new SqliteCommand(connection) };
        },
        closedCommand: () =>
          new SqliteCommand(new SqliteConnection()),
      });
    });
  }
});

describe("return statement", () => {
  it("selects a scalar", async () => {
    const connection = SqliteConnection.from({ filename: ":memory:" });
    await connection.open();
    const command = new SqliteCommand(connection);
    const parameters = [{ name: "x", value: -3 }];
    const response = await command.execute(
      new ReturnStatement("abs", parameters).getText(),
      parameters,
    );
    expect(response.result).toBe(3);
  });
});
