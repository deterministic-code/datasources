import { ConsoleCommandMiddleware, providerFromUrl, type DatasourceOptions, type IConnection } from "@datasources/core";
import "@datasources/register";
import { MysqlProvider } from "@datasources/mysql";
import { PostgresProvider } from "@datasources/postgres";
import { SqliteProvider } from "@datasources/sqlite";
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

const connect = (): IConnection => {
  const url = env("DATASOURCE_URL");
  const schema = env("DATABASE_SCHEMA");
  const provider = providerFromUrl(url);
  const options: DatasourceOptions = schema ? { url, schema } : { url };
  return ({
    sqlite: () => new SqliteProvider().connection(options),
    mysql: () => new MysqlProvider().connection(options),
    postgres: () => new PostgresProvider().connection(options),
  }[provider] ?? (() => {
    throw new Error("DATASOURCE_URL is required");
  }))();
};

const run = async (args: string[]): Promise<number> => {
  try {
    const parsed = parse(args);
    const connection = await connect();
    await connection.open();
    const command = connection.createCommand();
    await migrate(command);
    command.addMiddleware(new ConsoleCommandMiddleware((line) => console.error(line)));
    const frozen = env("CLI_NOW");
    process.stdout.write(await new App(command, frozen ? () => new Date(frozen) : undefined).run(parsed));
    return 0;
  } catch (error) {
    process.stderr.write(error instanceof Error ? error.message : "Unknown error");
    return 1;
  }
};

process.exit(await run(process.argv.slice(2)));
