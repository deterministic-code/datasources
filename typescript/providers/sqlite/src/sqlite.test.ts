import { afterEach, describe, expect, it } from "vitest";
import {
  CRUDRepository,
  ReadonlyRepository,
  untyped,
  Datasource,
  DatasourceError,
  DatasourceFactory,
  IsolationLevel,
  type Cell,
  type Cells,
  columnType,
  type IDriver,
  CreateTableStatement,
  Native,
  ReturnStatement,
  SelectStatement,
  type Scalar,
} from "@datasources/core";
import { SqliteCommand } from "./command.ts";
import { SqliteConnection } from "./connection.ts";
import { SqliteRowTypeConverters } from "./converters/SqliteRowTypeConverters.ts";
import type { SqliteDatabase, SqliteStatement } from "./i-sqlite-driver.ts";
import { filenameOf, sqliteFilename } from "./sqlitePath.ts";
import { SqliteDriver } from "./SqliteDriver.ts";
import { SqliteTransaction } from "./transaction.ts";

class FakeDatabase implements SqliteDatabase {
  static latest: FakeDatabase | undefined;
  runs: Array<{ sql: string; params?: Record<string, Scalar> }> = [];
  alls: Array<{ sql: string; params?: Record<string, Scalar> }> = [];
  execs: string[] = [];
  closed = false;
  rows: Cells[] = [{ id: 1 }];
  bySql?: Record<string, Cells[]>;
  omitRowid = false;
  columns?: Array<{ name: string; column: string | null; table: string | null }>;
  rawRows?: Array<Array<Cell | null>>;

  constructor(public filename: string) {
    FakeDatabase.latest = this;
  }

  prepare(sql: string): SqliteStatement {
    return {
      run: (params) => {
        this.runs.push({ sql, params });
        return this.omitRowid ? { changes: 1 } : { changes: 2, lastInsertRowid: 5 };
      },
      all: (params) => {
        this.alls.push({ sql, params });
        return this.bySql?.[sql] ?? this.rows;
      },
      iterate: (params) => {
        this.alls.push({ sql, params });
        return (this.bySql?.[sql] ?? this.rows)[Symbol.iterator]();
      },
      columns: this.columns ? () => this.columns! : undefined,
      raw: this.rawRows
        ? () => ({
            iterate: () => this.rawRows![Symbol.iterator](),
          })
        : undefined,
    };
  }

  exec(sql: string): void {
    this.execs.push(sql);
  }

  close(): void {
    this.closed = true;
  }
}

const opened = async (filename = ":memory:"): Promise<SqliteConnection> => {
  const connection = new SqliteConnection({ filename, Database: FakeDatabase });
  await connection.open();
  return connection;
};

afterEach(() => {
  DatasourceFactory.reset();
  DatasourceFactory.register("sqlite", new SqliteDriver());
});

describe("sqlitePath", () => {
  it("parses sqlite URLs and filenames", () => {
    expect(sqliteFilename("mysql://x")).toBeUndefined();
    expect(sqliteFilename("sqlite:")).toBe(":memory:");
    expect(sqliteFilename("sqlite:///:memory")).toBe(":memory:");
    expect(sqliteFilename("sqlite:///:memory:")).toBe(":memory:");
    expect(sqliteFilename("sqlite:///tmp/db.sqlite")).toBe("tmp/db.sqlite");
    expect(filenameOf({})).toBe(":memory:");
    expect(filenameOf({ filename: "x.db" })).toBe("x.db");
    expect(filenameOf({ url: "sqlite:///tmp/db.sqlite" })).toBe("tmp/db.sqlite");
    expect(filenameOf({ url: "mysql://x" })).toBe(":memory:");
  });
});

