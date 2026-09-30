import { readdirSync, readFileSync, existsSync } from "node:fs";
import { tmpdir } from "node:os";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import { parse } from "yaml";
import { expect } from "vitest";
import {
  ConsoleCommandMiddleware,
  CreateTableStatement,
  DatasourceError,
  Native,
  IsolationLevel,
  type ICommand,
  type IConnection,
  type IReader,
  type ITransaction,
  type IsolationLevelName,
  type Parameter,
} from "@datasources/core";
import {
  consoleSignature,
  expectedConsoleSignatures,
} from "../specs/expected-console-output.ts";
import { tables } from "./schema-spec.ts";

export type Scalar = string | number | boolean | bigint | null;
export type Cells = Record<string, Scalar>;
type Expectation = {
  rowsAffected?: number;
  result?: Scalar;
  rows?: Cells[] | string;
  error?: string;
};
type Op = {
  sql: string;
  connection?: string;
  parameters?: Cells;
  open?: boolean;
  expect?: Expectation;
};
type Tx = { connection?: string; isolation?: IsolationLevelName; as?: string };
type TxEnd = { transaction?: string };
type Step = {
  execute?: Op;
  executeReader?: Op;
  startTransaction?: Tx;
  commit?: TxEnd;
  abort?: TxEnd;
  close?: object;
};
type Spec = {
  name: string;
  connections?: string[];
  steps: Step[];
};
export type DialectConnection = {
  kind: string;
  filename?: string;
  shared?: boolean;
  env?: string;
};
export type Dialect = {
  connection: DialectConnection;
  provider: string;
  render: (statement: CreateTableStatement) => string;
  createUserTable: string;
  createUserTableVersioned: string;
  dirtyReadRows: Cells[];
};
type Session = { connection: IConnection; command: ICommand };

const sqlFragments = ["createUserTable", "createUserTableVersioned"] as const;
type SqlFragment = (typeof sqlFragments)[number];
const isSqlFragment = (name: string): name is SqlFragment =>
  (sqlFragments as readonly string[]).includes(name);

export const repoRoot = (from = fileURLToPath(import.meta.url)): string => {
  for (let dir = dirname(from); ; dir = dirname(dir)) {
    if (existsSync(join(dir, "specs", "command-crud.yaml"))) return dir;
    if (dir === dirname(dir)) throw new Error("specs not found");
  }
};

export const specVerbose = (): boolean =>
  process.env.VERBOSE === "1"
  || process.env.VERBOSE === "true"
  || process.argv.includes("--verbose");

const capture = (logs: string[]) => (line: string) => {
  logs.push(line);
  if (specVerbose()) console.log(line);
};

export const specFiles = (root = repoRoot()): string[] =>
  readdirSync(join(root, "specs"))
    .filter((name) => name.endsWith(".yaml") && name !== "schemas.yaml")
    .map((name) => join(root, "specs", name));

export const loadDialect = (engine: string, render: (statement: CreateTableStatement) => string, root = repoRoot()): Dialect => ({
  ...(parse(readFileSync(join(root, "specs", "dialects", `${engine}.yaml`), "utf8")) as Omit<Dialect, "provider" | "render">),
  provider: engine,
  render,
});

export const dialectTarget = (dialect: Dialect, connections: number): string => {
  if (dialect.connection.kind === "url") {
    const env = dialect.connection.env ?? "";
    const url = process.env[env] ?? "";
    if (!url) throw new Error(`${env} is not set`);
    return url;
  }
  return connections > 1
    ? join(tmpdir(), `ds-${crypto.randomUUID().replaceAll("-", "")}.db`)
    : (dialect.connection.filename ?? ":memory:");
};

const quoteIdent = (id: string): string => `"${id.replaceAll('"', '""')}"`;

const resolveSql = (sql: string, dialect: Dialect): string => {
  const key = sql.trim();
  return isSqlFragment(key)
    ? tables(dialect[key], quoteIdent).map((statement) => dialect.render(statement)).join(";\n")
    : sql;
};

const resolveRows = (
  rows: Cells[] | string | undefined,
  dialect: Dialect,
): Cells[] | undefined =>
  rows === undefined
    ? undefined
    : rows === "dirtyReadRows"
      ? dialect.dirtyReadRows
      : typeof rows === "string"
        ? []
        : rows;

const params = (map?: Cells): Parameter[] =>
  Object.entries(map ?? {}).map(([name, value]) => ({ name, value }));

