import { afterEach, describe, expect, it, vi } from "vitest";

vi.mock("mysql2/promise", () => ({
  createConnection: async () => ({
    query: async () => [{}],
    beginTransaction: async () => {},
    commit: async () => {},
    rollback: async () => {},
    end: async () => {},
  }),
}));

import { Datasource, DatasourceError, IsolationLevel } from "@datasources/core";
import {
  bindParameters,
  loadMysql2Driver,
  Mysql2Driver,
  MysqlCommand,
  MysqlConnection,
  MysqlDatasource,
  MysqlRepository,
  requireDriver,
  toMysql,
  type IMysqlClient,
  type MysqlQueryHeader,
} from "./index.ts";
import type { Cells, Scalar } from "./scalar.ts";

class FakeClient implements IMysqlClient {
  queries: Array<{ sql: string; params?: Record<string, Scalar> }> = [];
  begins = 0;
  commits = 0;
  rollbacks = 0;
  ended = false;
  header: MysqlQueryHeader = { affectedRows: 2, insertId: 5 };
  rows: Cells[] = [{ id: 1 }];
  arrayResult = false;

  async query(sql: string, params?: Record<string, Scalar>) {
    this.queries.push({ sql, params });
    return [this.arrayResult ? this.rows : this.header] as [
      MysqlQueryHeader | Cells[],
    ];
  }
  async beginTransaction(): Promise<void> {
    this.begins += 1;
  }
  async commit(): Promise<void> {
    this.commits += 1;
  }
  async rollback(): Promise<void> {
    this.rollbacks += 1;
  }
  async end(): Promise<void> {
    this.ended = true;
  }
}

function fakeDriver(url = "mysql://localhost/app") {
  const client = new FakeClient();
  return {
    driver: new Mysql2Driver(url, async () => client),
    client,
  };
}

afterEach(() => {
  Datasource.reset();
  Datasource.use(MysqlDatasource.factory);
});

describe("requireDriver and bindParameters", () => {
  it("requires a driver", () => {
    expect(() => requireDriver(undefined)).toThrow(DatasourceError);
    const { driver } = fakeDriver();
    expect(requireDriver(driver)).toBe(driver);
  });

  it("binds and rewrites parameters", () => {
    expect(bindParameters()).toEqual({});
    expect(bindParameters(null)).toEqual({});
    expect(bindParameters([{ name: "id", value: 1 }])).toEqual({ id: 1 });
    expect(toMysql("SELECT * FROM t WHERE id = $id")).toBe(
      "SELECT * FROM t WHERE id = :id",
    );
  });
});

describe("Mysql2Driver", () => {
  it("wraps a scripted client", async () => {
    const { driver, client } = fakeDriver("mysql://db/app");
    await driver.open("mysql://db/app");
    await driver.open("mysql://db/app");
    expect(await driver.execute("INSERT INTO items (id) VALUES ($id)", { id: 1 }))
      .toEqual({ rowsAffected: 2, result: 5 });
    expect(await driver.execute("   ")).toEqual({ rowsAffected: 0 });
    client.arrayResult = true;
    expect(await driver.query("SELECT * FROM items", { id: 1 })).toEqual([
      { id: 1 },
    ]);
    expect(await driver.query("")).toEqual([]);
    client.arrayResult = false;
    await driver.begin();
    await driver.begin(IsolationLevel.ReadUncommitted);
    await driver.begin(IsolationLevel.ReadCommitted);
    await driver.begin(IsolationLevel.RepeatableRead);
    await driver.commit();
    await driver.rollback();
    expect(client.begins).toBe(4);
    expect(client.commits).toBe(1);
    expect(client.rollbacks).toBe(1);
    expect(client.queries.map((q) => q.sql)).toEqual([
      "SET SESSION sql_mode = CONCAT(@@sql_mode, ',ANSI_QUOTES')",
      "INSERT INTO items (id) VALUES (:id)",
      "SELECT * FROM items",
      "SET TRANSACTION ISOLATION LEVEL SERIALIZABLE",
      "SET TRANSACTION ISOLATION LEVEL READ UNCOMMITTED",
      "SET TRANSACTION ISOLATION LEVEL READ COMMITTED",
      "SET TRANSACTION ISOLATION LEVEL REPEATABLE READ",
    ]);
    await driver.close();
    expect(client.ended).toBe(true);
    await expect(driver.execute("x")).rejects.toThrow("not open");
  });

  it("loads a module and treats array execute results as row counts", async () => {
    const named = await loadMysql2Driver("mysql://db/app", async () => ({
      createConnection: async () => new FakeClient(),
    }));
    await named.open("mysql://db/app");
    await named.close();

    const viaDefaultImport = await loadMysql2Driver("mysql://db/app");
    await viaDefaultImport.open("mysql://db/app");
    await viaDefaultImport.close();

    const { driver, client } = fakeDriver();
    await driver.open("x");
    client.arrayResult = true;
    expect(await driver.execute("SELECT 1")).toEqual({ rowsAffected: 1 });
    client.arrayResult = false;
    client.header = {};
    expect(await driver.execute("UPDATE items")).toEqual({
      rowsAffected: 0,
      result: undefined,
    });
    await driver.close();
    await driver.close();
  });
});

