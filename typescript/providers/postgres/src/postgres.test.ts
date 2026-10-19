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

import { Datasource, DatasourceFactory, DatasourceError, IsolationLevel, CRUDRepository, ReadonlyRepository, untyped, Native, ParameterDirection, columnType, RowTypeConverter, CreateTableStatement, ExecuteStoredProcedureStatement, ReturnStatement, SelectStatement, type IDriver } from "@deterministic-code/datasources";
import { bindParameters, toPostgres } from "./bind.ts";
import { PostgresCommand } from "./command.ts";
import { PostgresConnection } from "./connection.ts";
import { PostgresRowTypeConverters } from "./converters/PostgresRowTypeConverters.ts";
import { pgClient, type IPgClient, type PgModule, type PgQueryResult } from "./i-postgres-driver.ts";
import { PostgresDriver } from "./PostgresDriver.ts";
import { PostgresTransaction } from "./transaction.ts";
import type { Scalar } from "./scalar.ts";

class FakeClient implements IPgClient {
  static latest: FakeClient;
  queries: Array<{ text: string; values?: Scalar[] }> = [];
  connected = false;
  ended = false;
  result: PgQueryResult = { rowCount: 2, rows: [{ id: 5 }] };

  constructor(_options: { connectionString: string }) {
    FakeClient.latest = this;
  }

  async connect(): Promise<void> {
    this.connected = true;
  }
  async query(text: string, values?: Scalar[]): Promise<PgQueryResult> {
    this.queries.push({ text, values });
    return this.result;
  }
  async *rows(text: string, values?: Scalar[]): AsyncGenerator<PgQueryResult["rows"][number]> {
    this.queries.push({ text, values });
    for (const row of this.result.rows) yield row;
  }
  async end(): Promise<void> {
    this.ended = true;
  }
}

afterEach(() => {
  DatasourceFactory.reset();
  DatasourceFactory.register("postgres", new PostgresDriver());
});

describe("bind", () => {
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
    expect(toPostgres(`INSERT INTO "contact_sources" ("name") VALUES ('Manual')`).text)
      .toBe(`INSERT INTO "contact_sources" ("name") VALUES ('Manual')`);
    expect(toPostgres("SELECT * FROM OldContactsTbl").text).toBe(`SELECT * FROM "OldContactsTbl"`);
    expect(bindParameters([{ name: "at", value: new Date("2020-01-02T03:04:05.000Z") }]))
      .toEqual({ at: "2020-01-02T03:04:05.000Z" });
    expect(toPostgres("SELECT $at", { at: "2020-01-02T03:04:05.000Z" })).toEqual({
      text: "SELECT $1",
      values: [new Date("2020-01-02T03:04:05.000Z")],
    });
    expect(toPostgres("SELECT $v", { v: "AA==" })).toEqual({
      text: "SELECT $1",
      values: [Uint8Array.from([0])],
    });
    expect(toPostgres("SELECT $n", { n: "ada" }).values).toEqual(["ada"]);
  });
});

