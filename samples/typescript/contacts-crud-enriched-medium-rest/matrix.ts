import { ConsoleCommandMiddleware, type ICommandMiddleware } from "@deterministic-code/datasources";
import { MysqlDriver } from "@deterministic-code/datasources-mysql";
import { PostgresDriver } from "@deterministic-code/datasources-postgres";
import { SqliteDriver } from "@deterministic-code/datasources-sqlite";
import { mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { parse as parseYaml } from "yaml";
import { Api } from "./src/api.ts";
import { clock } from "./src/clock.ts";
import { LegacyContactRepository } from "./generated/typescript/repositories/legacy-contact-repository.ts";
import { ContactRepository } from "./generated/typescript/repositories/contact-repository.ts";

const sample = dirname(fileURLToPath(import.meta.url));
type Scalar = string | number | boolean | null;
type Node = Scalar | Node[] | { [key: string]: Node };

type Expect = { status?: number; body?: Node };
type Step = { method: string; path: string; body?: Node; saveId?: boolean; save?: Record<string, string>; expect?: Expect };
const spec = parseYaml(
  readFileSync(resolve(sample, "../../contacts-crud-enriched-medium-rest-shared/round-trip.yaml"), "utf8"),
) as { now: string; steps: Step[] };

const driver = (provider: string) =>
  ({ sqlite: () => new SqliteDriver(), mysql: () => new MysqlDriver(), postgres: () => new PostgresDriver() })[provider]?.() ??
  (() => {
    throw new Error(provider);
  })();

const substituteText = (text: string, saved: Record<string, string>) =>
  Object.entries(saved)
    .sort(([left], [right]) => right.length - left.length)
    .reduce((current, [name, value]) => current.replaceAll(`$${name}`, value), text);

const substituteNode = (node: Node, saved: Record<string, string>): Node =>
  Array.isArray(node)
    ? node.map((item) => substituteNode(item, saved))
    : node && typeof node === "object"
      ? Object.fromEntries(Object.entries(node).map(([key, value]) => [key, substituteNode(value, saved)]))
      : typeof node === "string" && node.startsWith("$") && saved[node.slice(1)] != null
        ? (Number.isFinite(Number(saved[node.slice(1)])) ? Number(saved[node.slice(1)]) : saved[node.slice(1)])
        : node;

const contains = (actual: Node | null, expected: Node, path: string): void => {
  if (expected === null || typeof expected !== "object") {
    if (actual !== expected) throw new Error(`${path}: expected ${JSON.stringify(expected)}, got ${JSON.stringify(actual)}`);
    return;
  }
  if (Array.isArray(expected)) {
    if (!Array.isArray(actual)) throw new Error(`${path}: expected array, got ${JSON.stringify(actual)}`);
    if (actual.length !== expected.length) throw new Error(`${path}: expected ${expected.length} items, got ${actual.length}`);
    expected.forEach((item, i) => contains(actual[i] ?? null, item, `${path}[${i}]`));
    return;
  }
  if (actual == null || typeof actual !== "object" || Array.isArray(actual))
    throw new Error(`${path}: expected object, got ${JSON.stringify(actual)}`);
  for (const [key, value] of Object.entries(expected)) contains(actual[key] ?? null, value, `${path}.${key}`);
};

const at = (root: Node | null, pointer: string): string => {
  let current: Node | undefined = root ?? undefined;
  for (const part of pointer.split(".")) {
    current = Array.isArray(current)
      ? current[Number(part)]
      : current && typeof current === "object"
        ? current[part]
        : undefined;
  }
  return current == null ? "" : String(current);
};

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
const connection = driver(provider).connection({ DATASOURCE_URL: url });
const lines: string[] = [];
try {
  await connection.open();
  const stamp = new Date(spec.now);
  let n = 0;
  clock.now = () => stamp;
  clock.uuid = () => `00000000-0000-0000-0000-${String(++n).padStart(12, "0")}`;
  const middleware = new ConsoleCommandMiddleware((line) => lines.push(line), () => stamp, { isVerbose: "true" });
  const contacts = new ContactRepository(connection);
  const legacy = new LegacyContactRepository(connection);
  attach(contacts, legacy, middleware);
  const api = new Api(contacts, legacy);
  const saved: Record<string, string> = {};
  for (const step of spec.steps) {
    const path = substituteText(step.path, saved);
    const result = await api.handle(step.method, path, step.body === undefined ? undefined : JSON.stringify(substituteNode(step.body, saved)));
    const payload = result.body ? (JSON.parse(result.body) as Node) : null;
    if (step.expect) {
      if (step.expect.status != null && result.status !== step.expect.status)
        throw new Error(`${step.method} ${path}: expected status ${step.expect.status}, got ${result.status} ${result.body}`);
      if (step.expect.body !== undefined) contains(payload, substituteNode(step.expect.body, saved), `${step.method} ${path}`);
    }
    if (step.saveId && payload && typeof payload === "object" && !Array.isArray(payload) && payload.id != null)
      saved.id = String(payload.id);
    for (const [name, pointer] of Object.entries(step.save ?? {})) saved[name] = at(payload, pointer);
  }
} finally {
  if (log) {
    mkdirSync(dirname(log), { recursive: true });
    writeFileSync(log, lines.length ? `${lines.join("\n")}\n` : "");
  }
  await connection.close().catch(() => undefined);
}
