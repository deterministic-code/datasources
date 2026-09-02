import { afterEach, describe, expect, it, vi } from "vitest";

vi.mock("pg", () => {
  class Client {
    async connect(): Promise<void> {}
    async query() {
      return { rowCount: 0, rows: [] };
    }
    async end(): Promise<void> {}
  }
  return { Client, default: { Client } };
});

import { Datasource, DatasourceError, IsolationLevel } from "@datasources/core";
import {
  bindParameters,
  loadPgDriver,
  PgDriver,
  PostgresCommand,
  PostgresConnection,
  PostgresDatasource,
  PostgresRepository,
  requireDriver,
  toPostgres,
  withReturning,
  type IPgClient,
  type PgQueryResult,
} from "./index.ts";
import type { Scalar } from "./scalar.ts";

class FakeClient implements IPgClient {
  queries: Array<{ text: string; values?: Scalar[] }> = [];
  connected = false;
  ended = false;
  result: PgQueryResult = { rowCount: 2, rows: [{ id: 5 }] };

  async connect(): Promise<void> {
    this.connected = true;
  }
  async query(text: string, values?: Scalar[]): Promise<PgQueryResult> {
    this.queries.push({ text, values });
    return this.result;
  }
  async end(): Promise<void> {
    this.ended = true;
  }
}

function fakeDriver(url = "postgres://localhost/app", schema?: string) {
  let instance: FakeClient | undefined;
  const Client = class {
    constructor() {
      instance = new FakeClient();
      return instance;
    }
  } as unknown as new (options: { connectionString: string }) => IPgClient;
  return {
    driver: new PgDriver(url, Client, schema),
    client: () => instance,
  };
}

afterEach(() => {
  Datasource.reset();
  Datasource.use(PostgresDatasource.instance);
});

describe("requireDriver and bind", () => {
  it("requires a driver", () => {
    expect(() => requireDriver(undefined)).toThrow(DatasourceError);
    const { driver } = fakeDriver();
    expect(requireDriver(driver)).toBe(driver);
  });

  it("rewrites named parameters and inserts", () => {
    expect(bindParameters()).toEqual({});
    expect(bindParameters([{ name: "id", value: 1 }])).toEqual({ id: 1 });
    expect(toPostgres("SELECT * FROM t WHERE id = $id AND n = $id", { id: 1 }))
      .toEqual({ text: "SELECT * FROM t WHERE id = $1 AND n = $1", values: [1] });
    expect(toPostgres("SELECT 1")).toEqual({ text: "SELECT 1", values: [] });
    expect(toPostgres("SELECT * FROM t WHERE id = $id")).toEqual({
      text: "SELECT * FROM t WHERE id = $1",
      values: [null],
    });
    expect(withReturning("INSERT INTO t (id) VALUES (1)")).toBe(
      "INSERT INTO t (id) VALUES (1) RETURNING id",
    );
    expect(withReturning("INSERT INTO t VALUES (1) RETURNING id")).toBe(
      "INSERT INTO t VALUES (1) RETURNING id",
    );
    expect(withReturning("UPDATE t SET x = 1")).toBe("UPDATE t SET x = 1");
  });
});

describe("PgDriver", () => {
  it("wraps a scripted client", async () => {
    const { driver, client } = fakeDriver("postgres://db/app", "iso");
    await driver.open("postgres://db/app");
    await driver.open("postgres://db/app");
    expect(client()?.connected).toBe(true);
    expect(await driver.execute("INSERT INTO items (id) VALUES ($id)", { id: 1 }))
      .toEqual({ rowsAffected: 2, result: 5 });
    client()!.result = { rowCount: 0, rows: [] };
    expect(await driver.execute("INSERT INTO items (id) VALUES (1)")).toEqual({
      rowsAffected: 0,
      result: undefined,
    });
    client()!.result = { rowCount: null, rows: [{ id: 9 }] };
    expect(await driver.execute("UPDATE items SET n = 1")).toEqual({
      rowsAffected: 1,
      result: 9,
    });
    client()!.result = { rowCount: 2, rows: [{ id: 5 }] };
    expect(await driver.execute("   ")).toEqual({ rowsAffected: 0 });
    client()!.result = { rowCount: null, rows: [{ id: 1 }] };
    expect(await driver.query("SELECT * FROM items", { id: 1 })).toEqual([
      { id: 1 },
    ]);
    expect(await driver.query("")).toEqual([]);
    await driver.begin();
    await driver.begin(IsolationLevel.ReadUncommitted);
    await driver.begin(IsolationLevel.ReadCommitted);
    await driver.begin(IsolationLevel.RepeatableRead);
    await driver.commit();
    await driver.rollback();
    expect(client()?.queries.map((q) => q.text)).toEqual([
      "SET search_path TO iso",
      "INSERT INTO items (id) VALUES ($1) RETURNING id",
      "INSERT INTO items (id) VALUES (1) RETURNING id",
      "UPDATE items SET n = 1",
      "SELECT * FROM items",
      "BEGIN ISOLATION LEVEL SERIALIZABLE",
      "BEGIN ISOLATION LEVEL READ UNCOMMITTED",
      "BEGIN ISOLATION LEVEL READ COMMITTED",
      "BEGIN ISOLATION LEVEL REPEATABLE READ",
      "COMMIT",
      "ROLLBACK",
    ]);
    await driver.close();
    expect(client()?.ended).toBe(true);
    await expect(driver.execute("x")).rejects.toThrow("not open");
  });

  it("loads default and named exports", async () => {
    const Client = class {
      async connect(): Promise<void> {}
      async query() {
        return { rowCount: 0, rows: [] };
      }
      async end(): Promise<void> {}
    } as unknown as new (options: { connectionString: string }) => IPgClient;
    const named = await loadPgDriver("postgres://db/app", async () => ({
      Client,
    }));
    await named.open("postgres://db/app");
    await named.close();
    const def = await loadPgDriver("postgres://db/app", async () => ({
      Client,
      default: { Client },
    }));
    await def.open("postgres://db/app");
    await def.close();
    const viaDefaultImport = await loadPgDriver("postgres://db/app");
    await viaDefaultImport.open("postgres://db/app");
    await viaDefaultImport.close();
  });
});