describe("PostgresConnection", () => {
  it("opens, runs SQL, transacts, and closes", async () => {
    const connection = new PostgresConnection({
      url: "postgres://db/app",
      schema: "iso",
      Client: FakeClient,
    });
    await connection.open();
    await expect(connection.open()).rejects.toThrow("Connection is already open");
    const client = FakeClient.latest;
    expect(client.connected).toBe(true);
    const command = new PostgresCommand(connection);
    expect(await command.execute("INSERT INTO items (id) VALUES ($id)", [{ name: "id", value: 1 }]))
      .toMatchObject({ rowsAffected: 2, result: null });
    client.result = { rowCount: 0, rows: [] };
    expect(await command.executeNative(new Native("INSERT INTO items (id) VALUES (1)"))).toEqual({
      rowsAffected: 0,
      result: null,
      parameters: undefined,
    });
    client.result = { rowCount: null, rows: [{ id: 9 }] };
    expect(await command.executeNative(new Native("UPDATE items SET n = 1"))).toMatchObject({
      rowsAffected: 1,
      result: null,
    });
    expect(await command.executeNative(new Native("   "))).toEqual({ rowsAffected: 0, parameters: undefined });
    client.result = { rowCount: null, rows: [{ id: 1 }] };
    expect(await command.executeReader("SELECT * FROM items")).toBeInstanceOf(Object);
    const reader = await command.executeReader("SELECT * FROM items");
    expect(await reader.next()).toBe(true);
    expect(await command.executeStatement(new CreateTableStatement("users", [{ name: "id", type: "INTEGER" }])))
      .toMatchObject({ rowsAffected: 1 });
    expect(await (await command.executeReaderNative(new Native("SELECT 1"))).next()).toBe(true);
    await connection.startTransaction();
    await connection.startTransaction(IsolationLevel.ReadUncommitted);
    await connection.startTransaction(IsolationLevel.ReadCommitted);
    await connection.startTransaction(IsolationLevel.RepeatableRead);
    await connection.startTransaction(IsolationLevel.Serializable);
    await new PostgresTransaction(connection).commit();
    await new PostgresTransaction(connection).abort();
    expect(client.queries.map((q) => q.text)).toEqual([
      "SET search_path TO iso",
      "INSERT INTO items (id) VALUES ($1)",
      "INSERT INTO items (id) VALUES (1)",
      "UPDATE items SET n = 1",
      "SELECT * FROM items",
      "SELECT * FROM items",
      "CREATE TABLE users (id INTEGER)",
      "SELECT 1",
      "BEGIN ISOLATION LEVEL READ COMMITTED",
      "BEGIN ISOLATION LEVEL READ UNCOMMITTED",
      "BEGIN ISOLATION LEVEL READ COMMITTED",
      "BEGIN ISOLATION LEVEL REPEATABLE READ",
      "BEGIN ISOLATION LEVEL SERIALIZABLE",
      "COMMIT",
      "ROLLBACK",
    ]);
    await connection.close();
    expect(client.ended).toBe(true);
    await expect(connection.exec("SELECT 1")).rejects.toThrow("not open");
    await expect(command.executeNative(new Native("x"))).rejects.toThrow("not open");
  });

  it("loads the pg client from a module", async () => {
    const Client = class implements IPgClient {
      async connect(): Promise<void> {}
      async query() {
        return { rowCount: 0, rows: [] };
      }
      async *rows() {}
      async end(): Promise<void> {}
    };
    const named = new PostgresConnection({
      url: "postgres://db/app",
      Client: pgClient({ Client } as PgModule),
    });
    await named.open();
    await named.close();
    const viaDefault = new PostgresConnection({
      url: "postgres://db/app",
      Client: pgClient({ Client, default: { Client } } as PgModule),
    });
    await viaDefault.open();
    await viaDefault.close();
    expect(PostgresConnection.from({ DATASOURCE_URL: "postgres://db/app" }).url).toBe("postgres://db/app");
  });

  it("opens, reopens, transacts, and closes through the connection", async () => {
    const connection = new PostgresConnection({
      url: "postgres://db/app",
      schema: "s",
      Client: FakeClient,
    });
    await connection.open();
    expect(connection.createCommand()).toBeInstanceOf(PostgresCommand);
    expect(connection.isOpen).toBe(true);
    expect(connection.schema).toBe("s");
    const txn = await connection.startTransaction();
    await txn.commit();
    const txn2 = await connection.startTransaction(IsolationLevel.ReadUncommitted);
    expect(txn2.isolation).toBe(IsolationLevel.ReadUncommitted);
    await txn2.abort();
    expect(txn.isolation).toBe("readCommitted");
    await connection.close();
    expect(FakeClient.latest.ended).toBe(true);
    expect(() => connection.ensureOpen()).toThrow(DatasourceError);
    await expect(connection.startTransaction()).rejects.toThrow(DatasourceError);
  });

  it("defaults url and requires a client to open", async () => {
    const connection = new PostgresConnection();
    expect(connection.url).toBe("");
    await expect(connection.open()).rejects.toThrow(DatasourceError);
  });
});

describe("PostgresCommand", () => {
  async function setup() {
    const connection = new PostgresConnection({ Client: FakeClient });
    await connection.open();
    return { connection, command: new PostgresCommand(connection), client: FakeClient.latest };
  }

  it("executes SQL through the client", async () => {
    const { command, client } = await setup();
    expect(await command.execute("DELETE FROM items", [{ name: "id", value: 1 }]))
      .toEqual({ result: null, rowsAffected: 2, parameters: [{ name: "id", value: 1 }] });
    expect(await command.executeNative(new Native(""))).toEqual({ result: undefined, rowsAffected: 0, parameters: undefined });
    const reader = await command.executeReader("SELECT * FROM items");
    expect(await reader.next()).toBe(true);
    expect(reader.current).toEqual({ id: 5 });
    const empty = await command.executeReaderNative(new Native(""));
    expect(await empty.next()).toBe(false);
    expect(await (await command.executeReaderNative(new Native(";"))).next()).toBe(false);
    client.result = { rowCount: 1, rows: [{ add_one: 5 }] };
    const n = { name: "n", value: 4 };
    expect(await command.executeStatement(new ReturnStatement("add_one", [n]), [n]))
      .toEqual({ result: 5, rowsAffected: 1, parameters: [n] });
    client.result = { rowCount: 0, rows: [] };
    expect(await command.executeStatement(new SelectStatement({ table: "dual", fields: ["1"] }))).toMatchObject({
      result: null,
      rowsAffected: 0,
    });
    client.result = { rowCount: 1, rows: [{}] };
    expect(await command.executeStatement(new SelectStatement({ table: "dual", fields: ["1"] }))).toMatchObject({ result: null });
    client.result = { rowCount: 0, rows: [] };
    expect(await command.executeNative(new Native("SELECT 1"))).toEqual({
      result: null,
      rowsAffected: 0,
      parameters: undefined,
    });
    client.result = { rowCount: 1, rows: [{ id: null }] };
    expect(await command.executeNative(new Native("SELECT flag($b)"), [{ name: "b", value: true }])).toMatchObject({ result: null });
    expect(await command.executeNative(new Native("SELECT when($d)"), [{ name: "d", value: new Date("2026-09-22T00:00:00.000Z") }])).toMatchObject({ result: null });
    expect(await command.executeNative(new Native("SELECT blob($b)"), [{ name: "b", value: new Uint8Array([1, 2, 3]) }])).toMatchObject({ result: null });
    const inout = { name: "result", value: 4, direction: ParameterDirection.InputOutput };
    client.result = { rowCount: 1, rows: [{ result: { n: 1 } }] };
    expect(await command.executeNative(new Native("CALL bump($result)"), [inout])).toMatchObject({
      parameters: [{ name: "result", value: "{\"n\":1}", direction: ParameterDirection.InputOutput }],
    });
    client.result = { rowCount: 1, rows: [{ result: 5 }] };
    expect(await command.executeStatement(new ExecuteStoredProcedureStatement("bump", [inout]), [inout])).toMatchObject({
      result: null,
      parameters: [{ name: "result", value: 5, direction: ParameterDirection.InputOutput }],
    });
    client.result = { rowCount: 1, rows: [{ id: 1 }] };
    expect(
      await command.executeNative(new Native("CALL noop($n)"), [
        { name: "n", value: 1, direction: ParameterDirection.Input },
      ]),
    ).toMatchObject({ rowsAffected: 1 });
    expect(await command.executeNative(new Native("CALL noop()"))).toMatchObject({ rowsAffected: 1 });
    client.result = { rowCount: 1, rows: [{ other: 9 }] };
    expect(
      await command.executeNative(new Native("CALL bump($missing)"), [
        { name: "missing", value: 1, direction: ParameterDirection.Output },
      ]),
    ).toMatchObject({
      parameters: [{ name: "missing", value: 1, direction: ParameterDirection.Output }],
    });
  });
});

