import { beforeAll, describe, it } from "vitest";
import { ensureCompose } from "../../ensure-compose.ts";
import {
  repositorySpecFiles,
  runRepositorySpec,
} from "../../repository-spec-runner.ts";
import { CreateTableStatement } from "@datasources/core";
import {
  loadMysql2Driver,
  MysqlColumnType,
  MysqlCommand,
  MysqlConnection,
  MysqlQuote,
} from "../src/index.ts";
import type { RepositoryDialect } from "../../repository-spec-runner.ts";

const dialect: RepositoryDialect = {
  quote: MysqlQuote.quote,
  createTable: (table, columns) =>
    new CreateTableStatement(
      MysqlQuote.quote(table),
      columns.map((column) => ({
        name: MysqlQuote.quote(column.name),
        type: column.id
          ? `${MysqlColumnType.map(column.type)} PRIMARY KEY`
          : MysqlColumnType.map(column.type),
      })),
    ),
};
import type { Mysql2Module } from "../src/i-mysql-driver.ts";

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

describe("repository specs", () => {
  for (const file of repositorySpecFiles()) {
    it(file.split("/").pop()!, async () => {
      const isolated = await isolate(url);
      try {
        const connection = new MysqlConnection({
          url: isolated.url,
          driver: await loadMysql2Driver(isolated.url),
        });
        await connection.open();
        const command = new MysqlCommand(connection);
        await runRepositorySpec(
          file,
          connection,
          (sql) => command.execute(sql),
          dialect,
        );
      } finally {
        await isolated.cleanup();
      }
    });
  }
});
