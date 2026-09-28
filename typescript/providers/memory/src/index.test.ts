import { afterEach, describe, expect, it } from "vitest";
import {
  collectAsync,
  CRUDRepository,
  ReadonlyRepository,
  untyped,
  Datasource,
  DatasourceFactory,
  DatasourceError,
  DeleteStatement,
  InsertedIdStatement,
  InsertStatement,
  Operator,
  SelectStatement,
  UpdateStatement,
  Where,
  WhereText,
} from "@datasources/core";
import { MemoryCommand } from "./command.ts";
import { MemoryConnection } from "./connection.ts";
import { asRecord, matches, parametersToObject, parseFilter } from "./filter.ts";
import { MemoryProvider } from "./MemoryProvider.ts";
import { MemoryStore } from "./store.ts";

afterEach(() => {
  DatasourceFactory.reset();
  DatasourceFactory.register("memory", new MemoryProvider());
});

describe("DatasourceFactory", () => {
  it("register and create", () => {
    DatasourceFactory.reset();
    DatasourceFactory.register("memory", new MemoryProvider());
    const ds = DatasourceFactory.create({ provider: "memory" });
    expect(ds.provider).toBe("memory");
  });
});

describe("Connection", () => {
  it("open and close", async () => {
    const connection = new MemoryConnection();
    await connection.open();
    await connection.close();
    expect(connection.isOpen).toBe(false);
  });

  it("start a transaction", async () => {
    const connection = new MemoryConnection();
    await connection.open();
    const txn = await connection.startTransaction();
    expect(txn.isolation).toBe("serializable");
  });
});

describe("Transaction", () => {
  it("commit", async () => {
    const connection = new MemoryConnection();
    await connection.open();
    const txn = await connection.startTransaction();
    await txn.commit();
    expect(txn.isolation).toBe("serializable");
  });

  it("abort", async () => {
    const connection = new MemoryConnection();
    await connection.open();
    const txn = await connection.startTransaction();
    await txn.abort();
    expect(txn.isolation).toBe("serializable");
  });
});

describe("filter helpers", () => {
  it("parses empty and object filters", () => {
    expect(parseFilter()).toEqual({});
    expect(parseFilter(null)).toEqual({});
    expect(parseFilter("")).toEqual({});
    expect(parseFilter("   ")).toEqual({});
    expect(parseFilter('{"id":1}')).toEqual({ id: 1 });
  });

  it("rejects non-object filters", () => {
    expect(() => parseFilter("null")).toThrow("Memory filter");
    expect(() => parseFilter("[]")).toThrow("Memory filter");
    expect(() => parseFilter("1")).toThrow("Memory filter");
  });

  it("matches every filter key", () => {
    expect(matches({ id: 1, name: "a" }, {})).toBe(true);
    expect(matches({ id: 1, name: "a" }, { id: 1 })).toBe(true);
    expect(matches({ id: 1, name: "a" }, { id: 2 })).toBe(false);
  });

  it("requires plain objects", () => {
    expect(asRecord({ id: 1 })).toEqual({ id: 1 });
    expect(() => asRecord(null)).toThrow("plain objects");
    expect(() => asRecord([1])).toThrow("plain objects");
    expect(() => asRecord("x")).toThrow("plain objects");
  });

  it("maps parameters", () => {
    expect(parametersToObject()).toEqual({});
    expect(parametersToObject([{ name: "id", value: 3 }])).toEqual({ id: 3 });
  });
});

describe("MemoryStore", () => {
  it("seeds arrays and named collections once", () => {
    const store = new MemoryStore();
    store.seed([{ id: 1 }], "");
    store.seed([{ id: 2 }], "");
    store.seed({ users: [{ id: 3 }], posts: [{ id: 4 }] }, "");
    store.seed({ users: [{ id: 9 }] }, "");
    expect(store.getCollection("")).toEqual([{ id: 1 }]);
    expect(store.getCollection("users")).toEqual([{ id: 3 }]);
    expect(store.getCollection("missing")).toEqual([]);
    const snap = store.snapshot();
    store.getCollection("").push({ id: 8 });
    store.restore(snap);
    expect(store.getCollection("")).toEqual([{ id: 1 }]);
  });
});