describe("SqliteConnection", () => {
  it("opens, reopens, transacts, and closes", async () => {
    const connection = await opened("x.db");
    const db = FakeDatabase.latest!;
    await expect(connection.open()).rejects.toThrow("Connection is already open");
    expect(connection.isOpen).toBe(true);
    expect(db.filename).toBe("x.db");
    const txn = await connection.startTransaction();
    await txn.commit();
    const txn2 = await connection.startTransaction(IsolationLevel.ReadUncommitted);
    expect(txn2.isolation).toBe(IsolationLevel.ReadUncommitted);
    await txn2.abort();
    expect(db.execs).toEqual([
      "PRAGMA read_uncommitted = 0",
      "BEGIN",
      "COMMIT",
      "PRAGMA read_uncommitted = 1",
      "BEGIN",
      "ROLLBACK",
    ]);
    expect(txn.isolation).toBe("readCommitted");
    await connection.close();
    expect(db.closed).toBe(true);
    expect(() => connection.ensureOpen()).toThrow(DatasourceError);
    expect(() => connection.require()).toThrow("not open");
    await expect(connection.startTransaction()).rejects.toThrow(DatasourceError);
    await connection.close();
  });

  it("defaults the filename and requires a database module to open", async () => {
    const connection = new SqliteConnection();
    expect(connection.filename).toBe(":memory:");
    await expect(connection.open()).rejects.toThrow("SQLite driver is required");
    await connection.close();
  });
});

describe("SqliteCommand", () => {
  it("executes SQL on the open database", async () => {
    const connection = await opened();
    const db = FakeDatabase.latest!;
    const command = new SqliteCommand(connection);
    db.rows = [{ id: 1, name: "a" }];
    expect(await command.execute("DELETE FROM items", [{ name: "id", value: 1 }])).toEqual({
      result: null,
      rowsAffected: 2,
      parameters: [{ name: "id", value: 1 }],
    });
    expect(await command.execute("")).toEqual({ result: undefined, rowsAffected: 0 });
    db.rows = [{ abs: 3 }];
    const x = { name: "x", value: -3 };
    expect(await command.executeStatement(new ReturnStatement("abs", [x]), [x])).toEqual({
      result: 3,
      rowsAffected: 1,
      parameters: [x],
    });
    db.rows = [];
    expect(await command.executeNative(new Native("SELECT 1"))).toEqual({ result: null, rowsAffected: 2 });
    db.omitRowid = true;
    expect(await command.executeNative(new Native("INSERT INTO items DEFAULT VALUES"))).toEqual({
      result: null,
      rowsAffected: 1,
    });
    db.rows = [{ id: 1, name: "a" }, { id: 2, name: "b" }];
    const reader = await command.executeReader("SELECT * FROM items");
    expect(await reader.next()).toBe(true);
    expect(reader.current).toEqual({ id: 1, name: "a" });
    expect(await reader.next()).toBe(true);
    expect(reader.current).toEqual({ id: 2, name: "b" });
    await reader.dispose();
    const empty = await command.executeReader("");
    expect(await empty.next()).toBe(false);
    expect(await (await command.executeReaderNative(new Native(";"))).next()).toBe(false);
    db.bySql = { "SELECT 1 AS id": [{ id: 1 }], "SELECT 2 AS id": [{ id: 2 }] };
    const sets = await command.executeReaderNative(new Native("SELECT 1 AS id; SELECT 2 AS id"));
    const first: Cells[] = [];
    while (await sets.next()) first.push(sets.current!);
    const moved = await sets.nextResult();
    const second: Cells[] = [];
    while (await sets.next()) second.push(sets.current!);
    expect(moved).toBe(true);
    expect(first).toEqual([{ id: 1 }]);
    expect(second).toEqual([{ id: 2 }]);
    expect(await sets.nextResult()).toBe(false);
    await sets.dispose();
    db.bySql = undefined;
    db.rows = [];
    expect(await command.executeStatement(new SelectStatement("SELECT 1"))).toEqual({
      result: null,
      rowsAffected: 0,
      parameters: undefined,
    });
    db.rows = [{}];
    expect(await command.executeStatement(new SelectStatement("SELECT 1"))).toMatchObject({ result: null });
    db.omitRowid = false;
    expect(await command.executeStatement(new CreateTableStatement("items"))).toMatchObject({
      result: null,
      rowsAffected: 2,
    });
    db.columns = [
      { name: "id", column: "id", table: "a" },
      { name: "id", column: null, table: null },
    ];
    db.rawRows = [[1, null]];
    const qualified = await command.executeReaderNative(new Native("SELECT a.id, id"));
    expect(await qualified.next()).toBe(true);
    expect(qualified.current).toEqual({ "a.id": 1, id: null });
    expect(await qualified.next()).toBe(false);
    await qualified.dispose();
    db.columns = [{ name: "id", column: "id", table: "t" }];
    db.rawRows = undefined;
    db.rows = [{ id: 4 }];
    const plain = await command.executeReaderNative(new Native("SELECT id"));
    expect(await plain.next()).toBe(true);
    expect(plain.current).toEqual({ id: 4 });
    await plain.dispose();
    await connection.close();
    await expect(command.execute("DELETE FROM items")).rejects.toThrow("not open");
  });
});

