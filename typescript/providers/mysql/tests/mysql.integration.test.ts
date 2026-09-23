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
import { MysqlCommand } from "../src/command.ts";
import { MysqlConnection } from "../src/connection.ts";
import type { Mysql2Module } from "../src/i-mysql-engine.ts";

const dialect = loadDialect("mysql");
const url = (process.env.MYSQL_URL ??=
  "mysql://ds:ds@127.0.0.1:3306/datasources");

beforeAll(() => ensureCompose("mysql"), 120_000);

const replaceDatabase = (base: string, name: string): string => {
  const parsed = new URL(base);
  parsed.pathname = `/${name}`;
  return parsed.toString();
};

const isolate = async (base: string) => {
  const mysql = (await import("mysql2/promise")) as Mysql2Module;
  const name = `ds_${crypto.randomUUID().replaceAll("-", "").slice(0, 16)}`;
  const admin = await mysql.createConnection({
    uri: base,
    namedPlaceholders: true,
  });
  await admin.query(`CREATE DATABASE \`${name}\``);
  await admin.end();
  const isolated = replaceDatabase(base, name);
  return {
    url: isolated,
    cleanup: async () => {
      const drop = await mysql.createConnection({
        uri: base,
        namedPlaceholders: true,
      });
      await drop.query(`DROP DATABASE \`${name}\``);
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
            const connection = MysqlConnection.from({ url: isolated.url });
            return { connection, command: new MysqlCommand(connection) };
          },
          closedCommand: () =>
            new MysqlCommand(new MysqlConnection()),
        });
      } finally {
        await isolated.cleanup();
      }
    });
  }
});

describe("return and output parameters", () => {
  it("reads OUT and function results", async () => {
    const isolated = await isolate(url);
    try {
      const connection = MysqlConnection.from({ url: isolated.url });
      await connection.open();
      const command = new MysqlCommand(connection);
      await command.execute(`
        CREATE PROCEDURE bump(IN start_n INT, OUT result INT)
        BEGIN
          SET result = start_n + 1;
        END
      `);
      await command.execute(`
        CREATE FUNCTION add_one(n INT) RETURNS INT DETERMINISTIC
        RETURN n + 1
      `);
      const start = { name: "start_n", value: 4 };
      const result = { name: "result", direction: ParameterDirection.Output };
      const call = await command.execute(
        new ExecuteStoredProcedureStatement("bump", [start, result]).getText(),
        [start, result],
      );
      expect(call.parameters?.[1]?.value).toBe(5);
      const n = { name: "n", value: 4 };
      const ret = await command.execute(
        new ReturnStatement("add_one", [n]).getText(),
        [n],
      );
      expect(ret.result).toBe(5);
    } finally {
      await isolated.cleanup();
    }
  });
});