describe("MemoryConnection", () => {
  it("opens from memory data and is idempotent", async () => {
    const connection = new MemoryConnection({ data: [{ id: 1 }] });
    await connection.open();
    await connection.open();
    expect(connection.getCollection()).toEqual([{ id: 1 }]);
    await connection.close();
    expect(connection.isOpen).toBe(false);
  });

  it("uses a named collection and shared store", async () => {
    const store = new MemoryStore();
    const first = new MemoryConnection({
      store,
      collection: "users",
      data: { users: [{ name: "a" }] },
    });
    await first.open();
    const second = new MemoryConnection({ store, collection: "users" });
    await second.open();
    expect(second.getCollection()).toEqual([{ name: "a" }]);

    const empty = new MemoryConnection();
    await empty.open();
    expect(empty.getCollection()).toEqual([]);
  });

  it("aborts a transaction and rejects IO when closed", async () => {
    const connection = new MemoryConnection({ data: [{ id: 1 }] });
    await expect(connection.startTransaction()).rejects.toThrow(DatasourceError);
    await connection.open();
    const txn = await connection.startTransaction();
    connection.getCollection().push({ id: 2 });
    await txn.abort();
    expect(connection.getCollection()).toEqual([{ id: 1 }]);

    const committed = await connection.startTransaction();
    connection.getCollection().push({ id: 2 });
    await committed.commit();
    expect(connection.getCollection()).toEqual([{ id: 1 }, { id: 2 }]);
    await connection.close();
    expect(() => connection.ensureOpen()).toThrow(DatasourceError);
    connection.finishAbort();
  });
});

describe("MemoryCommand and CRUDRepository", () => {
  async function openRepo(data: Record<string, unknown>[] = []) {
    const connection = new MemoryConnection({ data });
    await connection.open();
    return {
      connection,
      command: new MemoryCommand(connection),
      repo: new CRUDRepository(connection, {}),
    };
  }

  it("reads with filters and parameters", async () => {
    const { command } = await openRepo([
      { id: 1, name: "a" },
      { id: 2, name: "b" },
    ]);
    const all = await command.executeReader("");
    expect(await all.next()).toBe(true);
    const filtered = await command.executeReader('{"id":2}');
    expect(await filtered.next()).toBe(true);
    expect(filtered.current).toEqual({ id: 2, name: "b" });
    const byParam = await command.executeReader("", [
      { name: "name", value: "a" },
    ]);
    expect(await byParam.next()).toBe(true);
    expect(byParam.current).toEqual({ id: 1, name: "a" });
  });

  it("inserts updates and deletes through execute", async () => {
    const { command, repo } = await openRepo([{ id: 1, name: "a" }]);
    expect(
      await command.execute("insert", [
        { name: "id", value: 2 },
        { name: "name", value: "b" },
      ]),
    ).toEqual({
      result: 2,
      rowsAffected: 1,
      parameters: [
        { name: "id", value: 2 },
        { name: "name", value: "b" },
      ],
    });

    expect(
      await command.execute("update", [
        { name: "id", value: 2 },
        { name: "name", value: "B" },
        { name: "$where", value: '{"id":2}' },
      ]),
    ).toMatchObject({ rowsAffected: 1 });

    expect(
      await command.execute("update", [
        { name: "name", value: "Z" },
        { name: "$where", value: '{"id":1}' },
      ]),
    ).toMatchObject({ rowsAffected: 1 });
    expect(
      await command.execute("update", [
        { name: "name", value: "Y" },
        { name: "$where", value: 1 },
      ]),
    ).toMatchObject({ rowsAffected: 0 });

    expect(
      await command.execute("delete", [{ name: "id", value: 2 }]),
    ).toMatchObject({ rowsAffected: 1 });
    expect(await command.execute("delete")).toMatchObject({
      rowsAffected: 1,
    });
    await expect(command.execute("nope")).rejects.toThrow("Unsupported Memory");
    await expect(command.execute("")).rejects.toThrow("Unsupported Memory");
    expect(await collectAsync(repo.read())).toEqual([]);
  });

  it("performs repository CRUD", async () => {
    const { repo } = await openRepo([{ id: 1, name: "a" }]);
    expect(await repo.readSingle()).toEqual({ id: 1, name: "a" });
    expect(await repo.readSingle(new SelectStatement('{"id":9}'))).toBeUndefined();
    expect(await collectAsync(repo.read(new SelectStatement('{"id":1}')))).toEqual([
      { id: 1, name: "a" },
    ]);
    expect(await repo.readSingle(new WhereText('{"id":1}'))).toEqual({ id: 1, name: "a" });
    expect(await collectAsync(repo.read(new WhereText('{"id":1}')))).toEqual([
      { id: 1, name: "a" },
    ]);
    expect((await repo.create({ id: 2, name: "b" })).id).toBe(2);
    expect((await repo.update({ id: 2, name: "B" })).rowsAffected).toBe(1);
    expect(
      (await repo.update({ name: "C" }, new WhereText('{"id":2}'))).rowsAffected,
    ).toBe(1);
    expect((await repo.delete({ id: 1 })).rowsAffected).toBe(1);
    expect(
      (await repo.delete({ id: 2 }, new WhereText('{"id":2}'))).rowsAffected,
    ).toBe(1);
    expect(await collectAsync(repo.read())).toEqual([]);
  });

  it("rejects command and repository IO when closed", async () => {
    const connection = new MemoryConnection({ data: [{ id: 1 }] });
    const command = new MemoryCommand(connection);
    const repo = new CRUDRepository(connection, {});
    await expect(command.executeReader("")).rejects.toThrow(DatasourceError);
    await expect(command.execute("insert")).rejects.toThrow(DatasourceError);
    await expect(repo.create({ id: 2 })).rejects.toThrow(DatasourceError);
    await expect(repo.update({ id: 1 })).rejects.toThrow(DatasourceError);
    await expect(repo.delete({ id: 1 })).rejects.toThrow(DatasourceError);
  });
});

