import { afterEach, describe, expect, it, vi } from "vitest";

const readFile = vi.fn(async () => "[]");
const writeFile = vi.fn(async () => undefined);
const access = vi.fn(async () => undefined);

vi.mock("node:fs/promises", () => ({
  readFile: (...args: unknown[]) => readFile(...args),
  writeFile: (...args: unknown[]) => writeFile(...args),
  access: (...args: unknown[]) => access(...args),
}));
import {
  collectAsync,
  CRUDRepository,
  ReadonlyRepository,
  untyped,
  Datasource,
  DatasourceFactory,
  DatasourceError,
  InsertStatement,
  QueryStatement,
  UpdateStatement,
  WhereText,
} from "@datasources/core";
import { JsonCommand } from "./command.ts";
import { JsonConnection } from "./connection.ts";
import { asRecord, matches, parametersToObject, parseFilter } from "./filter.ts";
import type { FileSystemApi } from "./i-file-store.ts";
import { JsonProvider } from "./JsonProvider.ts";
import { MemoryFileStore } from "./memory-file-store.ts";
import { NodeFileStore } from "./node-file-store.ts";

afterEach(() => {
  DatasourceFactory.reset();
  DatasourceFactory.register("json", new JsonProvider());
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
    expect(() => parseFilter("null")).toThrow("JSON filter");
    expect(() => parseFilter("[]")).toThrow("JSON filter");
    expect(() => parseFilter("1")).toThrow("JSON filter");
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

describe("file stores", () => {
  it("stores files in memory", async () => {
    const store = new MemoryFileStore();
    expect(await store.exists("a.json")).toBe(false);
    await expect(store.read("a.json")).rejects.toThrow("File not found");
    await store.write("a.json", "[]");
    expect(await store.exists("a.json")).toBe(true);
    expect(await store.read("a.json")).toBe("[]");
  });

  it("accepts a seeded map", async () => {
    const store = new MemoryFileStore(new Map([["b.json", "{}"]]));
    expect(await store.read("b.json")).toBe("{}");
  });

  it("delegates to an injected filesystem", async () => {
    const files = new Map<string, string>();
    const fs: FileSystemApi = {
      async readFile(path) {
        return files.get(path) ?? "";
      },
      async writeFile(path, contents) {
        files.set(path, contents);
      },
      async access(path) {
        if (!files.has(path)) {
          throw new Error("missing");
        }
      },
    };
    const store = new NodeFileStore(fs);
    expect(await store.exists("missing.json")).toBe(false);
    await store.write("ok.json", "[]");
    expect(await store.exists("ok.json")).toBe(true);
    expect(await store.read("ok.json")).toBe("[]");
  });
});

describe("JsonConnection", () => {
  it("opens from memory data and is idempotent", async () => {
    const connection = new JsonConnection({
      data: [{ id: 1 }],
    });
    await connection.open();
    await connection.open();
    expect(connection.getCollection()).toEqual([{ id: 1 }]);
    await connection.close();
    expect(connection.isOpen).toBe(false);
  });

  it("loads an existing file and starts empty when missing", async () => {
    const store = new MemoryFileStore(
      new Map([["items.json", JSON.stringify([{ id: 2 }])]]),
    );
    const existing = new JsonConnection({ path: "items.json", fileStore: store });
    await existing.open();
    expect(existing.getCollection()).toEqual([{ id: 2 }]);

    const missing = new JsonConnection({
      path: "none.json",
      fileStore: new MemoryFileStore(),
    });
    await missing.open();
    expect(missing.getCollection()).toEqual([]);
  });

  it("uses an object root when a collection is configured", async () => {
    const connection = new JsonConnection({ collection: "users" });
    await connection.open();
    expect(connection.data).toEqual({ users: [] });
    connection.data = null;
    expect(connection.getCollection()).toEqual([]);
    connection.data = [];
    connection.collection = "users";
    expect(connection.getCollection()).toEqual([]);
    connection.data = { users: "bad" };
    expect(connection.getCollection()).toEqual([]);

    const fromFile = new JsonConnection({
      path: "c.json",
      collection: "users",
      fileStore: new MemoryFileStore(new Map([["c.json", "{}"]])),
    });
    await fromFile.open();
    expect(fromFile.data).toEqual({ users: [] });

    const fromData = new JsonConnection({ collection: "users", data: {} });
    await fromData.open();
    expect(fromData.data).toEqual({ users: [] });
  });

  it("coerces a non-array root without a collection", async () => {
    const connection = new JsonConnection();
    await connection.open();
    connection.data = { nope: true };
    expect(connection.getCollection()).toEqual([]);
  });

  it("skips persist during a transaction and without a path", async () => {
    const store = new MemoryFileStore();
    const connection = new JsonConnection({
      path: "t.json",
      fileStore: store,
      data: [{ id: 1 }],
    });
    await connection.open();
    const txn = await connection.startTransaction();
    connection.getCollection().push({ id: 2 });
    await connection.persist();
    expect(store.files.has("t.json")).toBe(false);
    await txn.commit();
    expect(JSON.parse(store.files.get("t.json") ?? "")).toEqual([
      { id: 1 },
      { id: 2 },
    ]);

    const memory = new JsonConnection({ data: [] });
    await memory.open();
    memory.getCollection().push({ id: 1 });
    await memory.persist();
  });

  it("aborts a transaction and rejects IO when closed", async () => {
    const connection = new JsonConnection({ data: [{ id: 1 }] });
    await expect(connection.startTransaction()).rejects.toThrow(DatasourceError);
    await connection.open();
    const txn = await connection.startTransaction();
    connection.getCollection().push({ id: 2 });
    await txn.abort();
    expect(connection.getCollection()).toEqual([{ id: 1 }]);

    const noPath = new JsonConnection({ data: [] });
    await noPath.open();
    const committed = await noPath.startTransaction();
    await committed.commit();
    await connection.close();
    expect(() => connection.ensureOpen()).toThrow(DatasourceError);
  });
});

describe("JsonCommand and CRUDRepository", () => {
  async function openRepo(data: Record<string, string | number | boolean>[] = []) {
    const store = new MemoryFileStore();
    const connection = new JsonConnection({
      path: "db.json",
      fileStore: store,
      data,
    });
    await connection.open();
    return {
      connection,
      store,
      command: new JsonCommand(connection),
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
      result: { id: 2, name: "b" },
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
      ]),
    ).toMatchObject({ rowsAffected: 1 });

    expect(
      await command.execute("update", [
        { name: "name", value: "Z" },
        { name: "$where", value: '{"id":1}' },
      ]),
    ).toMatchObject({ rowsAffected: 1 });

    expect(
      await command.execute("delete", [{ name: "id", value: 2 }]),
    ).toMatchObject({ rowsAffected: 1 });
    expect(await command.execute("delete")).toMatchObject({
      rowsAffected: 1,
    });
    await expect(command.execute("nope")).rejects.toThrow("Unsupported JSON");
    await expect(command.execute("")).rejects.toThrow("Unsupported JSON");
    expect(await collectAsync(repo.read())).toEqual([]);
  });

  it("performs repository CRUD", async () => {
    const { repo } = await openRepo([{ id: 1, name: "a" }]);
    expect(await repo.readSingle()).toEqual({ id: 1, name: "a" });
    expect(await repo.readSingle(new QueryStatement('{"id":9}'))).toBeUndefined();
    expect(await collectAsync(repo.read(new QueryStatement('{"id":1}')))).toEqual([
      { id: 1, name: "a" },
    ]);
    expect(await repo.readSingle(new WhereText('{"id":1}'))).toEqual({ id: 1, name: "a" });
    expect(await collectAsync(repo.read(new WhereText('{"id":1}')))).toEqual([
      { id: 1, name: "a" },
    ]);
    expect((await repo.create({ id: 2, name: "b" })).rowsAffected).toBe(1);
    expect(
      (await repo.update({ id: 2, name: "B" })).rowsAffected,
    ).toBe(1);
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
    const connection = new JsonConnection({ data: [{ id: 1 }] });
    const command = new JsonCommand(connection);
    const repo = new CRUDRepository(connection, {});
    await expect(command.executeReader("")).rejects.toThrow(DatasourceError);
    await expect(command.execute("insert")).rejects.toThrow(DatasourceError);
    await expect(repo.create({ id: 2 })).rejects.toThrow(DatasourceError);
    await expect(repo.update({ id: 1 })).rejects.toThrow(DatasourceError);
    await expect(repo.delete({ id: 1 })).rejects.toThrow(DatasourceError);
  });
});

