import { ConsoleCommandMiddleware, Datasource, DatasourceFactory } from "@deterministic-code/datasources";
import { MysqlDriver } from "@deterministic-code/datasources-mysql";
import { PostgresDriver } from "@deterministic-code/datasources-postgres";
import { SqliteDriver } from "@deterministic-code/datasources-sqlite";

DatasourceFactory.register("sqlite", new SqliteDriver());
DatasourceFactory.register("mysql", new MysqlDriver());
DatasourceFactory.register("postgres", new PostgresDriver());
import { existsSync, readFileSync } from "node:fs";
import { migrateUp } from "../../../../scripts/migraters.mjs";
import { App } from "./app.ts";
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
    const provider = env("PROVIDER");
    const url = env("DATASOURCE_URL");
    migrateUp({ provider, connection: url, migrationsRoot: env("MIGRATIONS_ROOT") || "sql" });
    await using ds = Datasource.default;
    const connection = await ds.createConnection();
    await connection.open();
    const command = await ds.createCommand(connection);
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