describe("MysqlConnection", () => {
  it("opens, reopens, transacts, and closes", async () => {
    const { driver, client } = fakeDriver();
    const connection = new MysqlConnection({ url: "mysql://db/app", driver });
    await connection.open();
    await connection.open();
    expect(connection.opened).toBe(true);
    const txn = await connection.startTransaction();
    await txn.commit();
    const txn2 = await connection.startTransaction(IsolationLevel.ReadUncommitted);
    expect(txn2.isolation).toBe(IsolationLevel.ReadUncommitted);
    await txn2.abort();
    expect(txn.isolation).toBe("serializable");
    await connection.close();
    expect(client.ended).toBe(true);
    expect(() => connection.ensureOpen()).toThrow(DatasourceError);
    await expect(connection.startTransaction()).rejects.toThrow(DatasourceError);
  });

  it("defaults url and requires a driver", () => {
    expect(() => new MysqlConnection()).toThrow(DatasourceError);
    const connection = new MysqlConnection({ driver: fakeDriver().driver });
    expect(connection.url).toBe("");
  });
});

describe("MysqlCommand and MysqlRepository", () => {
  async function setup() {
    const { driver, client } = fakeDriver();
    const connection = new MysqlConnection({ driver });
    await connection.open();
    return {
      client,
      connection,
      command: new MysqlCommand(connection),
      repo: new MysqlRepository<{ id?: number; name?: string }>(connection),
    };
  }

  it("executes SQL through the driver", async () => {
    const { command, client } = await setup();
    client.arrayResult = true;
    expect(await command.execute("DELETE FROM items", [{ name: "id", value: 1 }]))
      .toEqual({ result: undefined, rowsAffected: 1 });
    expect(await command.execute()).toEqual({ rowsAffected: 0 });
    const reader = await command.executeReader("SELECT * FROM items");
    expect(await reader.next()).toBe(true);
    expect(reader.current).toEqual({ id: 1 });
    const empty = await command.executeReader();
    expect(await empty.next()).toBe(false);
  });

  it("holds connection and table as a placeholder", async () => {
    const { repo, connection } = await setup();
    expect(repo.connection).toBe(connection);
    expect(repo.table).toBe("items");
    const named = new MysqlRepository(connection, { table: "users" });
    expect(named.table).toBe("users");
    await expect(repo.getSingle()).rejects.toThrow("mysql");
    await expect(repo.get().next()).rejects.toThrow("mysql");
    await expect(repo.add({})).rejects.toThrow("mysql");
    await expect(repo.update({})).rejects.toThrow("mysql");
    await expect(repo.delete({})).rejects.toThrow("mysql");
  });
});

describe("MysqlDatasource", () => {
  it("registers and constructs children", async () => {
    const { driver } = fakeDriver();
    expect(MysqlDatasource.create().provider).toBe("mysql");
    expect(() => new MysqlDatasource().createConnection()).toThrow(
      DatasourceError,
    );
    const ds = new MysqlDatasource({ driver, table: "people" });
    expect(ds.create({ driver }).provider).toBe("mysql");
    const connection = await ds.createConnection();
    await (connection as MysqlConnection).open();
    const command = ds.createCommand(connection);
    expect(command).toBeInstanceOf(MysqlCommand);
    expect(command.connection).toBe(connection);
    await command.execute("DELETE FROM items");
    await command.executeReader("SELECT 1");
    const implicit = new MysqlDatasource({ driver }).createCommand();
    expect(implicit).toBeInstanceOf(MysqlCommand);
    expect(implicit.connection).toBeInstanceOf(MysqlConnection);
    await implicit.connection.open();
    await implicit.execute("DELETE FROM items");
    await implicit.executeReader("SELECT 1");
    const repo = ds.createRepository(connection);
    expect((repo as MysqlRepository<never>).table).toBe("people");
    expect(
      (new MysqlDatasource({ driver }).createRepository() as MysqlRepository<never>)
        .table,
    ).toBe("items");
    expect(Datasource.create("mysql", { driver }).provider).toBe("mysql");
  });
});