describe("Json datasource", () => {
  it("registers and constructs children", async () => {
    const ds = new Datasource(
      "json",
      new JsonProvider(),
      { data: [{ id: 1 }] },
    );
    expect(ds.provider).toBe("json");
    const connection = (await ds.createConnection()) as JsonConnection;
    await connection.open();
    expect(connection.createCommand()).toBeInstanceOf(JsonCommand);
    const command = ds.createCommand(connection);
    expect(command).toBeInstanceOf(JsonCommand);
    expect(command.connection).toBe(connection);
    await command.execute("insert", [{ name: "id", value: 99 }]);
    await command.executeReader("");
    const implicit = ds.createCommand();
    expect(implicit).toBeInstanceOf(JsonCommand);
    expect(implicit.connection).toBeInstanceOf(JsonConnection);
    await implicit.connection.open();
    await implicit.execute("insert", [{ name: "id", value: 1 }]);
    await implicit.executeReader("");
    const repo = ds.createRepository(untyped(), connection);
    expect(repo).toBeInstanceOf(CRUDRepository);
    expect(ds.createRepository(untyped())).toBeInstanceOf(CRUDRepository);
    expect(ds.createReadonlyRepository(untyped(), connection).constructor).toBe(ReadonlyRepository);
    expect(JsonConnection.from().path).toBe("");
    expect(JsonConnection.from({ path: "db.json" }).path).toBe("db.json");
    const registered = Datasource.create({ provider: "json" });
    expect(registered.provider).toBe("json");
    expect(await registered.createConnection()).toBeInstanceOf(JsonConnection);
    expect(registered.createRepository(untyped())).toBeInstanceOf(CRUDRepository);
    const provider = new JsonProvider();
    expect(provider.transaction(connection).isolation).toBe("serializable");
    await expect(command.executeStatement(new QueryStatement("nope"))).rejects.toThrow(
      "Unsupported JSON",
    );
    await command.executeReaderStatement(new InsertStatement("t", []));
    await command.executeStatement(
      new UpdateStatement("t", [{ name: "name", value: "Z" }], new WhereText('{"id":99}')),
    );
  });
});

describe("NodeFileStore default fs", () => {
  it("uses node:fs/promises when no api is injected", async () => {
    access.mockRejectedValueOnce(new Error("nope"));
    const store = new NodeFileStore();
    expect(await store.exists("anything")).toBe(false);
    access.mockResolvedValueOnce(undefined);
    expect(await store.exists("present")).toBe(true);
    expect(await store.read("x")).toBe("[]");
    await store.write("x", "{}");
    expect(writeFile).toHaveBeenCalled();
    expect(readFile).toHaveBeenCalled();
  });
});