const norm = (value: Scalar): Scalar =>
  typeof value === "bigint"
    ? Number(value)
    : typeof value === "number" && Number.isInteger(value)
      ? Math.trunc(value)
      : value;

const cells = (row: Cells): Cells =>
  Object.fromEntries(Object.entries(row).map(([key, value]) => [key, norm(value)]));

const collect = async (reader: IReader) => {
  const rows: Cells[] = [];
  try {
    while (await reader.next()) if (reader.current) rows.push(reader.current as Cells);
    return rows;
  } finally {
    await reader.dispose();
  }
};

const iso = (name?: IsolationLevelName): IsolationLevelName =>
  name === IsolationLevel.ReadUncommitted
    || name === IsolationLevel.ReadCommitted
    || name === IsolationLevel.RepeatableRead
    || name === IsolationLevel.Serializable
    ? name
    : IsolationLevel.Serializable;

export type SpecHost = {
  openSession: (target: string) => Promise<Session>;
  closedCommand: () => ICommand;
};

export const runSpec = async (
  path: string,
  dialect: Dialect,
  host: SpecHost,
): Promise<void> => {
  const spec = parse(readFileSync(path, "utf8")) as Spec;
  const names = spec.connections?.length ? spec.connections : ["default"];
  const target = dialectTarget(dialect, names.length);
  const logs: string[] = [];
  if (specVerbose()) console.log(`\n--- ${spec.name} (${dialect.connection.kind}) ---`);
  const attach = (command: ICommand) =>
    command.addMiddleware(new ConsoleCommandMiddleware(capture(logs)));
  const sessions = new Map<string, Session>();
  for (const name of names) {
    const session = await host.openSession(target);
    await session.connection.open();
    attach(session.command);
    sessions.set(name, session);
  }
  const session = (name?: string) => sessions.get(name ?? names[0])!;
  const txs = new Map<string, ITransaction>();
  for (const step of spec.steps) {
    if (step.close !== undefined) {
      await session().connection.close();
      continue;
    }
    if (step.startTransaction) {
      const { connection, isolation, as: alias } = step.startTransaction;
      txs.set(
        alias ?? "default",
        await session(connection).connection.startTransaction(iso(isolation)),
      );
      continue;
    }
    if (step.commit) {
      await txs.get(step.commit.transaction ?? "default")!.commit();
      continue;
    }
    if (step.abort) {
      await txs.get(step.abort.transaction ?? "default")!.abort();
      continue;
    }
    const op = step.execute ?? step.executeReader;
    if (!op) throw new Error(`${spec.name}: empty step`);
    op.sql = resolveSql(op.sql, dialect);
    if (op.expect) op.expect.rows = resolveRows(op.expect.rows, dialect);
    const command = op.open === false
      ? attach(host.closedCommand())
      : session(op.connection).command;
    try {
      if (step.execute) {
        const response = await command.executeNative(new Native(op.sql), params(op.parameters));
        if (op.expect?.error) throw new Error(`${spec.name}: expected ${op.expect.error}`);
        if (op.expect?.rowsAffected !== undefined)
          expect(response.rowsAffected).toBe(op.expect.rowsAffected);
        if (op.expect?.result !== undefined)
          expect(norm(response.result as Scalar)).toBe(norm(op.expect.result));
      } else {
        const rows = await collect(await command.executeReaderNative(new Native(op.sql), params(op.parameters)));
        if (op.expect?.error) throw new Error(`${spec.name}: expected ${op.expect.error}`);
        if (op.expect?.rows)
          expect(rows.map(cells)).toEqual((op.expect.rows as Cells[]).map(cells));
      }
    } catch (error) {
      if (!op.expect?.error) throw error;
      expect(error).toBeInstanceOf(DatasourceError);
      expect(error instanceof Error ? error.message : "").toContain(op.expect.error);
    }
  }
  const expected = expectedConsoleSignatures(spec);
  const actual = logs.map(consoleSignature);
  let mismatch = actual.findIndex((line, i) => line !== expected[i]);
  if (mismatch === -1 && actual.length !== expected.length)
    mismatch = Math.min(actual.length, expected.length);
  if (mismatch === -1) return;
  throw new Error(
    `${spec.name}: first logging mismatch at ${mismatch}\nexpected: ${expected[mismatch] ?? "<missing>"}\nactual:   ${actual[mismatch] ?? "<missing>"}\n--- expected ---\n${expected.join("\n")}\n--- actual ---\n${actual.join("\n")}`,
  );
};
