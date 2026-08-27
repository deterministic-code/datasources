import { afterEach, describe, expect, it } from "vitest";
import {
  ArrayReader,
  collectAsync,
  createStubDatasource,
  Datasource,
  DatasourceError,
  identityFilter,
  identityWhereText,
  MiddlewareHost,
  MockCommand,
  MockConnection,
  MockDatasource,
  MockQuery,
  MockReader,
  MockRepository,
  MockTransaction,
  MockWhere,
  parametersToRecord,
  ProviderNotImplementedError,
  ProviderNotInstalledError,
  Query,
  readerToAsyncIterable,
  StubCommand,
  StubConnection,
  StubDatasource,
  StubReader,
  StubRepository,
  StubTransaction,
  Where,
  type IReader,
  type MiddlewareContext,
} from "./index.ts";

afterEach(() => {
  Datasource.reset();
});

describe("errors", () => {
  it("reports a missing provider", () => {
    const error = new ProviderNotInstalledError("mysql");
    expect(error.name).toBe("ProviderNotInstalledError");
    expect(error.provider).toBe("mysql");
    expect(error.message).toContain("mysql");
  });

  it("reports an unimplemented provider", () => {
    const error = new ProviderNotImplementedError("oracle");
    expect(error.name).toBe("ProviderNotImplementedError");
    expect(error.provider).toBe("oracle");
    expect(error.message).toContain("oracle");
  });

  it("wraps a generic datasource error", () => {
    const error = new DatasourceError("broken");
    expect(error.name).toBe("DatasourceError");
    expect(error.message).toBe("broken");
  });
});

describe("parametersToRecord", () => {
  it("returns an empty record when parameters are omitted", () => {
    expect(parametersToRecord()).toEqual({});
    expect(parametersToRecord(null)).toEqual({});
    expect(parametersToRecord(undefined)).toEqual({});
  });

  it("maps names to values", () => {
    expect(
      parametersToRecord([
        { name: "id", value: 1 },
        { name: "label", value: "a" },
      ]),
    ).toEqual({ id: 1, label: "a" });
  });
});

describe("Query and Where", () => {
  it("returns constructor text", () => {
    expect(new Query("SELECT 1").getText()).toBe("SELECT 1");
    expect(new Where("id = 1").getText()).toBe("id = 1");
  });
});

describe("identity helpers", () => {
  it("filters by id when present", () => {
    expect(identityFilter({ id: 9, name: "a" })).toEqual({ id: 9 });
    expect(identityWhereText({ id: 9, name: "a" })).toBe("id = $id");
  });

  it("copies the whole object when id is missing", () => {
    expect(identityFilter({ name: "a" })).toEqual({ name: "a" });
    expect(identityWhereText({ name: "a", city: "x" })).toBe(
      "name = $name AND city = $city",
    );
  });

  it("matches everything when the object has no keys", () => {
    expect(identityWhereText({})).toBe("1 = 1");
  });
});

describe("readers", () => {
  it("walks an array including the empty case", async () => {
    const empty = new ArrayReader([]);
    expect(empty.current).toBeUndefined();
    expect(await empty.next()).toBe(false);
    expect(empty.current).toBeUndefined();

    const reader = new ArrayReader([{ id: 1 }, { id: 2 }]);
    expect(await reader.next()).toBe(true);
    expect(reader.current).toEqual({ id: 1 });
    expect(await reader.next()).toBe(true);
    expect(reader.current).toEqual({ id: 2 });
    expect(await reader.next()).toBe(false);
    expect(reader.current).toBeUndefined();
  });

  it("converts a reader to an async iterable", async () => {
    const items = await collectAsync(
      readerToAsyncIterable(new ArrayReader(["a", "b"])),
    );
    expect(items).toEqual(["a", "b"]);
    expect(await collectAsync(readerToAsyncIterable(new ArrayReader([])))).toEqual(
      [],
    );
  });

  it("skips a defined-next row whose current is undefined", async () => {
    const reader: IReader<string> = {
      current: undefined,
      async next() {
        if (this.current === undefined && (this as { once?: boolean }).once) {
          return false;
        }
        (this as { once?: boolean }).once = true;
        this.current = undefined;
        return true;
      },
    };
    expect(await collectAsync(readerToAsyncIterable(reader))).toEqual([]);
  });
});

