import { afterEach, describe, expect, it } from "vitest";
import {
  CRUDRepository,
  ReadonlyRepository,
  untyped,
  Datasource,
  DatasourceError,
  DatasourceFactory,
  IsolationLevel,
  type Value,
  type Cells,
  columnType,
  emptyDefaultValueConverter,
  type IDriver,
  RowTypeConverter,
  CreateTableStatement,
  InsertStatement,
  Native,
  ReturnStatement,
  SelectStatement,
  type Scalar,
} from "@deterministic-code/datasources";
import { SqliteCommand } from "./command.ts";
import { SqliteConnection } from "./connection.ts";
import { SqliteRowTypeConverters } from "./converters/SqliteRowTypeConverters.ts";
import type { SqliteDatabase, SqliteStatement } from "./i-sqlite-driver.ts";
import {
  filenameOf,
  isMemoryFilename,
  optionsOf,
  parseFilenameQuery,
  resolvedOptions,
  sqliteFilename,
  sqlitePath,
} from "./sqlitePath.ts";
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
  zeroRows = false;
  columns?: Array<{ name: string; column: string | null; table: string | null }>;
  rawRows?: Array<Array<Value | null>>;

  timeout?: number;

  constructor(public filename: string, options?: { timeout?: number }) {
    this.timeout = options?.timeout;
    FakeDatabase.latest = this;
  }

  prepare(sql: string): SqliteStatement {
    return {
      run: (params) => {
        this.runs.push({ sql, params });
        if (this.zeroRows) return { changes: 0 };
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
    expect(optionsOf({ url: "mysql://x" }).filename).toBe(":memory:");
  });

  it("parses connection-string options", () => {
    const defaults = resolvedOptions("app.sqlite", {});
    expect(defaults.journalMode).toBe("wal");
    expect(defaults.busyTimeout).toBe(5000);
    expect(defaults.poolSize).toBe(8);
    const parsed = sqlitePath(
      "sqlite:///./app.sqlite?journal_mode=delete&busy_timeout=1000&pool_size=4&synchronous=normal",
    );
    expect(parsed?.filename).toBe("./app.sqlite");
    expect(parsed?.options).toEqual({
      journalMode: "delete",
      busyTimeout: 1000,
      poolSize: 4,
      synchronous: "normal",
    });
    expect(resolvedOptions("app.sqlite", { journalMode: "delete" }).poolSize).toBe(0);
    expect(resolvedOptions("app.sqlite", { poolSize: 0 }).poolSize).toBe(0);
    expect(resolvedOptions(":memory:", { journalMode: "wal", poolSize: 8 })).toEqual({
      journalMode: undefined,
      busyTimeout: 5000,
      poolSize: 0,
      synchronous: undefined,
    });
    expect(
      sqlitePath("sqlite:///file:x?mode=memory&journal_mode=delete&cache=shared")?.filename,
    ).toBe("file:x?mode=memory&cache=shared");
    expect(sqlitePath("sqlite:///app.db?journal_mode=fast")?.error).toContain("journal_mode=fast");
    expect(parseFilenameQuery("app.db?pool_size=65").error).toContain("pool_size=65");
    expect(parseFilenameQuery("app.db?busy_timeout=-1").error).toContain("busy_timeout=-1");
    expect(parseFilenameQuery("app.db?journal_mode").error).toContain("journal_mode");
    expect(parseFilenameQuery("app.db?synchronous=fast").error).toContain("synchronous=fast");
    expect(parseFilenameQuery("app.db?&journal_mode=wal&synchronous=extra").options).toEqual({
      journalMode: "wal",
      synchronous: "extra",
    });
    expect(isMemoryFilename(":memory")).toBe(true);
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
    expect(db.timeout).toBe(5000);
    expect(db.execs).toEqual([
      "PRAGMA journal_mode = WAL",
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

  it("applies a synchronous pragma", async () => {
    const connection = new SqliteConnection({
      url: "sqlite:///x.db?synchronous=full",
      Database: FakeDatabase,
    });
    await connection.open();
    expect(FakeDatabase.latest!.execs).toEqual([
      "PRAGMA journal_mode = WAL",
      "PRAGMA synchronous = FULL",
    ]);
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
    expect(await command.executeNative(new Native("SELECT $missing"))).toEqual({
      result: null,
      rowsAffected: 2,
    });
    expect(await command.executeNative(new Native(""))).toEqual({ result: null, rowsAffected: 0 });
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
    db.omitRowid = false;
    expect(await command.executeNative(new Native("INSERT INTO items DEFAULT VALUES"))).toEqual({
      result: null,
      rowsAffected: 2,
    });
    expect(await command.executeStatement(new InsertStatement("items", [], "id"))).toEqual({
      result: 5,
      rowsAffected: 2,
    });
    db.zeroRows = true;
    expect(await command.executeStatement(new InsertStatement("items", [], "id"))).toEqual({
      result: null,
      rowsAffected: 0,
    });
    db.zeroRows = false;
    db.rows = [{ id: 1, name: "a" }, { id: 2, name: "b" }];
    const reader = await command.executeReader("SELECT * FROM items");
    expect(await reader.next()).toBe(true);
    expect(reader.current).toEqual({ id: 1, name: "a" });
    expect(await reader.next()).toBe(true);
    expect(reader.current).toEqual({ id: 2, name: "b" });
    await reader.dispose();
    const empty = await command.executeReaderNative(new Native(""));
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
    expect(await command.executeStatement(new SelectStatement({ table: "dual", fields: ["1"] }))).toEqual({
      result: null,
      rowsAffected: 0,
      parameters: undefined,
    });
    db.rows = [{}];
    expect(await command.executeStatement(new SelectStatement({ table: "dual", fields: ["1"] }))).toMatchObject({ result: null });
    db.omitRowid = false;
    expect(await command.executeStatement(new CreateTableStatement("items", [{ name: "id", type: "INTEGER" }]))).toMatchObject({
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
      defaultValueConverter: emptyDefaultValueConverter,
    });
    const ds = new Datasource("sqlite", of());
    expect(ds.provider).toBe("sqlite");
    const connection = await ds.createConnection();
    await (connection as SqliteConnection).open();
    expect(connection.createCommand()).toBeInstanceOf(SqliteCommand);
    const command = await ds.createCommand(connection);
    expect(command).toBeInstanceOf(SqliteCommand);
    expect(command.connection).toBe(connection);
    await command.execute("DELETE FROM items");
    await command.executeReaderNative(new Native("SELECT 1"));
    await expect(ds.createCommand()).rejects.toThrow("Connection is already open");
    const repo = ds.createRepository(untyped(), connection);
    expect(ds.createReadonlyRepository(untyped(), connection).constructor).toBe(ReadonlyRepository);
    expect(repo).toBeInstanceOf(CRUDRepository);
    expect(new Datasource("sqlite", of()).createRepository(untyped())).toBeInstanceOf(CRUDRepository);
    expect(Datasource.create({ PROVIDER: "sqlite" }).provider).toBe("sqlite");
    expect(SqliteConnection.from({ DATASOURCE_URL: "sqlite:///:memory:" }).filename).toBe(":memory:");
    await expect(
      new SqliteConnection({
        url: "sqlite:///app.db?journal_mode=fast",
        Database: FakeDatabase,
      }).open(),
    ).rejects.toThrow("journal_mode=fast");
    expect(SqliteConnection.from().filename).toBe(":memory:");
    const provider = new SqliteDriver();
    expect(provider.columnType("int")).toBe("INTEGER");
    expect(provider.columnType("uuid")).toBe("TEXT");
    expect(provider.columnType("string")).toBe("TEXT");
    expect(provider.columnType("bool")).toBe("BOOLEAN");
    expect(provider.columnType("decimal")).toBe("NUMERIC");
    expect(provider.columnType("float")).toBe("REAL");
    expect(provider.columnType("datetime")).toBe("TEXT");
    expect(provider.columnType("binary")).toBe("BLOB");
    expect(() => provider.columnType("xml")).toThrow(DatasourceError);
    expect(provider.command(connection)).toBeInstanceOf(SqliteCommand);
    expect(provider.transaction(connection as SqliteConnection).isolation).toBe("readCommitted");
  });

  it("maps property targets", () => {
    const provider = new SqliteDriver();
    const created = "2020-01-02T03:04:05.000Z";
    const converter = RowTypeConverter.from(provider.rowTypeConverters, {
      Key: "uuid",
      Active: "boolean",
      Login: "string",
      Created: "datetime",
    });
    const row = converter.convert({
      Key: "11111111-1111-1111-1111-111111111111",
      Active: 1,
      Login: "ada",
      Created: created,
    });
    expect(row).toEqual({
      Key: "11111111-1111-1111-1111-111111111111",
      Active: true,
      Login: "ada",
      Created: new Date(created),
    });
    expect(converter.reverseConvert(row)).toEqual({
      Key: "11111111-1111-1111-1111-111111111111",
      Active: 1,
      Login: "ada",
      Created: created,
    });
  });
});
