import { beforeAll, describe, it } from "vitest";
import { execute } from "@deterministic-code/datasources";
import { ensureCompose } from "../../ensure-compose.ts";
import {
  repositorySpecFiles,
  runRepositorySpec,
} from "../../repository-spec-runner.ts";
import { MysqlCommand } from "../src/command.ts";
import { MysqlConnection } from "../src/connection.ts";
import { MysqlRowTypeConverters } from "../src/converters/MysqlRowTypeConverters.ts";
import type { RepositoryDialect } from "../../repository-spec-runner.ts";

const quote = (id: string): string => `\`${id.replaceAll("`", "``")}\``;

const dialect: RepositoryDialect = { quote };
import type { Mysql2Module } from "../src/i-mysql-engine.ts";

let url = (process.env.MYSQL_URL ??=
  "mysql://ds:ds@127.0.0.1:3306/datasources");

beforeAll(() => {
  ensureCompose("mysql");
  url = process.env.MYSQL_URL ?? url;
}, 120_000);

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
        const connection = MysqlConnection.from({ DATASOURCE_URL: isolated.url });
        await connection.open();
        const command = new MysqlCommand(connection);
        await runRepositorySpec(
          file,
          connection,
          (statement) => execute(command, statement),
          dialect,
          MysqlRowTypeConverters,
        );
      } finally {
        await isolated.cleanup();
      }
    });
  }
});
