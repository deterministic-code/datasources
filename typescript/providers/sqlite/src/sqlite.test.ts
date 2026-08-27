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
import {
  collectAsync,
  Datasource,
  DatasourceError,
  Query,
  Where,
} from "@datasources/core";
import {
  BetterSqliteDriver,
  bindParameters,
  loadBetterSqlite3Driver,
  requireDriver,
  SqliteCommand,
  SqliteConnection,
  SqliteDatasource,
  SqliteRepository,
  type ISqliteDriver,
  type SqliteDatabase,
  type SqliteExecuteResult,
} from "./index.ts";

class ScriptedDriver implements ISqliteDriver {
  opened = false;
  closed = false;
  begins = 0;
  commits = 0;
  rollbacks = 0;
  executes: Array<{ sql: string; params?: Record<string, unknown> }> = [];
  queries: Array<{ sql: string; params?: Record<string, unknown> }> = [];
  queryResult: Record<string, unknown>[] = [];
  executeResult: SqliteExecuteResult = { rowsAffected: 1, result: 10 };

  async open(_filename: string): Promise<void> {
    this.opened = true;
  }

  async close(): Promise<void> {
    this.closed = true;
  }

  async execute(
    sql: string,
    params?: Record<string, unknown>,
  ): Promise<SqliteExecuteResult> {
    this.executes.push({ sql, params });
    return this.executeResult;
  }

  async query(
    sql: string,
    params?: Record<string, unknown>,
  ): Promise<Record<string, unknown>[]> {
    this.queries.push({ sql, params });
    return this.queryResult;
  }

  async begin(): Promise<void> {
    this.begins += 1;
  }

  async commit(): Promise<void> {
    this.commits += 1;
  }

  async rollback(): Promise<void> {
    this.rollbacks += 1;
  }
}

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

afterEach(() => {
  Datasource.reset();
  Datasource.use(SqliteDatasource.factory);
});

describe("requireDriver and bindParameters", () => {
  it("requires a driver", () => {
    expect(() => requireDriver(undefined)).toThrow(DatasourceError);
    const driver = new ScriptedDriver();
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
    await driver.commit();
    await driver.rollback();
    expect(instance?.execs).toEqual(["BEGIN", "COMMIT", "ROLLBACK"]);
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
    const driver = new ScriptedDriver();
    const connection = new SqliteConnection({ filename: "x.db", driver });
    await connection.open();
    await connection.open();
    expect(driver.opened).toBe(true);
    const txn = await connection.startTransaction();
    expect(driver.begins).toBe(1);
    await txn.commit();
    expect(driver.commits).toBe(1);
    const txn2 = await connection.startTransaction();
    await txn2.abort();
    expect(driver.rollbacks).toBe(1);
    await connection.close();
    expect(driver.closed).toBe(true);
    expect(() => connection.ensureOpen()).toThrow(DatasourceError);
    await expect(connection.startTransaction()).rejects.toThrow(DatasourceError);
  });

  it("defaults filename and requires a driver", () => {
    expect(() => new SqliteConnection()).toThrow(DatasourceError);
    const connection = new SqliteConnection({ driver: new ScriptedDriver() });
    expect(connection.filename).toBe(":memory:");
  });
});

describe("SqliteCommand and SqliteRepository", () => {
  async function setup() {
    const driver = new ScriptedDriver();
    driver.queryResult = [{ id: 1, name: "a" }];
    const connection = new SqliteConnection({ driver });
    await connection.open();
    return {
      driver,
      connection,
      command: new SqliteCommand(connection),
      repo: new SqliteRepository<{ id?: number; name?: string }>(connection),
    };
  }

  it("executes SQL through the driver", async () => {
    const { command, driver } = await setup();
    expect(await command.execute("DELETE FROM items", [{ name: "id", value: 1 }]))
      .toEqual({ result: 10, rowsAffected: 1 });
    expect(await command.execute()).toEqual({ result: 10, rowsAffected: 1 });
    const reader = await command.executeReader("SELECT * FROM items");
    expect(await reader.next()).toBe(true);
    expect(reader.current).toEqual({ id: 1, name: "a" });
    await command.executeReader();
    expect(driver.queries.at(-1)?.sql).toBe("");
  });

  it("maps repository operations to SQL", async () => {
    const { repo, driver } = await setup();
    expect(await repo.getSingle()).toEqual({ id: 1, name: "a" });
    expect(driver.queries.at(-1)?.sql).toBe("SELECT * FROM items LIMIT 1");
    driver.queryResult = [];
    expect(await repo.getSingle(new Query("id = $id"))).toBeUndefined();

    const withParams = new Query("id = $id") as Query & {
      parameters?: { name: string; value: unknown }[];
    };
    withParams.parameters = [{ name: "id", value: 1 }];
    driver.queryResult = [{ id: 1 }];
    expect(await collectAsync(repo.get(withParams))).toEqual([{ id: 1 }]);
    expect(await collectAsync(repo.get())).toEqual([{ id: 1 }]);

    await repo.add({ id: 2, name: "b" });
    expect(driver.executes.at(-1)?.sql).toContain("INSERT INTO items (id, name)");
    await repo.add({});
    expect(driver.executes.at(-1)?.sql).toBe(
      "INSERT INTO items DEFAULT VALUES",
    );
    await repo.update({ id: 2, name: "B" });
    expect(driver.executes.at(-1)?.sql).toContain("UPDATE items SET");
    await repo.update({}, new Where("id = 1"));
    expect(driver.executes.at(-1)?.sql).toContain("SET id = id");
    await repo.delete({ id: 2 });
    expect(driver.executes.at(-1)?.sql).toBe("DELETE FROM items WHERE id = $id");
    await repo.delete({ id: 2 }, new Where("name = $name"));
    expect(driver.executes.at(-1)?.sql).toContain("WHERE name = $name");
    await expect(repo.add("nope" as never)).rejects.toThrow("plain objects");
    await expect(repo.add(null as never)).rejects.toThrow("plain objects");
  });

  it("uses a custom table name", async () => {
    const driver = new ScriptedDriver();
    const connection = new SqliteConnection({ driver });
    await connection.open();
    const repo = new SqliteRepository(connection, { table: "users" });
    await collectAsync(repo.get());
    expect(driver.queries[0]?.sql).toBe("SELECT * FROM users");
  });
});

describe("SqliteDatasource", () => {
  it("registers and constructs children", async () => {
    const driver = new ScriptedDriver();
    expect(SqliteDatasource.create().provider).toBe("sqlite");
    expect(() => new SqliteDatasource().createConnection()).toThrow(
      DatasourceError,
    );
    const ds = new SqliteDatasource({ driver, table: "people" });
    expect(ds.create({ driver }).provider).toBe("sqlite");
    const connection = await ds.createConnection();
    await (connection as SqliteConnection).open();
    expect(ds.createCommand(connection).connection).toBe(connection);
    const implicit = new SqliteDatasource({ driver }).createCommand();
    expect(implicit.connection).toBeInstanceOf(SqliteConnection);
    const repo = ds.createRepository(connection, { table: "accounts" });
    expect((repo as SqliteRepository).table).toBe("accounts");
    expect((ds.createRepository(connection) as SqliteRepository).table).toBe(
      "people",
    );
    expect(
      (new SqliteDatasource({ driver }).createRepository() as SqliteRepository)
        .table,
    ).toBe("items");
    expect(Datasource.create("sqlite", { driver }).provider).toBe("sqlite");
  });
});