describe("Postgres datasource", () => {
  it("registers and constructs children", async () => {
    expect(
      await new Datasource("postgres", new PostgresDriver()).createConnection(),
    ).toBeInstanceOf(PostgresConnection);
    const of = (): IDriver => ({
      connection: () => new PostgresConnection({ Client: FakeClient }),
      command: (c) => c.createCommand(),
      transaction: (c, isolation = "readCommitted") =>
        new PostgresTransaction(c as PostgresConnection, isolation),
      rowTypeConverters: PostgresRowTypeConverters,
      columnType,
    });
    const ds = new Datasource("postgres", of());
    expect(ds.provider).toBe("postgres");
    const connection = await ds.createConnection();
    await (connection as PostgresConnection).open();
    const command = await ds.createCommand(connection);
    expect(command).toBeInstanceOf(PostgresCommand);
    expect(command.connection).toBe(connection);
    await command.execute("DELETE FROM items");
    await command.executeReaderNative(new Native("SELECT 1"));
    await expect(ds.createCommand()).rejects.toThrow("Connection is already open");
    const repo = ds.createRepository(untyped(), connection);
    expect(ds.createReadonlyRepository(untyped(), connection).constructor).toBe(ReadonlyRepository);
    expect(repo).toBeInstanceOf(CRUDRepository);
    expect(new Datasource("postgres", of()).createRepository(untyped())).toBeInstanceOf(CRUDRepository);
    expect(Datasource.create({ PROVIDER: "postgres" }).provider).toBe("postgres");
    const provided = PostgresConnection.from({
      DATASOURCE_URL: "postgres://db/app?schema=s",
    });
    expect(provided.url).toBe("postgres://db/app?schema=s");
    expect(provided.schema).toBe("s");
    expect(PostgresConnection.from({ DATASOURCE_URL: "postgres://db/app?schema=app" }).schema).toBe("app");
    expect(PostgresConnection.from().url).toBe("");
    expect(PostgresConnection.from().schema).toBeUndefined();
    const provider = new PostgresDriver();
    expect(provider.columnType("int")).toBe("INTEGER");
    expect(provider.columnType("uuid")).toBe("UUID");
    expect(provider.columnType("string")).toBe("TEXT");
    expect(provider.columnType("bool")).toBe("BOOLEAN");
    expect(provider.columnType("decimal")).toBe("NUMERIC");
    expect(provider.columnType("float")).toBe("DOUBLE PRECISION");
    expect(provider.columnType("datetime")).toBe("TIMESTAMPTZ");
    expect(provider.columnType("binary")).toBe("BYTEA");
    expect(() => provider.columnType("xml")).toThrow('Unknown postgres type "xml"');
    expect(provider.command(connection)).toBeInstanceOf(PostgresCommand);
    expect(provider.transaction(connection as PostgresConnection).isolation).toBe("readCommitted");
  });

  it("maps property targets", () => {
    const provider = new PostgresDriver();
    const created = "2020-01-02T03:04:05.000Z";
    const converter = RowTypeConverter.from(provider.rowTypeConverters, {
      Key: "uuid",
      Active: "bool",
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
      Active: true,
      Login: "ada",
      Created: created,
    });
  });
});