describe("Sqlite datasource", () => {
  it("registers and constructs children", async () => {
    expect(await new Datasource("sqlite", new SqliteDriver()).createConnection()).toBeInstanceOf(
      SqliteConnection,
    );
    const of = (): IDriver => ({
      connection: () => new SqliteConnection({ Database: FakeDatabase }),
      command: (c) => c.createCommand(),
      transaction: (c, isolation = "readCommitted") =>
        new SqliteTransaction(c as SqliteConnection, isolation),
      rowTypeConverters: SqliteRowTypeConverters,
      columnType,
    });
    const ds = new Datasource("sqlite", of());
    expect(ds.provider).toBe("sqlite");
    const connection = await ds.createConnection();
    await (connection as SqliteConnection).open();
    expect(connection.createCommand()).toBeInstanceOf(SqliteCommand);
    const command = ds.createCommand(connection);
    expect(command).toBeInstanceOf(SqliteCommand);
    expect(command.connection).toBe(connection);
    await command.execute("DELETE FROM items");
    await command.executeReaderNative(new Native("SELECT 1"));
    const implicit = ds.createCommand();
    expect(implicit).toBeInstanceOf(SqliteCommand);
    expect(implicit.connection).toBeInstanceOf(SqliteConnection);
    await implicit.execute("DELETE FROM items");
    await implicit.executeReaderNative(new Native("SELECT 1"));
    const repo = ds.createRepository(untyped(), connection);
    expect(ds.createReadonlyRepository(untyped(), connection).constructor).toBe(ReadonlyRepository);
    expect(repo).toBeInstanceOf(CRUDRepository);
    expect(new Datasource("sqlite", of()).createRepository(untyped())).toBeInstanceOf(CRUDRepository);
    expect(Datasource.create({ PROVIDER: "sqlite" }).provider).toBe("sqlite");
    expect(SqliteConnection.from({ DATASOURCE_URL: "sqlite:///:memory:" }).filename).toBe(":memory:");
    expect(SqliteConnection.from().filename).toBe(":memory:");
    const provider = new SqliteDriver();
    expect(provider.columnType("int")).toBe("INTEGER");
    expect(provider.columnType("uuid")).toBe("TEXT");
    expect(provider.columnType("string")).toBe("TEXT");
    expect(provider.columnType("bool")).toBe("INTEGER");
    expect(provider.columnType("decimal")).toBe("NUMERIC");
    expect(provider.columnType("float")).toBe("REAL");
    expect(provider.columnType("datetime")).toBe("TEXT");
    expect(provider.columnType("binary")).toBe("BLOB");
    expect(() => provider.columnType("xml")).toThrow(DatasourceError);
    expect(provider.command(connection)).toBeInstanceOf(SqliteCommand);
    expect(provider.transaction(connection as SqliteConnection).isolation).toBe("readCommitted");
  });
});