describe("Memory datasource", () => {
  it("registers and constructs children", async () => {
    const ds = new Datasource(
      "memory",
      new MemoryProvider(),
      { data: [{ id: 1 }] },
    );
    expect(ds.provider).toBe("memory");
    const connection = (await ds.createConnection()) as MemoryConnection;
    await connection.open();
    expect(connection.createCommand()).toBeInstanceOf(MemoryCommand);
    const command = ds.createCommand(connection);
    expect(command).toBeInstanceOf(MemoryCommand);
    expect(command.connection).toBe(connection);
    await command.execute("insert", [{ name: "id", value: 99 }]);
    await command.executeReader("");
    const implicit = ds.createCommand();
    expect(implicit).toBeInstanceOf(MemoryCommand);
    expect(implicit.connection).toBeInstanceOf(MemoryConnection);
    await implicit.connection.open();
    await implicit.execute("insert", [{ name: "id", value: 1 }]);
    await implicit.executeReader("");
    const repo = ds.createRepository(untyped(), connection);
    expect(repo).toBeInstanceOf(CRUDRepository);
    expect(ds.createRepository(untyped())).toBeInstanceOf(CRUDRepository);
    expect(ds.createReadonlyRepository(untyped(), connection).constructor).toBe(ReadonlyRepository);
    expect(MemoryConnection.from().provider).toBe("memory");
    const registered = Datasource.create({ provider: "memory" });
    expect(registered.provider).toBe("memory");
    expect(await registered.createConnection()).toBeInstanceOf(MemoryConnection);
    expect(registered.createRepository(untyped())).toBeInstanceOf(CRUDRepository);
    const provider = new MemoryProvider();
    expect(provider.transaction(connection).isolation).toBe("serializable");
    await expect(command.executeStatement(new SelectStatement("nope"))).rejects.toThrow(
      "Unsupported Memory",
    );
    await command.executeStatement(new InsertStatement("people", [{ name: "name", value: "ada" }]));
    const inserted = await command.executeReaderStatement(new InsertedIdStatement("people", "id"));
    expect(await inserted.next()).toBe(true);
    expect(inserted.current).toEqual({ id: 1 });
    const matched = await command.executeReaderStatement(
      new SelectStatement("SELECT id FROM people", new Where(new Operator.Equals("id", 1))),
    );
    expect(await matched.next()).toBe(true);
    await command.executeStatement(new InsertStatement("people", [{ name: "id", value: 7 }, { name: "name", value: "bea" }]));
    await command.executeStatement(new UpdateStatement("people", [{ name: "name", value: "bee" }]));
    await command.executeStatement(
      new UpdateStatement("people", [{ name: "name", value: "bee" }], new Where(new Operator.Equals("id", 7))),
    );
    await command.executeStatement(new DeleteStatement("people"));
    await command.executeStatement(new InsertStatement("", [{ name: "name", value: "x" }]));
    await command.executeReaderStatement({
      getText: () => "SELECT id FROM people",
      table: "people",
      where: new Where(new Operator.Equals("id", 1)),
    } as SelectStatement);
    await command.executeReaderStatement({
      getText: () => "SELECT id FROM people",
      table: "",
      where: new Where(new Operator.Equals("id", 1)),
    } as SelectStatement);
    class OpenWhere extends Where {
      override get parameters() {
        return undefined as unknown as Where["parameters"];
      }
    }
    await command.executeStatement(
      new UpdateStatement("people", [{ name: "name", value: "bee" }], new OpenWhere(new Operator.Equals("id", 1))),
    );
    const missingId = await command.executeReaderStatement(new InsertedIdStatement("other", "id"));
    expect(await missingId.next()).toBe(false);
    await command.executeReaderStatement(
      new SelectStatement("SELECT id FROM missing", new Where(new Operator.Equals("id", 1))),
    );
    await command.executeReaderStatement({
      getText: () => "nope",
      where: { getText: () => "id = 1" },
    } as SelectStatement);
    await command.executeReaderStatement(new InsertStatement("t", []));
    await command.executeStatement(
      new UpdateStatement("t", [{ name: "name", value: "Z" }], new WhereText('{"id":99}')),
    );
  });
});
