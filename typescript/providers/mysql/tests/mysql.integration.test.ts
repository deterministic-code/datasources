import { beforeAll, describe, it } from "vitest";
import { ensureCompose } from "../../ensure-compose.ts";
import {
  loadDialect,
  runSpec,
  specFiles,
} from "../../sql-spec-runner.ts";
import {
  loadMysql2Driver,
  MysqlCommand,
  MysqlConnection,
} from "../src/index.ts";
import type { Mysql2Module } from "../src/i-mysql-driver.ts";

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
            const connection = new MysqlConnection({
              url: isolated.url,
              driver: await loadMysql2Driver(isolated.url),
            });
            return { connection, command: new MysqlCommand(connection) };
          },
          closedCommand: () =>
            new MysqlCommand(
              new MysqlConnection({
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
      } finally {
        await isolated.cleanup();
      }
    });
  }
});
