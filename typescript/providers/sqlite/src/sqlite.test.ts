import { afterEach, describe, expect, it, vi } from "vitest";

vi.mock("better-sqlite3", () => {
  class Database {
    constructor(public filename: string) {}
    prepare() {
      return {
        run: () => ({ changes: 0 }),
        all: () => [],
      };
    }
    exec(): void {}
    close(): void {}
  }
  return { default: Database };
});
import { Datasource, DatasourceFactory, DatasourceError, IsolationLevel, Repository } from "@datasources/core";
import {
  BetterSqliteDriver,
  bindParameters,
  loadBetterSqlite3Driver,
  requireDriver,
  SqliteCommand,
  SqliteConnection,
  type SqliteDatabase,
} from "./index.ts";
import { filenameOf, sqliteFilename } from "./sqlitePath.ts";

class FakeStatement {
  constructor(
    private readonly db: FakeDatabase,
    private readonly sql: string,
  ) {}

  run(params?: Record<string, unknown>) {
    this.db.runs.push({ sql: this.sql, params });
    return { changes: 2, lastInsertRowid: 5 };
  }

  all(params?: Record<string, unknown>) {
    this.db.alls.push({ sql: this.sql, params });
    return this.db.rows;
  }
}

class FakeDatabase implements SqliteDatabase {
  runs: Array<{ sql: string; params?: Record<string, unknown> }> = [];
  alls: Array<{ sql: string; params?: Record<string, unknown> }> = [];
  execs: string[] = [];
  closed = false;
  rows: Record<string, unknown>[] = [{ id: 1 }];

  constructor(public filename: string) {}

  prepare(sql: string) {
    return new FakeStatement(this, sql);
  }

  exec(sql: string): void {
    this.execs.push(sql);
  }

  close(): void {
    this.closed = true;
  }
}

function fakeDriver(filename = ":memory:") {
  let instance: FakeDatabase | undefined;
  const Database = class {
    constructor(name: string) {
      instance = new FakeDatabase(name);
      return instance;
    }
  } as unknown as new (filename: string) => SqliteDatabase;
  return {
    driver: new BetterSqliteDriver(filename, Database),
    db: () => instance,
  };
}

