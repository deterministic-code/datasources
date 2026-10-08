import { ConsoleCommandMiddleware, Native, type ICommandMiddleware } from "@datasources/core";
import { MysqlDriver } from "@datasources/mysql";
import { PostgresDriver } from "@datasources/postgres";
import { SqliteDriver } from "@datasources/sqlite";
import { mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { parse as parseYaml } from "yaml";
import { Api } from "./src/api.ts";
import { clock } from "./src/clock.ts";
import { LegacyContactRepository } from "./generated/typescript/repositories/legacy-contact-repository.ts";
import { ContactRepository } from "./generated/typescript/repositories/contact-repository.ts";
import { statements } from "./src/schema.ts";

const sample = dirname(fileURLToPath(import.meta.url));
const spec = parseYaml(
  readFileSync(resolve(sample, "../../contacts-crud-enriched-medium-rest-shared/round-trip.yaml"), "utf8"),
) as { now: string; steps: { method: string; path: string; body?: object; saveId?: boolean }[] };

const driver = (provider: string) =>
  ({ sqlite: () => new SqliteDriver(), mysql: () => new MysqlDriver(), postgres: () => new PostgresDriver() })[provider]?.() ??
  (() => {
    throw new Error(provider);
  })();

const attach = (contacts: ContactRepository, legacy: LegacyContactRepository, middleware: ICommandMiddleware) => {
  contacts.addMiddleware(middleware);
  contacts.address.addMiddleware(middleware);
  contacts.phone.addMiddleware(middleware);
  contacts.contactSource.addMiddleware(middleware);
  contacts.contactSettings.addMiddleware(middleware);
  contacts.contactPreferences.addMiddleware(middleware);
  contacts.address.addressType.addMiddleware(middleware);
  contacts.phone.phoneType.addMiddleware(middleware);
  legacy.addMiddleware(middleware);
};

const provider = process.env.PROVIDER ?? "sqlite";
const url = process.env.DATASOURCE_URL ?? "sqlite:///:memory:";
const log = process.env.E2E_MATRIX_LOG ?? "";
const sqlFile = process.env.E2E_MATRIX_SQL;
const connection = driver(provider).connection({ DATASOURCE_URL: url });
const lines: string[] = [];
try {
  await connection.open();
  const command = connection.createCommand();
  if (provider === "sqlite") await command.executeNative(new Native("PRAGMA foreign_keys = ON"));
  for (const sql of sqlFile ? statements(readFileSync(sqlFile, "utf8")) : statements())
    await command.executeNative(new Native(sql));
  const stamp = new Date(spec.now);
  let n = 0;
  clock.now = () => stamp;
  clock.uuid = () => `00000000-0000-0000-0000-${String(++n).padStart(12, "0")}`;
  const middleware = new ConsoleCommandMiddleware((line) => lines.push(line), () => stamp, { isVerbose: "true" });
  const contacts = new ContactRepository(connection);
  const legacy = new LegacyContactRepository(connection);
  attach(contacts, legacy, middleware);
  const api = new Api(contacts, legacy);
  let id = "";
  for (const step of spec.steps) {
    const path = step.path.replaceAll("$id", id);
    const result = await api.handle(step.method, path, step.body === undefined ? undefined : JSON.stringify(step.body));
    if (step.saveId) id = String((JSON.parse(result.body) as { id: number }).id);
  }
} finally {
  if (log) {
    mkdirSync(dirname(log), { recursive: true });
    writeFileSync(log, lines.length ? `${lines.join("\n")}\n` : "");
  }
  await connection.close().catch(() => undefined);
}
