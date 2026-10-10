import { describe, expect, it } from "vitest";
import { splitStatements, statementUsesParameter, statementReader, StreamingReader } from "./index.ts";

describe("splitStatements", () => {
  it("splits on semicolons outside quotes and comments", () => {
    expect(splitStatements("")).toEqual([]);
    expect(splitStatements("SELECT 1")).toEqual(["SELECT 1"]);
    expect(splitStatements("SELECT 1;")).toEqual(["SELECT 1"]);
    expect(splitStatements("SELECT 1 AS id; SELECT 2 AS id")).toEqual([
      "SELECT 1 AS id",
      "SELECT 2 AS id",
    ]);
    expect(splitStatements("SELECT ';' AS id; SELECT 2 AS id")).toEqual([
      "SELECT ';' AS id",
      "SELECT 2 AS id",
    ]);
    expect(splitStatements('SELECT "a""b" AS id; SELECT `c`')).toEqual([
      'SELECT "a""b" AS id',
      "SELECT `c`",
    ]);
    expect(splitStatements("SELECT 1; -- tail\nSELECT 2")).toEqual([
      "SELECT 1",
      "-- tail\nSELECT 2",
    ]);
    expect(splitStatements("SELECT 1; /* ; */ SELECT 2")).toEqual([
      "SELECT 1",
      "/* ; */ SELECT 2",
    ]);
    const mysql = "CREATE PROCEDURE add_user()\nBEGIN\n  INSERT INTO t VALUES (1);\nEND";
    expect(splitStatements(mysql)).toEqual([mysql]);
    const postgres =
      "CREATE PROCEDURE bulk_create_user(payload jsonb)\nLANGUAGE plpgsql\nAS $$\nBEGIN\n  INSERT INTO t VALUES (1);\nEND;\n$$";
    expect(splitStatements(postgres)).toEqual([postgres]);
  });

  it("matches a parameter token at a word boundary", () => {
    expect(statementUsesParameter("INSERT INTO t (is_enabled) VALUES ($is_enabled__10)", "is_enabled__10")).toBe(true);
    expect(statementUsesParameter("INSERT INTO t (is_enabled) VALUES ($is_enabled__10)", "is_enabled__1")).toBe(false);
    expect(statementUsesParameter("$is_enabled__1", "is_enabled__1")).toBe(true);
  });
});

describe("StreamingReader", () => {
  it("reads the current set and advances on nextResult", async () => {
    const sets = [[{ id: 1 }], [{ id: 2 }]];
    let set = 0;
    let row = 0;
    let reads = 0;
    let closed = false;
    const reader = new StreamingReader(
      async () => {
        reads += 1;
        const rows = sets[set] ?? [];
        if (row >= rows.length) return undefined;
        const current = rows[row];
        row += 1;
        return current;
      },
      async () => {
        set += 1;
        row = 0;
        return set < sets.length;
      },
      async () => {
        closed = true;
      },
    );
    expect(reads).toBe(0);
    expect(await reader.next()).toBe(true);
    expect(reader.current).toEqual({ id: 1 });
    expect(await reader.next()).toBe(false);
    expect(reader.current).toBeUndefined();
    expect(await reader.nextResult()).toBe(true);
    expect(reader.current).toBeUndefined();
    expect(await reader.next()).toBe(true);
    expect(reader.current).toEqual({ id: 2 });
    expect(await reader.nextResult()).toBe(false);
    await reader.dispose();
    await reader.dispose();
    expect(closed).toBe(true);
    const plain = new StreamingReader(
      async () => undefined,
      async () => false,
    );
    expect(await Array.fromAsync(plain.read())).toEqual([]);
    await plain.dispose();
    await plain[Symbol.asyncDispose]();
    await plain.dispose();
  });

  it("loads one statement at a time", async () => {
    const loaded: string[] = [];
    const reader = statementReader(["SELECT 1 AS id", "SELECT 2 AS id"], async function* (sql) {
      loaded.push(sql);
      yield sql.endsWith("1 AS id") ? { id: 1 } : { id: 2 };
    });
    expect(loaded).toEqual([]);
    expect(await reader.next()).toBe(true);
    expect(reader.current).toEqual({ id: 1 });
    expect(await reader.next()).toBe(false);
    expect(loaded).toEqual(["SELECT 1 AS id"]);
    expect(await reader.nextResult()).toBe(true);
    expect(await reader.next()).toBe(true);
    expect(reader.current).toEqual({ id: 2 });
    expect(await reader.nextResult()).toBe(false);
    expect(await reader.next()).toBe(false);
    expect(loaded).toEqual(["SELECT 1 AS id", "SELECT 2 AS id"]);
    await reader.dispose();
  });

  it("returns false when a statement reader has no statements", async () => {
    const reader = statementReader([], async function* () {});
    expect(await reader.next()).toBe(false);
    expect(await reader.nextResult()).toBe(false);
    const blank = statementReader([""], async function* () {
      yield { id: 1 };
    });
    expect(await blank.next()).toBe(false);
  });
});