describe("middleware", () => {
  it("runs before then the operation then after", async () => {
    const host = new MiddlewareHost();
    const order: string[] = [];
    const contexts: MiddlewareContext[] = [];
    host.use(
      (ctx) => {
        order.push(`before:${ctx.operation}`);
        contexts.push({ ...ctx });
      },
      (ctx) => {
        order.push(`after:${ctx.operation}`);
        contexts.push({ ...ctx });
      },
    );
    const result = await host.run("open", async () => {
      order.push("op");
      return 7;
    });
    expect(result).toBe(7);
    expect(order).toEqual(["before:open", "op", "after:open"]);
    expect(contexts[1]?.error).toBeUndefined();
  });

  it("accepts use() with neither hook", async () => {
    const host = new MiddlewareHost();
    expect(host.use()).toBe(host);
    expect(await host.run("close", async () => "ok")).toBe("ok");
  });

  it("accepts before-only and after-only hooks", async () => {
    const host = new MiddlewareHost();
    const seen: string[] = [];
    host.use((ctx) => {
      seen.push(`b:${ctx.operation}`);
    });
    host.use(undefined, (ctx) => {
      seen.push(`a:${ctx.operation}`);
    });
    await host.run("commit", async () => undefined);
    expect(seen).toEqual(["b:commit", "a:commit"]);
  });

  it("skips the operation when before throws", async () => {
    const host = new MiddlewareHost();
    let ran = false;
    host.use(async () => {
      throw new Error("blocked");
    }, () => {
      ran = true;
    });
    await expect(
      host.run("execute", async () => {
        ran = true;
        return 1;
      }),
    ).rejects.toThrow("blocked");
    expect(ran).toBe(false);
  });

  it("runs after with error when the operation fails", async () => {
    const host = new MiddlewareHost();
    let captured: unknown;
    host.use(undefined, (ctx) => {
      captured = ctx.error;
    });
    await expect(
      host.run("abort", async () => {
        throw new Error("fail-op");
      }),
    ).rejects.toThrow("fail-op");
    expect(captured).toBeInstanceOf(Error);
    expect((captured as Error).message).toBe("fail-op");
  });

  it("forwards extras onto the context", async () => {
    const host = new MiddlewareHost();
    let ctx: MiddlewareContext | undefined;
    host.use((c) => {
      ctx = c;
    });
    await host.run("execute", async () => 1, {
      text: "insert",
      parameters: [{ name: "id", value: 1 }],
    });
    expect(ctx?.text).toBe("insert");
    expect(ctx?.parameters).toEqual([{ name: "id", value: 1 }]);
    expect(ctx?.host).toBe(host);
  });
});

describe("Datasource registry", () => {
  it("registers, lists, creates, and resets", () => {
    const factory = new MockDatasource("json");
    expect(Datasource.registered()).toEqual([]);
    Datasource.use(factory, new MockDatasource("memory"));
    expect(Datasource.registered()).toEqual(["json", "memory"]);
    const created = Datasource.create("json", { path: "x" });
    expect(created.provider).toBe("json");
    Datasource.reset();
    expect(Datasource.registered()).toEqual([]);
    expect(() => Datasource.create("json")).toThrow(ProviderNotInstalledError);
  });
});

describe("stubs", () => {
  it("creates stub objects that throw on IO", async () => {
    const ds = createStubDatasource("text");
    expect(ds.provider).toBe("text");
    expect(ds.create().provider).toBe("text");
    const connection = await ds.createConnection();
    expect(connection).toBeInstanceOf(StubConnection);
    await expect(connection.open()).rejects.toThrow(ProviderNotImplementedError);
    await expect(connection.close()).rejects.toThrow(ProviderNotImplementedError);
    await expect(connection.startTransaction()).rejects.toThrow(
      ProviderNotImplementedError,
    );

    const command = ds.createCommand();
    expect(command).toBeInstanceOf(StubCommand);
    await expect(command.execute()).rejects.toThrow(ProviderNotImplementedError);
    await expect(command.executeReader("select", [])).rejects.toThrow(
      ProviderNotImplementedError,
    );

    const provided = new StubConnection("text");
    const bound = ds.createCommand(provided);
    expect(bound.connection).toBe(provided);

    const repo = ds.createRepository(provided, { table: "items" });
    expect(repo).toBeInstanceOf(StubRepository);
    await expect(repo.getSingle()).rejects.toThrow(ProviderNotImplementedError);
    await expect(collectAsync(repo.get())).rejects.toThrow(
      ProviderNotImplementedError,
    );
    await expect(repo.add({})).rejects.toThrow(ProviderNotImplementedError);
    await expect(repo.update({})).rejects.toThrow(ProviderNotImplementedError);
    await expect(repo.delete({}, new Where("1=1"))).rejects.toThrow(
      ProviderNotImplementedError,
    );

    const unbound = ds.createRepository();
    expect(unbound.connection).toBeInstanceOf(StubConnection);

    const txn = new StubTransaction("text", provided);
    await expect(txn.commit()).rejects.toThrow(ProviderNotImplementedError);
    await expect(txn.abort()).rejects.toThrow(ProviderNotImplementedError);

    const reader = new StubReader("text");
    expect(reader.current).toBeUndefined();
    await expect(reader.next()).rejects.toThrow(ProviderNotImplementedError);

    const named = new StubDatasource("yaml");
    expect(named.createCommand().connection).toBeInstanceOf(StubConnection);
  });
});

