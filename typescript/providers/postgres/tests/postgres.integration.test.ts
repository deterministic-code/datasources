import { beforeAll, describe, expect, it } from "vitest";
import {
  ParameterDirection,
  ReturnStatement,
  ExecuteStoredProcedureStatement,
} from "@datasources/core";
import { ensureCompose } from "../../ensure-compose.ts";
import {
  loadDialect,
  runSpec,
  specFiles,
} from "../../sql-spec-runner.ts";
import { PostgresCommand } from "../src/command.ts";
import { PostgresConnection } from "../src/connection.ts";
import { PostgresStatementFactory } from "../src/PostgresStatementFactory.ts";
import type { PgModule } from "../src/i-postgres-driver.ts";

const dialect = loadDialect("postgres", (statement) => new PostgresStatementFactory().transform(statement).getText());
const url = (process.env.POSTGRES_URL ??=
  "postgres://ds:ds@127.0.0.1:5432/datasources");

beforeAll(() => ensureCompose("postgres"), 120_000);

const isolate = async (base: string) => {
  const pg = (await import("pg")) as PgModule;
  const Client = pg.default?.Client ?? pg.Client;
  const schema = `ds_${crypto.randomUUID().replaceAll("-", "").slice(0, 16)}`;
  const admin = new Client({ connectionString: base });
  await admin.connect();
  await admin.query(`CREATE SCHEMA ${schema}`);
  await admin.end();
  return {
    schema,
    cleanup: async () => {
      const drop = new Client({ connectionString: base });
      await drop.connect();
      await drop.query(`DROP SCHEMA ${schema} CASCADE`);
      await drop.end();
    },
  };
};

describe("shared sql specs", () => {
  for (const file of specFiles()) {
    it(file.split("/").pop()!, async () => {
      const isolated = await isolate(url);
      try {
        await runSpec(file, dialect, {
          openSession: async () => {
            const connection = PostgresConnection.from({ url, schema: isolated.schema });
            return { connection, command: new PostgresCommand(connection) };
          },
          closedCommand: () =>
            new PostgresCommand(new PostgresConnection()),
        });
      } finally {
        await isolated.cleanup();
      }
    });
  }
});

describe("return and output parameters", () => {
  it("reads INOUT and function results", async () => {
    const isolated = await isolate(url);
    try {
      const connection = PostgresConnection.from({ url, schema: isolated.schema });
      await connection.open();
      const command = new PostgresCommand(connection);
      await command.execute(`
        CREATE PROCEDURE bump(INOUT result INT)
        LANGUAGE plpgsql
        AS $$
        BEGIN
          result := result + 1;
        END;
        $$
      `);
      await command.execute(`
        CREATE FUNCTION add_one(n INT) RETURNS INT
        LANGUAGE sql
        AS $$ SELECT n + 1 $$
      `);
      const result = {
        name: "result",
        value: 4,
        direction: ParameterDirection.InputOutput,
      };
      const call = await command.executeStatement(
        new ExecuteStoredProcedureStatement("bump", [result]),
        [result],
      );
      expect(call.parameters?.[0]?.value).toBe(5);
      const n = { name: "n", value: 4 };
      const ret = await command.executeStatement(
        new ReturnStatement("add_one", [n]),
        [n],
      );
      expect(ret.result).toBe(5);
    } finally {
      await isolated.cleanup();
    }
  });
});
