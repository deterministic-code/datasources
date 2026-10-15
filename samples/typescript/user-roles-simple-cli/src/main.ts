import { ConsoleCommandMiddleware, Datasource, DatasourceFactory } from "@datasources/core";
import { MysqlDriver } from "@datasources/mysql";
import { PostgresDriver } from "@datasources/postgres";
import { SqliteDriver } from "@datasources/sqlite";

DatasourceFactory.register("sqlite", new SqliteDriver());
DatasourceFactory.register("mysql", new MysqlDriver());
DatasourceFactory.register("postgres", new PostgresDriver());
import { existsSync, readFileSync } from "node:fs";
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

const run = async (args: string[]): Promise<number> => {
  try {
    const parsed = parse(args);
    const ds = Datasource.default;
    const connection = await ds.createConnection();
    await connection.open();
    const command = await ds.createCommand(connection);
    await migrate(command);
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