describe("PostgresConnection", () => {
  it("opens, reopens, transacts, and closes", async () => {
    const { driver, client } = fakeDriver();
    const connection = new PostgresConnection({
      url: "postgres://db/app",
      schema: "s",
      driver,
    });
    await connection.open();
    await connection.open();
    expect(connection.createCommand()).toBeInstanceOf(PostgresCommand);
    expect(connection.opened).toBe(true);
    expect(connection.schema).toBe("s");
    const txn = await connection.startTransaction();
    await txn.commit();
    const txn2 = await connection.startTransaction(IsolationLevel.ReadUncommitted);
    expect(txn2.isolation).toBe(IsolationLevel.ReadUncommitted);
    await txn2.abort();
    expect(txn.isolation).toBe("serializable");
    await connection.close();
    expect(client()?.ended).toBe(true);
    expect(() => connection.ensureOpen()).toThrow(DatasourceError);
    await expect(connection.startTransaction()).rejects.toThrow(DatasourceError);
  });

  it("defaults url and requires a driver", () => {
    expect(() => new PostgresConnection()).toThrow(DatasourceError);
    const connection = new PostgresConnection({ driver: fakeDriver().driver });
    expect(connection.url).toBe("");
  });
});

describe("PostgresCommand and PostgresRepository", () => {
  async function setup() {
    const { driver } = fakeDriver();
    const connection = new PostgresConnection({ driver });
    await connection.open();
    return {
      connection,
      command: new PostgresCommand(connection),
      repo: new PostgresRepository<{ id?: number }>(connection),
    };
  }

  it("executes SQL through the driver", async () => {
    const { command } = await setup();
    expect(await command.execute("DELETE FROM items", [{ name: "id", value: 1 }]))
      .toEqual({ result: 5, rowsAffected: 2 });
    expect(await command.execute("")).toEqual({ rowsAffected: 0 });
    const reader = await command.executeReader("SELECT * FROM items");
    expect(await reader.next()).toBe(true);
    expect(reader.current).toEqual({ id: 5 });
    const empty = await command.executeReader("");
    expect(await empty.next()).toBe(false);
  });

  it("holds connection and table as a placeholder", async () => {
    const { repo, connection } = await setup();
    expect(repo).toBeInstanceOf(PostgresRepository);
    expect(repo.table).toBe("items");
    expect(new PostgresRepository(connection, { table: "users" }).table).toBe(
      "users",
    );
    await expect(repo.getSingle()).rejects.toThrow("postgres");
    await expect(repo.get().next()).rejects.toThrow("postgres");
    await expect(repo.add({})).rejects.toThrow("postgres");
    await expect(repo.update({})).rejects.toThrow("postgres");
    await expect(repo.delete({})).rejects.toThrow("postgres");
  });
});

describe("PostgresDatasource", () => {
  it("registers and constructs children", async () => {
    const { driver } = fakeDriver();
    expect(PostgresDatasource.create().provider).toBe("postgres");
    expect(() => new PostgresDatasource().createConnection()).toThrow(
      DatasourceError,
    );
    const ds = new PostgresDatasource({ driver, table: "people" });
    expect(ds.create({ driver }).provider).toBe("postgres");
    const connection = await ds.createConnection();
    await (connection as PostgresConnection).open();
    const command = ds.createCommand(connection);
    expect(command).toBeInstanceOf(PostgresCommand);
    expect(command.connection).toBe(connection);
    await command.execute("DELETE FROM items");
    await command.executeReader("SELECT 1");
    const implicit = new PostgresDatasource({ driver }).createCommand();
    expect(implicit).toBeInstanceOf(PostgresCommand);
    expect(implicit.connection).toBeInstanceOf(PostgresConnection);
    await implicit.connection.open();
    await implicit.execute("DELETE FROM items");
    await implicit.executeReader("SELECT 1");
    const repo = ds.createRepository(connection);
    expect(ds.createReadonlyRepository(connection)).toBeInstanceOf(PostgresRepository);
    expect((repo as PostgresRepository<never>).table).toBe("people");
    expect(
      (new PostgresDatasource({ driver }).createRepository() as PostgresRepository<never>)
        .table,
    ).toBe("items");
    expect(Datasource.create("postgres", { driver }).provider).toBe("postgres");
  });
});