afterEach(() => {
  DatasourceFactory.reset();
  DatasourceFactory.register("sqlite", SqliteConnection.from, SqliteConnection.repository);
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

describe("requireDriver and bindParameters", () => {
  it("requires a driver", () => {
    expect(() => requireDriver(undefined)).toThrow(DatasourceError);
    const { driver } = fakeDriver();
    expect(requireDriver(driver)).toBe(driver);
  });

  it("binds parameters", () => {
    expect(bindParameters()).toEqual({});
    expect(bindParameters(null)).toEqual({});
    expect(bindParameters([{ name: "id", value: 1 }])).toEqual({ id: 1 });
  });
});

describe("BetterSqliteDriver", () => {
  it("wraps a scripted database module", async () => {
    let instance: FakeDatabase | undefined;
    const Database = class {
      constructor(filename: string) {
        instance = new FakeDatabase(filename);
        return instance;
      }
    } as unknown as new (filename: string) => SqliteDatabase;

    const driver = new BetterSqliteDriver("app.db", Database);
    await driver.open("app.db");
    await driver.open("app.db");
    expect(instance?.filename).toBe("app.db");
    expect(await driver.execute("INSERT INTO items (id) VALUES ($id)", { id: 1 }))
      .toEqual({ rowsAffected: 2, result: 5 });
    expect(await driver.execute("   ")).toEqual({ rowsAffected: 0 });
    expect(await driver.query("SELECT * FROM items", { id: 1 })).toEqual([
      { id: 1 },
    ]);
    expect(await driver.query("")).toEqual([]);
    await driver.begin();
    await driver.begin(IsolationLevel.ReadUncommitted);
    await driver.commit();
    await driver.rollback();
    expect(instance?.execs).toEqual([
      "PRAGMA read_uncommitted = 0",
      "BEGIN",
      "PRAGMA read_uncommitted = 1",
      "BEGIN",
      "COMMIT",
      "ROLLBACK",
    ]);
    await driver.close();
    expect(instance?.closed).toBe(true);
    await expect(driver.execute("x")).rejects.toThrow("not open");
  });

  it("loads default and named exports from an importer", async () => {
    const Database = class {
      constructor(public filename: string) {}
      prepare() {
        return {
          run: () => ({ changes: 0 }),
          all: () => [],
        };
      }
      exec(): void {}
      close(): void {}
    };
    const named = await loadBetterSqlite3Driver(
      ":memory:",
      async () => Database as never,
    );
    await named.open(":memory:");
    await named.close();

    const def = await loadBetterSqlite3Driver(":memory:", async () => ({
      default: Database,
    }));
    await def.open(":memory:");
    await def.close();

    const viaDefaultImport = await loadBetterSqlite3Driver(":memory:");
    await viaDefaultImport.open(":memory:");
    await viaDefaultImport.close();
  });

  it("closes before open and omits lastInsertRowid", async () => {
    const Database = class {
      prepare() {
        return {
          run: () => ({ changes: 1 }),
          all: () => [],
        };
      }
      exec(): void {}
      close(): void {}
    } as unknown as new (filename: string) => SqliteDatabase;
    const driver = new BetterSqliteDriver(":memory:", Database);
    await driver.close();
    await driver.open(":memory:");
    expect(await driver.execute("INSERT INTO items DEFAULT VALUES")).toEqual({
      rowsAffected: 1,
      result: undefined,
    });
  });
});

describe("SqliteConnection", () => {
  it("opens, reopens, transacts, and closes", async () => {
    const { driver, db } = fakeDriver("x.db");
    const connection = new SqliteConnection({ filename: "x.db", driver });
    await connection.open();
    await connection.open();
    expect(connection.isOpen).toBe(true);
    const txn = await connection.startTransaction();
    await txn.commit();
    const txn2 = await connection.startTransaction(IsolationLevel.ReadUncommitted);
    expect(txn2.isolation).toBe(IsolationLevel.ReadUncommitted);
    await txn2.abort();
    expect(db()?.execs).toEqual([
      "PRAGMA read_uncommitted = 0",
      "BEGIN",
      "COMMIT",
      "PRAGMA read_uncommitted = 1",
      "BEGIN",
      "ROLLBACK",
    ]);
    expect(txn.isolation).toBe("serializable");
    await connection.close();
    expect(db()?.closed).toBe(true);
    expect(() => connection.ensureOpen()).toThrow(DatasourceError);
    await expect(connection.startTransaction()).rejects.toThrow(DatasourceError);
  });

  it("defaults filename and requires a driver", () => {
    expect(() => new SqliteConnection()).toThrow(DatasourceError);
    const connection = new SqliteConnection({ driver: fakeDriver().driver });
    expect(connection.filename).toBe(":memory:");
  });
});

describe("SqliteCommand and Repository", () => {
  async function setup() {
    const { driver, db } = fakeDriver();
    const connection = new SqliteConnection({ driver });
    await connection.open();
    return {
      driver,
      db,
      connection,
      command: new SqliteCommand(connection),
    };
  }

  it("executes SQL through the driver", async () => {
    const { command, db } = await setup();
    db()!.rows = [{ id: 1, name: "a" }];
    expect(await command.execute("DELETE FROM items", [{ name: "id", value: 1 }]))
      .toEqual({ result: 5, rowsAffected: 2, parameters: [{ name: "id", value: 1 }] });
    expect(await command.execute("")).toEqual({ result: undefined, rowsAffected: 0 });
    db()!.rows = [{ abs: 3 }];
    expect(await command.execute("SELECT abs($x)", [{ name: "x", value: -3 }]))
      .toEqual({ result: 3, rowsAffected: 1, parameters: [{ name: "x", value: -3 }] });
    db()!.rows = [];
    expect(await command.execute("SELECT 1")).toEqual({ result: null, rowsAffected: 0 });
    db()!.rows = [{ id: 1, name: "a" }];
    const reader = await command.executeReader("SELECT * FROM items");
    expect(await reader.next()).toBe(true);
    expect(reader.current).toEqual({ id: 1, name: "a" });
    const empty = await command.executeReader("");
    expect(await empty.next()).toBe(false);
  });
});

describe("Sqlite datasource", () => {
  it("registers and constructs children", async () => {
    const { driver } = fakeDriver();
    expect(() =>
      new Datasource("sqlite", () => new SqliteConnection(), SqliteConnection.repository)
        .createConnection(),
    ).toThrow(DatasourceError);
    const ds = new Datasource(
      "sqlite",
      () => new SqliteConnection({ driver }),
      SqliteConnection.repository,
    );
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
    expect(ds.createReadonlyRepository(connection)).toBeInstanceOf(Repository);
    expect(repo).toBeInstanceOf(Repository);
    expect(
      new Datasource("sqlite", () => new SqliteConnection({ driver }), SqliteConnection.repository)
        .createRepository(),
    ).toBeInstanceOf(Repository);
    expect(Datasource.create({ provider: "sqlite" }).provider).toBe("sqlite");
    expect(() => SqliteConnection.from({ filename: ":memory:" })).toThrow(DatasourceError);
    expect(() =>
      SqliteConnection.from({ database_url: "sqlite:///:memory:" }),
    ).toThrow(DatasourceError);
  });
});
