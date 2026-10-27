import { afterEach, beforeEach, describe, expect, it } from "vitest";
import {
  collectAsync,
  CRUDRepository,
  ReadonlyRepository,
  untyped,
  Datasource,
  DatasourceFactory,
  DatasourceError,
  DeleteStatement,
  CreateTableStatement,
  Native,
  SelectInsertedIdStatement,
  InsertStatement,
  Operator,
  SelectStatement,
  UpdateStatement,
  Where,
  WhereText,
} from "@deterministic-code/datasources";
import { MemoryCommand } from "./command.ts";
import { MemoryConnection } from "./connection.ts";
import { asRecord, matches, parametersToObject, parseFilter } from "./filter.ts";
import { MemoryDriver } from "./MemoryDriver.ts";
import { MemoryStore } from "./store.ts";

const envKeys = ["INSTALLED_PROVIDERS", "DATASOURCE_URL", "PROVIDER"] as const;

beforeEach(() => {
  for (const key of envKeys) delete process.env[key];
  DatasourceFactory.reset();
  DatasourceFactory.register("memory", new MemoryDriver());
  process.env.PROVIDER = "memory";
});

afterEach(() => {
  DatasourceFactory.reset();
  for (const key of envKeys) delete process.env[key];
});

describe("DatasourceFactory", () => {
  it("register and create", () => {
    DatasourceFactory.reset();
    DatasourceFactory.register("memory", new MemoryDriver());
    const ds = DatasourceFactory.create({ PROVIDER: "memory" });
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

  it("throws when opened twice", async () => {
    const connection = new MemoryConnection();
    await connection.open();
    await expect(connection.open()).rejects.toThrow("Connection is already open");
  });

  it("start a transaction", async () => {
    const connection = new MemoryConnection();
    await connection.open();
    const txn = await connection.startTransaction();
    expect(txn.isolation).toBe("readCommitted");
  });

  it("create and run a command", async () => {
    const connection = new MemoryConnection();
    await connection.open();
    const command = connection.createCommand();
    await command.execute("insert");
    expect(connection.getCollection()).toEqual([{ id: 1 }]);
  });

  it("closes with using", async () => {
    const connection = new MemoryConnection();
    {
      await using held = connection;
      await held.open();
    }
    expect(connection.isOpen).toBe(false);
  });
});

describe("Transaction", () => {
  it("commit", async () => {
    const connection = new MemoryConnection();
    await connection.open();
    const txn = await connection.startTransaction();
    await txn.commit();
    expect(txn.isolation).toBe("readCommitted");
  });

  it("abort", async () => {
    const connection = new MemoryConnection();
    await connection.open();
    const txn = await connection.startTransaction();
    await txn.abort();
    expect(txn.isolation).toBe("readCommitted");
  });

  it("command running in one atomic transaction", async () => {
    const ds = Datasource.default;
    const connection = await ds.createConnection();
    await connection.open();
    const txn = await connection.startTransaction();
    const command = connection.createCommand();
    const fields = [{ name: "name", value: "ada" }];
    await command.executeStatement(new InsertStatement("people", fields), fields);
    const reader = await command.executeReaderStatement(new SelectInsertedIdStatement("people", "id"));
    const moved = await reader.next();
    const id = reader.current;
    await txn.commit();
    expect(moved).toBe(true);
    expect(id).toEqual({ id: 1 });
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
  it("opens from memory data", async () => {
    const connection = new MemoryConnection({ data: [{ id: 1 }] });
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
    await expect(command.execute("")).rejects.toThrow("empty command");
    expect(await collectAsync(repo.read())).toEqual([]);
  });

  it("performs repository CRUD", async () => {
    const { repo } = await openRepo([{ id: 1, name: "a" }]);
    expect(await repo.readSingle()).toEqual({ id: 1, name: "a" });
    expect(await repo.readSingle(new WhereText('{"id":9}'))).toBeUndefined();
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

  it("rejects command IO when closed and opens on repository create", async () => {
    const connection = new MemoryConnection({ data: [{ id: 1 }] });
    const command = new MemoryCommand(connection);
    const repo = new CRUDRepository(connection, {});
    await expect(command.executeReader("")).rejects.toThrow(DatasourceError);
    await expect(command.execute("insert")).rejects.toThrow(DatasourceError);
    expect((await repo.create({ id: 2, name: "b" })).id).toBe(2);
    expect(connection.isOpen).toBe(true);
  });
});

describe("Memory datasource", () => {
  it("registers and constructs children", async () => {
    const ds = new Datasource(
      "memory",
      new MemoryDriver(),
      { data: [{ id: 1 }] },
    );
    expect(ds.provider).toBe("memory");
    const connection = (await ds.createConnection()) as MemoryConnection;
    await connection.open();
    expect(connection.createCommand()).toBeInstanceOf(MemoryCommand);
    const command = await ds.createCommand(connection);
    expect(command).toBeInstanceOf(MemoryCommand);
    expect(command.connection).toBe(connection);
    await command.execute("insert", [{ name: "id", value: 99 }]);
    await command.executeReader("");
    await expect(ds.createCommand()).rejects.toThrow("Connection is already open");
    const repo = ds.createRepository(untyped(), connection);
    expect(repo).toBeInstanceOf(CRUDRepository);
    expect(ds.createRepository(untyped())).toBeInstanceOf(CRUDRepository);
    expect(ds.createReadonlyRepository(untyped(), connection).constructor).toBe(ReadonlyRepository);
    expect(MemoryConnection.from().provider).toBe("memory");
    const registered = Datasource.create({ PROVIDER: "memory" });
    expect(registered.provider).toBe("memory");
    expect(await registered.createConnection()).toBeInstanceOf(MemoryConnection);
    expect(registered.createRepository(untyped())).toBeInstanceOf(CRUDRepository);
    const provider = new MemoryDriver();
    expect(provider.transaction(connection).isolation).toBe("readCommitted");
    await expect(command.executeNative(new Native("nope"))).rejects.toThrow(
      "Unsupported Memory",
    );
    expect(await (await command.executeReaderNative(new Native("{}"))).next()).toBe(true);
    await expect(
      command.executeStatement(new CreateTableStatement("t", [{ name: "id", type: "INTEGER" }])),
    ).rejects.toThrow("Unsupported Memory");
    await command.executeStatement(new InsertStatement("people", [{ name: "name", value: "ada" }]), [
      { name: "name", value: "ada" },
    ]);
    await command.executeStatement(
      new InsertStatement("people", [{ name: "id", value: 5 }, { name: "name", value: "hi" }]),
      [
        { name: "id", value: 5 },
        { name: "name", value: "hi" },
      ],
    );
    await command.executeStatement(
      new InsertStatement("people", [{ name: "id", value: 2 }, { name: "name", value: "lo" }]),
      [
        { name: "id", value: 2 },
        { name: "name", value: "lo" },
      ],
    );
    connection.rows("people", true).push({ id: "x", name: "str" });
    await command.executeStatement(new InsertStatement("people", [{ name: "name", value: "bea" }]), [
      { name: "name", value: "bea" },
    ]);
    const inserted = await command.executeReaderStatement(new SelectInsertedIdStatement("people", "id"));
    expect(await inserted.next()).toBe(true);
    expect(inserted.current).toEqual({ id: 6 });
    const matched = await command.executeReaderStatement(
      new SelectStatement({
        table: "people",
        fields: ["id"],
        where: new Where(new Operator.Equals("id", 1)),
      }),
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
      accept: () => "SELECT id FROM people",
      table: "people",
      where: new Where(new Operator.Equals("id", 1)),
    } as SelectStatement);
    await command.executeReaderStatement({
      accept: () => "SELECT id FROM people",
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
    const missingId = await command.executeReaderStatement(new SelectInsertedIdStatement("other", "id"));
    expect(await missingId.next()).toBe(false);
    await command.executeReaderStatement(
      new SelectStatement({
        table: "missing",
        fields: ["id"],
        where: new Where(new Operator.Equals("id", 1)),
      }),
    );
    await command.executeReaderStatement({
      accept: () => "nope",
      where: { accept: () => "id = 1" },
    } as SelectStatement);
    await command.executeReaderStatement({
      accept: () => '{"id":1}',
    } as SelectStatement);
    await command.executeReaderStatement(new InsertStatement("t", []));
    await command.executeStatement(
      new UpdateStatement("t", [{ name: "name", value: "Z" }], new WhereText('{"id":99}')),
    );
  });
});
