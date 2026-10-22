import { ConsoleCommandMiddleware } from "@deterministic-code/datasources";
import { MysqlDriver } from "@deterministic-code/datasources-mysql";
import { PostgresDriver } from "@deterministic-code/datasources-postgres";
import { SqliteDriver } from "@deterministic-code/datasources-sqlite";
import { mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { parse as parseYaml } from "yaml";
import { App } from "./src/app.ts";
import { parse } from "./src/parse.ts";

const sample = dirname(fileURLToPath(import.meta.url));
const spec = parseYaml(readFileSync(resolve(sample, "../../user-roles-simple-cli-shared/cli.yaml"), "utf8")) as {
  now: string;
  steps: { argv: string[] }[];
};

const driver = (provider: string) =>
  ({ sqlite: () => new SqliteDriver(), mysql: () => new MysqlDriver(), postgres: () => new PostgresDriver() })[provider]?.() ??
  (() => {
    throw new Error(provider);
  })();

const provider = process.env.PROVIDER ?? "sqlite";
const url = process.env.DATASOURCE_URL ?? "sqlite:///:memory:";
const log = process.env.E2E_MATRIX_LOG ?? "";
const connection = driver(provider).connection({ DATASOURCE_URL: url });
const lines: string[] = [];
try {
  await connection.open();
  const command = connection.createCommand();
  const now = () => new Date(spec.now);
  command.addMiddleware(
    new ConsoleCommandMiddleware((line) => lines.push(line), now, { isVerbose: "true" }),
  );
  const app = new App(command, now, () => "hash", () => new Uint8Array([1, 2, 3]), false);
  for (const step of spec.steps) await app.run(parse(step.argv));
} finally {
  if (log) {
    mkdirSync(dirname(log), { recursive: true });
    writeFileSync(log, lines.length ? `${lines.join("\n")}\n` : "");
  }
  await connection.close().catch(() => undefined);
}
