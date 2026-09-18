import { afterEach, describe, expect, it } from "vitest";
import {
  collectAsync,
  Datasource,
  DatasourceError,
  Query,
  Where,
} from "@datasources/core";
import {
  asRecord,
  matches,
  MemoryCommand,
  MemoryConnection,
  MemoryDatasource,
  MemoryRepository,
  MemoryStore,
  parametersToObject,
  parseFilter,
} from "./index.ts";

afterEach(() => {
  Datasource.reset();
  Datasource.register(MemoryDatasource.instance);
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
    expect(connection.opened).toBe(false);
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

describe("MemoryCommand and MemoryRepository", () => {
  async function openRepo(data: Record<string, unknown>[] = []) {
    const connection = new MemoryConnection({ data });
    await connection.open();
    return {
      connection,
      command: new MemoryCommand(connection),
      repo: new MemoryRepository(connection),
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
    ).toEqual({ result: { id: 2, name: "b" }, rowsAffected: 1 });

    expect(
      await command.execute("update", [
        { name: "id", value: 2 },
        { name: "name", value: "B" },
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
    expect(await collectAsync(repo.get())).toEqual([]);
  });

  it("performs repository CRUD", async () => {
    const { repo } = await openRepo([{ id: 1, name: "a" }]);
    expect(await repo.getSingle()).toEqual({ id: 1, name: "a" });
    expect(await repo.getSingle(new Query('{"id":9}'))).toBeUndefined();
    expect(await collectAsync(repo.get(new Query('{"id":1}')))).toEqual([
      { id: 1, name: "a" },
    ]);
    expect(await repo.getSingle(new Where('{"id":1}'))).toEqual({ id: 1, name: "a" });
    expect(await collectAsync(repo.get(new Where('{"id":1}')))).toEqual([
      { id: 1, name: "a" },
    ]);
    expect((await repo.create({ id: 2, name: "b" })).rowsAffected).toBe(1);
    expect((await repo.update({ id: 2, name: "B" })).rowsAffected).toBe(1);
    expect(
      (await repo.update({ name: "C" }, new Where('{"id":2}'))).rowsAffected,
    ).toBe(1);
    expect((await repo.delete({ id: 1 })).rowsAffected).toBe(1);
    expect(
      (await repo.delete({ id: 2 }, new Where('{"id":2}'))).rowsAffected,
    ).toBe(1);
    expect(await collectAsync(repo.get())).toEqual([]);
  });

  it("rejects command and repository IO when closed", async () => {
    const connection = new MemoryConnection({ data: [{ id: 1 }] });
    const command = new MemoryCommand(connection);
    const repo = new MemoryRepository(connection);
    await expect(command.executeReader("")).rejects.toThrow(DatasourceError);
    await expect(command.execute("insert")).rejects.toThrow(DatasourceError);
    await expect(repo.create({ id: 2 })).rejects.toThrow(DatasourceError);
    await expect(repo.update({ id: 1 })).rejects.toThrow(DatasourceError);
    await expect(repo.delete({ id: 1 })).rejects.toThrow(DatasourceError);
  });
});

describe("MemoryDatasource", () => {
  it("registers and constructs children", async () => {
    const created = MemoryDatasource.create();
    expect(created.provider).toBe("memory");
    expect(MemoryDatasource.instance.provider).toBe("memory");
    const ds = new MemoryDatasource({ data: [{ id: 1 }] });
    expect(ds.create({ data: [] }).provider).toBe("memory");
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
    const repo = ds.createRepository(connection);
    expect(repo).toBeInstanceOf(MemoryRepository);
    expect(ds.createRepository()).toBeInstanceOf(MemoryRepository);
    expect(ds.createReadonlyRepository(connection)).toBeInstanceOf(MemoryRepository);

    const fromRegistry = Datasource.create("memory", { data: [] });
    expect(fromRegistry.provider).toBe("memory");
  });
});
