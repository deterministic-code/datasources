import { readdirSync, readFileSync, existsSync } from "node:fs";
import { tmpdir } from "node:os";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import { parse } from "yaml";
import { describe, expect, it } from "vitest";
import {
  DatasourceError,
  IsolationLevel,
  type ITransaction,
  type IsolationLevelName,
  type Parameter,
} from "@datasources/core";
import {
  loadBetterSqlite3Driver,
  SqliteCommand,
  SqliteConnection,
} from "./index.ts";

type Expectation = {
  rowsAffected?: number;
  result?: unknown;
  rows?: Array<Record<string, unknown>>;
  error?: string;
};

type Op = {
  sql: string;
  connection?: string;
  parameters?: Record<string, unknown>;
  open?: boolean;
  expect?: Expectation;
};

type Tx = {
  connection?: string;
  isolation?: IsolationLevelName;
  as?: string;
};

type TxEnd = { transaction?: string };

type Step = {
  execute?: Op;
  executeReader?: Op;
  startTransaction?: Tx;
  commit?: TxEnd;
  abort?: TxEnd;
  close?: unknown;
};

type Spec = {
  name: string;
  filename?: string;
  connections?: string[];
  steps: Step[];
};

const specsDir = () => {
  let dir = dirname(fileURLToPath(import.meta.url));
  for (let i = 0; i < 8; i++) {
    const candidate = join(dir, "specs", "sqlite");
    if (existsSync(candidate)) return candidate;
    dir = dirname(dir);
  }
  throw new Error("specs/sqlite not found");
};

const params = (map?: Record<string, unknown>): Parameter[] =>
  Object.entries(map ?? {}).map(([name, value]) => ({ name, value }));

const norm = (value: unknown): unknown => {
  if (typeof value === "number" && Number.isFinite(value))
    return Number.isInteger(value) ? Math.trunc(value) : value;
  if (typeof value === "bigint") return Number(value);
  return value;
};

const collect = async (reader: { next(): Promise<boolean>; current: unknown }) => {
  const rows: Record<string, unknown>[] = [];
  while (await reader.next()) rows.push(reader.current as Record<string, unknown>);
  return rows;
};

const shared = (filename: string, connections: number) =>
  connections > 1
    ? join(tmpdir(), `ds-${crypto.randomUUID().replaceAll("-", "")}.db`)
    : filename;

const iso = (name?: string): IsolationLevelName =>
  name === IsolationLevel.ReadUncommitted
    || name === IsolationLevel.ReadCommitted
    || name === IsolationLevel.RepeatableRead
    || name === IsolationLevel.Serializable
    ? name
    : IsolationLevel.Serializable;

const run = async (path: string) => {
  const spec = parse(readFileSync(path, "utf8")) as Spec;
  const names = spec.connections?.length ? spec.connections : ["default"];
  const filename = shared(spec.filename ?? ":memory:", names.length);
  const sessions = new Map<string, { connection: SqliteConnection; command: SqliteCommand }>();
  for (const name of names) {
    const driver = await loadBetterSqlite3Driver(filename);
    const connection = new SqliteConnection({ filename, driver });
    await connection.open();
    sessions.set(name, { connection, command: new SqliteCommand(connection) });
  }
  const txs = new Map<string, ITransaction>();
  for (const step of spec.steps) {
    if (step.close !== undefined) {
      await sessions.get(names[0])!.connection.close();
      continue;
    }
    if (step.startTransaction) {
      const session = sessions.get(step.startTransaction.connection ?? names[0])!;
      txs.set(
        step.startTransaction.as ?? "default",
        await session.connection.startTransaction(iso(step.startTransaction.isolation)),
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
    const target =
      op.open === false
        ? new SqliteCommand(new SqliteConnection({ driver: await loadBetterSqlite3Driver(filename) }))
        : sessions.get(op.connection ?? names[0])!.command;
    try {
      if (step.execute) {
        const response = await target.execute(op.sql, params(op.parameters));
        if (op.expect?.error) throw new Error(`${spec.name}: expected ${op.expect.error}`);
        if (op.expect?.rowsAffected !== undefined)
          expect(response.rowsAffected).toBe(op.expect.rowsAffected);
        if (op.expect?.result !== undefined)
          expect(norm(response.result)).toBe(norm(op.expect.result));
      } else {
        const rows = await collect(
          await target.executeReader(op.sql, params(op.parameters)),
        );
        if (op.expect?.error) throw new Error(`${spec.name}: expected ${op.expect.error}`);
        if (op.expect?.rows) {
          expect(rows.map((row) => Object.fromEntries(
            Object.entries(row).map(([key, value]) => [key, norm(value)]),
          ))).toEqual(
            op.expect.rows.map((row) => Object.fromEntries(
              Object.entries(row).map(([key, value]) => [key, norm(value)]),
            )),
          );
        }
      }
    } catch (error) {
      if (!op.expect?.error) throw error;
      const message = error instanceof Error ? error.message : String(error);
      expect(message).toContain(op.expect.error);
      expect(error).toBeInstanceOf(DatasourceError);
    }
  }
};

describe("shared sqlite specs", () => {
  for (const file of readdirSync(specsDir()).filter((name) => name.endsWith(".yaml"))) {
    it(file, async () => run(join(specsDir(), file)));
  }
});
