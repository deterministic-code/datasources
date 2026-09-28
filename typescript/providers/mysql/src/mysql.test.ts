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

import { Datasource, DatasourceFactory, DatasourceError, IsolationLevel, CRUDRepository, ReadonlyRepository, untyped, ParameterDirection, columnType, CreateTableStatement, ExecuteStoredProcedureStatement, ReturnStatement, SelectStatement, type IDriver } from "@datasources/core";
import { bindParameters, toMysql } from "./bind.ts";
import { MysqlCommand } from "./command.ts";
import { MysqlConnection } from "./connection.ts";
import { MysqlRowTypeConverters } from "./converters/MysqlRowTypeConverters.ts";
import { mysql2Connect, type IMysqlClient, type MysqlQueryHeader } from "./i-mysql-engine.ts";
import { MysqlDriver } from "./MysqlDriver.ts";
import { MysqlTransaction } from "./transaction.ts";
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
  async *queryRows(sql: string, params?: Record<string, Scalar>): AsyncGenerator<Cells> {
    this.queries.push({ sql, params });
    if (!this.arrayResult) return;
    for (const row of this.rows) yield row;
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

const opened = async (url = "mysql://localhost/app", client = new FakeClient()) => {
  const connection = new MysqlConnection({ url, connect: async () => client });
  await connection.open();
  return { connection, client, command: new MysqlCommand(connection) };
};

afterEach(() => {
  DatasourceFactory.reset();
  DatasourceFactory.register("mysql", new MysqlDriver());
});

describe("bindParameters", () => {
  it("binds and rewrites parameters", () => {
    expect(bindParameters()).toEqual({});
    expect(bindParameters(null)).toEqual({});
    expect(bindParameters([{ name: "id", value: 1 }])).toEqual({ id: 1 });
    expect(toMysql("SELECT * FROM t WHERE id = $id")).toBe(
      "SELECT * FROM t WHERE id = :id",
    );
    expect(toMysql("INSERT INTO t (id) VALUES ($id) RETURNING id")).toBe(
      "INSERT INTO t (id) VALUES (:id)",
    );
  });
});

describe("MysqlConnection", () => {
  it("opens, runs SQL, transacts, and closes", async () => {
    const { connection, client, command } = await opened("mysql://db/app");
    await connection.open();
    expect(await command.execute("INSERT INTO items (id) VALUES ($id)", [{ name: "id", value: 1 }]))
      .toMatchObject({ rowsAffected: 2, result: null });
    expect(await command.executeStatement(new CreateTableStatement("users", [{ name: "id", type: "INTEGER" }])))
      .toMatchObject({ rowsAffected: 2 });
    expect(client.queries.at(-1)?.sql).toBe("CREATE TABLE users (id INTEGER)");
    client.arrayResult = true;
    expect(await (await command.executeReaderStatement(new SelectStatement("SELECT 1"))).next()).toBe(true);
    expect(await command.execute("   ")).toEqual({ result: undefined, rowsAffected: 0, parameters: undefined });
    client.arrayResult = true;
    const reader = await command.executeReader("SELECT * FROM items");
    expect(await reader.next()).toBe(true);
    expect(reader.current).toEqual({ id: 1 });
    expect(await (await command.executeReader("")).next()).toBe(false);
    client.arrayResult = false;
    await connection.startTransaction();
    await connection.startTransaction(IsolationLevel.ReadUncommitted);
    await connection.startTransaction(IsolationLevel.ReadCommitted);
    await connection.startTransaction(IsolationLevel.RepeatableRead);
    await new MysqlTransaction(connection).commit();
    await new MysqlTransaction(connection).abort();
    expect(client.begins).toBe(4);
    expect(client.commits).toBe(1);
    expect(client.rollbacks).toBe(1);
    expect(client.queries.map((q) => q.sql)).toEqual([
      "SET SESSION sql_mode = CONCAT(@@sql_mode, ',ANSI_QUOTES')",
      "INSERT INTO items (id) VALUES (:id)",
      "CREATE TABLE users (id INTEGER)",
      "SELECT 1",
      "SELECT * FROM items",
      "SET TRANSACTION ISOLATION LEVEL REPEATABLE READ",
      "SET TRANSACTION ISOLATION LEVEL READ UNCOMMITTED",
      "SET TRANSACTION ISOLATION LEVEL READ COMMITTED",
      "SET TRANSACTION ISOLATION LEVEL REPEATABLE READ",
    ]);
    await connection.close();
    expect(client.ended).toBe(true);
    expect(() => connection.require()).toThrow("not open");
    await expect(command.execute("x")).rejects.toThrow("not open");
  });

  it("connects through mysql2 and treats array results as row counts", async () => {
    const connect = mysql2Connect({
      createConnection: async () => new FakeClient(),
    });
    const named = new MysqlConnection({ url: "mysql://db/app", connect });
    await named.open();
    await named.close();

    const { command, client, connection } = await opened("x");
    client.arrayResult = true;
    expect(await command.execute("SELECT 1")).toMatchObject({ rowsAffected: 1, result: undefined });
    client.arrayResult = false;
    client.header = {};
    expect(await command.execute("UPDATE items")).toMatchObject({ rowsAffected: 0 });
    await connection.close();
    await connection.close();
  });

  it("opens, reopens, transacts, and closes", async () => {
    const { connection, client } = await opened("mysql://db/app");
    await connection.open();
    expect(connection.createCommand()).toBeInstanceOf(MysqlCommand);
    expect(connection.isOpen).toBe(true);
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

  it("defaults url and requires a client to open", async () => {
    const connection = new MysqlConnection();
    expect(connection.url).toBe("");
    await expect(connection.open()).rejects.toThrow(DatasourceError);
  });
});

describe("MysqlCommand", () => {
  it("executes SQL through the client", async () => {
    const { command, client } = await opened();
    client.arrayResult = true;
    expect(await command.execute("DELETE FROM items", [{ name: "id", value: 1 }]))
      .toEqual({ result: undefined, rowsAffected: 1, parameters: [{ name: "id", value: 1 }] });
    expect(await command.execute("")).toEqual({ result: undefined, rowsAffected: 0, parameters: undefined });
    const reader = await command.executeReader("SELECT * FROM items");
    expect(await reader.next()).toBe(true);
    expect(reader.current).toEqual({ id: 1 });
    const empty = await command.executeReader("");
    expect(await empty.next()).toBe(false);
    expect(await (await command.executeReader(";")).next()).toBe(false);
    client.rows = [{ add_one: 5 }];
    const n = { name: "n", value: 4 };
    expect(await command.executeStatement(new ReturnStatement("add_one", [n]), [n]))
      .toEqual({ result: 5, rowsAffected: 1, parameters: [n] });
    client.rows = [];
    expect(await command.execute("SELECT 1")).toEqual({
      result: undefined,
      rowsAffected: 0,
      parameters: undefined,
    });
    client.rows = [{ result: 5 }];
    const start = { name: "start_n", value: 4 };
    const out = { name: "result", direction: ParameterDirection.Output };
    expect(
      await command.executeStatement(new ExecuteStoredProcedureStatement("bump", [start, out]), [start, out]),
    ).toMatchObject({
      parameters: [start, { name: "result", direction: ParameterDirection.Output, value: 5 }],
    });
    const inout = { name: "n", value: 4, direction: ParameterDirection.InputOutput };
    client.rows = [{ n: 5 }];
    expect(await command.executeStatement(new ExecuteStoredProcedureStatement("bump", [inout]), [inout])).toMatchObject({
      parameters: [{ name: "n", value: 5, direction: ParameterDirection.InputOutput }],
    });
    expect(
      await command.executeStatement(new ExecuteStoredProcedureStatement("bump", [
        { name: "start_n" },
        { name: "result", direction: ParameterDirection.Output },
      ]), [
        { name: "start_n" },
        { name: "result", direction: ParameterDirection.Output },
      ]),
    ).toMatchObject({ rowsAffected: 0 });
    client.arrayResult = true;
    client.rows = [];
    expect(await command.executeStatement(new SelectStatement("SELECT 1"))).toMatchObject({
      result: null,
      rowsAffected: 0,
    });
    client.rows = [{}];
    expect(await command.executeStatement(new SelectStatement("SELECT 1"))).toMatchObject({ result: null });
    expect(await command.executeStatement(new ExecuteStoredProcedureStatement("bump", []))).toMatchObject({
      result: undefined,
    });
    client.arrayResult = false;
    expect(await (await command.executeReader("SELECT 1")).next()).toBe(false);
    expect(
      await command.executeStatement(new ExecuteStoredProcedureStatement("bump", [
        { name: "result", direction: ParameterDirection.Output },
      ]), [
        { name: "result", direction: ParameterDirection.Output },
      ]),
    ).toMatchObject({ rowsAffected: 0 });
  });
});

describe("Mysql datasource", () => {
  it("registers and constructs children", async () => {
    expect(
      await new Datasource("mysql", new MysqlDriver()).createConnection(),
    ).toBeInstanceOf(MysqlConnection);
    const of = (): IDriver => ({
      connection: () => new MysqlConnection({ connect: async () => new FakeClient() }),
      command: (c) => c.createCommand(),
      transaction: (c, isolation = "serializable") =>
        new MysqlTransaction(c as MysqlConnection, isolation),
      rowTypeConverters: MysqlRowTypeConverters,
      columnType,
    });
    const ds = new Datasource("mysql", of());
    expect(ds.provider).toBe("mysql");
    const connection = await ds.createConnection();
    await (connection as MysqlConnection).open();
    const command = ds.createCommand(connection);
    expect(command).toBeInstanceOf(MysqlCommand);
    expect(command.connection).toBe(connection);
    await command.execute("DELETE FROM items");
    await command.executeReader("SELECT 1");
    const implicit = ds.createCommand();
    expect(implicit).toBeInstanceOf(MysqlCommand);
    expect(implicit.connection).toBeInstanceOf(MysqlConnection);
    await implicit.connection.open();
    await implicit.execute("DELETE FROM items");
    await implicit.executeReader("SELECT 1");
    const repo = ds.createRepository(untyped(), connection);
    expect(ds.createReadonlyRepository(untyped(), connection).constructor).toBe(ReadonlyRepository);
    expect(repo).toBeInstanceOf(CRUDRepository);
    expect(new Datasource("mysql", of()).createRepository(untyped())).toBeInstanceOf(CRUDRepository);
    expect(Datasource.create({ provider: "mysql" }).provider).toBe("mysql");
    expect(MysqlConnection.from({ url: "mysql://db/app" }).url).toBe("mysql://db/app");
    expect(MysqlConnection.from({ database_url: "mysql://db/app" }).url).toBe("mysql://db/app");
    expect(MysqlConnection.from().url).toBe("");
    const provider = new MysqlDriver();
    expect(provider.columnType("int")).toBe("INT");
    expect(provider.columnType("uuid")).toBe("CHAR(36)");
    expect(provider.columnType("string")).toBe("TEXT");
    expect(provider.columnType("datetime")).toBe("TEXT");
    expect(provider.columnType("bool")).toBe("TINYINT");
    expect(provider.columnType("decimal")).toBe("DECIMAL(18,4)");
    expect(provider.columnType("float")).toBe("DOUBLE");
    expect(provider.columnType("binary")).toBe("BLOB");
    expect(() => provider.columnType("xml")).toThrow('Unknown mysql type "xml"');
    expect(provider.command(connection)).toBeInstanceOf(MysqlCommand);
    expect(provider.transaction(connection as MysqlConnection).isolation).toBe("serializable");
  });
});
