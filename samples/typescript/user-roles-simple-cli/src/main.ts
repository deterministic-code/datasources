import { ConsoleCommandMiddleware, providerFromUrl, type DatasourceOptions, type IConnection } from "@datasources/core";
import "@datasources/register";
import { MysqlDriver } from "@datasources/mysql";
import { PostgresDriver } from "@datasources/postgres";
import { SqliteDriver } from "@datasources/sqlite";
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

const connect = (): { connection: IConnection; driver: SqliteDriver | MysqlDriver | PostgresDriver } => {
  const url = env("DATASOURCE_URL");
  const schema = env("DATABASE_SCHEMA");
  const provider = providerFromUrl(url);
  const options: DatasourceOptions = schema ? { url, schema } : { url };
  const driver = ({
    sqlite: () => new SqliteDriver(),
    mysql: () => new MysqlDriver(),
    postgres: () => new PostgresDriver(),
  }[provider] ?? (() => {
    throw new Error("DATASOURCE_URL is required");
  }))();
  return { connection: driver.connection(options), driver };
};

const run = async (args: string[]): Promise<number> => {
  try {
    const parsed = parse(args);
    const { connection, driver } = connect();
    await connection.open();
    const command = connection.createCommand();
    await migrate(command, driver);
    const frozen = env("CLI_NOW");
    const now = frozen ? () => new Date(frozen) : () => new Date();
    command.addMiddleware(new ConsoleCommandMiddleware((line) => console.error(line), now));
    process.stdout.write(await new App(command, frozen ? now : undefined).run(parsed));
    return 0;
  } catch (error) {
    process.stderr.write(error instanceof Error ? error.message : "Unknown error");
    return 1;
  }
};

process.exit(await run(process.argv.slice(2)));
