import { afterEach, describe, expect, it } from "vitest";
import {
  CRUDRepository,
  Datasource,
  DatasourceError,
  DatasourceFactory,
  IsolationLevel,
  type Cells,
  type IDriver,
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
  omitRowid = false;

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
        return this.rows;
      },
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
    await connection.open();
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
    expect(txn.isolation).toBe("serializable");
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
      result: 5,
      rowsAffected: 2,
      parameters: [{ name: "id", value: 1 }],
    });
    expect(await command.execute("")).toEqual({ result: undefined, rowsAffected: 0 });
    db.rows = [{ abs: 3 }];
    expect(await command.execute("SELECT abs($x)", [{ name: "x", value: -3 }])).toEqual({
      result: 3,
      rowsAffected: 1,
      parameters: [{ name: "x", value: -3 }],
    });
    db.rows = [];
    expect(await command.execute("SELECT 1")).toEqual({ result: null, rowsAffected: 0 });
    db.omitRowid = true;
    expect(await command.execute("INSERT INTO items DEFAULT VALUES")).toEqual({
      result: undefined,
      rowsAffected: 1,
    });
    db.rows = [{ id: 1, name: "a" }];
    const reader = await command.executeReader("SELECT * FROM items");
    expect(await reader.next()).toBe(true);
    expect(reader.current).toEqual({ id: 1, name: "a" });
    const empty = await command.executeReader("");
    expect(await empty.next()).toBe(false);
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
      transaction: (c, isolation = "serializable") =>
        new SqliteTransaction(c as SqliteConnection, isolation),
      rowTypeConverters: SqliteRowTypeConverters,
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
    await command.executeReader("SELECT 1");
    const implicit = ds.createCommand();
    expect(implicit).toBeInstanceOf(SqliteCommand);
    expect(implicit.connection).toBeInstanceOf(SqliteConnection);
    await implicit.connection.open();
    await implicit.execute("DELETE FROM items");
    await implicit.executeReader("SELECT 1");
    const repo = ds.createRepository(connection);
    expect(ds.createReadonlyRepository(connection)).toBeInstanceOf(CRUDRepository);
    expect(repo).toBeInstanceOf(CRUDRepository);
    expect(new Datasource("sqlite", of()).createRepository()).toBeInstanceOf(CRUDRepository);
    expect(Datasource.create({ provider: "sqlite" }).provider).toBe("sqlite");
    expect(SqliteConnection.from({ filename: ":memory:" }).filename).toBe(":memory:");
    expect(SqliteConnection.from({ url: "sqlite:///:memory:" }).filename).toBe(":memory:");
    expect(SqliteConnection.from({ database_url: "sqlite:///:memory:" }).filename).toBe(":memory:");
    expect(SqliteConnection.from().filename).toBe(":memory:");
    const provider = new SqliteDriver();
    expect(provider.command(connection)).toBeInstanceOf(SqliteCommand);
    expect(provider.transaction(connection as SqliteConnection).isolation).toBe("serializable");
  });
});
