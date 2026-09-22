import { providerFromUrl, type IConnection } from "@datasources/core";
import "@datasources/register";
import { loadMysql2Driver, MysqlConnection } from "@datasources/mysql";
import { loadPgDriver, PostgresConnection } from "@datasources/postgres";
import { loadBetterSqlite3Driver, SqliteConnection } from "@datasources/sqlite";
import { readFileSync, existsSync } from "node:fs";
import { App } from "./app.ts";
import { migrate } from "./migrate.ts";
import { parse } from "./parse.ts";

const env = (key: string): string => {
  const value = process.env[key]?.trim();
  if (value) return value;
  if (!existsSync(".env")) return "";
  for (const line of readFileSync(".env", "utf8").split(/\r?\n/)) {
    const i = line.indexOf("=");
    if (i > 0 && line.slice(0, i).trim() === key) return line.slice(i + 1).trim();
  }
  return "";
};

const connect = async (): Promise<IConnection> => {
  const url = env("DATASOURCE_URL");
  const schema = env("DATABASE_SCHEMA") || undefined;
  const provider = providerFromUrl(url);
  return provider === "sqlite"
    ? new SqliteConnection({
        url,
        driver: await loadBetterSqlite3Driver(url.replace(/^sqlite:(?:\/\/\/|\/\/)?/i, "") || ":memory:"),
      })
    : provider === "mysql"
      ? new MysqlConnection({ url, driver: await loadMysql2Driver(url) })
      : provider === "postgres"
        ? new PostgresConnection({
            url,
            schema,
            driver: await loadPgDriver(url, () => import("pg"), schema),
          })
        : (() => {
            throw new Error("DATASOURCE_URL is required");
          })();
};

const run = async (args: string[]): Promise<number> => {
  try {
    const parsed = parse(args);
    const connection = await connect();
    await connection.open();
    const command = connection.createCommand();
    await migrate(command);
    const frozen = env("CLI_NOW");
    process.stdout.write(await new App(command, frozen ? () => new Date(frozen) : undefined).run(parsed));
    return 0;
  } catch (error) {
    process.stderr.write(error instanceof Error ? error.message : "Unknown error");
    return 1;
  }
};

process.exit(await run(process.argv.slice(2)));