describe("published mocks", () => {
  it("records connection operations and errors", async () => {
    const connection = new MockConnection();
    await connection.open();
    expect(connection.openCalls).toBe(1);
    expect(connection.opened).toBe(true);
    const txn = await connection.startTransaction();
    expect(txn).toBe(connection.transaction);
    await connection.close();
    expect(connection.opened).toBe(false);

    connection.openError = new Error("open-fail");
    await expect(connection.open()).rejects.toThrow("open-fail");
    connection.closeError = new Error("close-fail");
    await expect(connection.close()).rejects.toThrow("close-fail");
    connection.startTransactionError = new Error("tx-fail");
    await expect(connection.startTransaction()).rejects.toThrow("tx-fail");

    const customTxn = new MockTransaction(connection);
    const withTxn = new MockConnection(customTxn);
    expect(withTxn.transaction).toBe(customTxn);
  });

  it("records transaction commit and abort", async () => {
    const connection = new MockConnection();
    const txn = new MockTransaction(connection);
    await txn.commit();
    expect(txn.committed).toBe(true);
    await txn.abort();
    expect(txn.aborted).toBe(true);
    txn.commitError = new Error("c");
    await expect(txn.commit()).rejects.toThrow("c");
    txn.abortError = new Error("a");
    await expect(txn.abort()).rejects.toThrow("a");
  });

  it("records command execute paths", async () => {
    const connection = new MockConnection();
    const command = new MockCommand(connection);
    command.response = { result: 1, rowsAffected: 2 };
    command.reader = new MockReader([{ id: 1 }]);
    expect(await command.execute("insert", [{ name: "id", value: 1 }])).toEqual({
      result: 1,
      rowsAffected: 2,
    });
    expect(command.executeCalls).toHaveLength(1);
    const reader = await command.executeReader();
    expect(reader).toBe(command.reader);

    command.executeError = new Error("exec");
    await expect(command.execute()).rejects.toThrow("exec");
    command.executeReaderError = new Error("read");
    await expect(command.executeReader("x")).rejects.toThrow("read");
  });

  it("walks mock reader rows and errors", async () => {
    const reader = new MockReader(["x"]);
    expect(await reader.next()).toBe(true);
    expect(reader.current).toBe("x");
    expect(await reader.next()).toBe(false);
    reader.nextError = new Error("next");
    await expect(reader.next()).rejects.toThrow("next");
  });

  it("records repository CRUD and errors", async () => {
    const repo = new MockRepository<{ id: number }>(new MockConnection());
    expect(await repo.getSingle()).toBeUndefined();
    repo.items = [{ id: 1 }, { id: 2 }];
    expect(await repo.getSingle(new MockQuery("id=1"))).toEqual({ id: 1 });
    expect(await collectAsync(repo.get())).toEqual([{ id: 1 }, { id: 2 }]);
    expect(await collectAsync(repo.get(new MockQuery("all")))).toHaveLength(2);
    expect((await repo.add({ id: 3 })).rowsAffected).toBe(1);
    expect(
      (await repo.update({ id: 3 }, new MockWhere("id=3"))).rowsAffected,
    ).toBe(1);
    expect((await repo.delete({ id: 3 })).rowsAffected).toBe(1);

    repo.getSingleError = new Error("gs");
    await expect(repo.getSingle()).rejects.toThrow("gs");
    repo.getError = new Error("g");
    await expect(collectAsync(repo.get())).rejects.toThrow("g");
    repo.addError = new Error("ad");
    await expect(repo.add({ id: 0 })).rejects.toThrow("ad");
    repo.updateError = new Error("up");
    await expect(repo.update({ id: 0 })).rejects.toThrow("up");
    repo.deleteError = new Error("del");
    await expect(repo.delete({ id: 0 })).rejects.toThrow("del");
  });

  it("builds commands and repositories from a mock datasource", async () => {
    const ds = new MockDatasource();
    expect(ds.create().provider).toBe("mock");
    expect(await ds.createConnection()).toBe(ds.connection);
    const mockCommand = ds.createCommand();
    expect(mockCommand).toBeInstanceOf(MockCommand);
    await mockCommand.execute();
    await mockCommand.executeReader();
    const provided = new MockConnection();
    expect(ds.createCommand(provided).connection).toBe(provided);
    expect(ds.createRepository(provided, { table: "t" })).toBeInstanceOf(
      MockRepository,
    );
    expect(ds.createRepository()).toBeInstanceOf(MockRepository);

    ds.command = new MockCommand(ds.connection);
    ds.repository = new MockRepository(ds.connection);
    expect(ds.createCommand()).toBe(ds.command);
    expect(ds.createRepository()).toBe(ds.repository);

    const named = new MockDatasource("custom");
    expect(named.create().provider).toBe("custom");
  });

  it("counts mock query and where text access", () => {
    const query = new MockQuery("q");
    const where = new MockWhere("w");
    expect(query.getText()).toBe("q");
    expect(where.getText()).toBe("w");
    expect(query.getTextCalls).toBe(1);
    expect(where.getTextCalls).toBe(1);
  });

  it("runs middleware on mock hosts", async () => {
    const connection = new MockConnection();
    const seen: string[] = [];
    connection.use(
      () => {
        seen.push("before");
      },
      () => {
        seen.push("after");
      },
    );
    await connection.open();
    expect(seen).toEqual(["before", "after"]);
  });
});
