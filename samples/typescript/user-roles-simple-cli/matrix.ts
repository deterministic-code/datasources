import { ConsoleCommandMiddleware, Datasource, DatasourceFactory } from "@deterministic-code/datasources";
import { MysqlDriver } from "@deterministic-code/datasources-mysql";
import { PostgresDriver } from "@deterministic-code/datasources-postgres";
import { SqliteDriver } from "@deterministic-code/datasources-sqlite";
import { mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { parse as parseYaml } from "yaml";
import { run } from "./src/main.ts";

const sample = dirname(fileURLToPath(import.meta.url));
const spec = parseYaml(readFileSync(resolve(sample, "../../user-roles-simple-cli-shared/cli.yaml"), "utf8")) as {
  now: string;
  steps: { argv: string[] }[];
};

const log = process.env.E2E_MATRIX_LOG ?? "";
DatasourceFactory.register("sqlite", new SqliteDriver());
DatasourceFactory.register("mysql", new MysqlDriver());
DatasourceFactory.register("postgres", new PostgresDriver());
await using const ds = Datasource.default;
const connection = await ds.createConnection();
const lines: string[] = [];
try {
  await connection.open();
  const command = await ds.createCommand(connection);
  const now = () => new Date(spec.now);
  command.addMiddleware(
    new ConsoleCommandMiddleware((line) => lines.push(line), now, { isVerbose: "true" }),
  );
  for (const step of spec.steps) await run(command, step.argv, () => "hash", false);
} finally {
  if (log) {
    mkdirSync(dirname(log), { recursive: true });
    writeFileSync(log, lines.length ? `${lines.join("\n")}\n` : "");
  }
  await connection.close().catch(() => undefined);
}
