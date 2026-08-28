import { afterEach, describe, expect, it, vi } from "vitest";
import {
  ArrayReader,
  collectAsync,
  createStubDatasource,
  Datasource,
  DatasourceError,
  identityFilter,
  identityWhereText,
  IsolationLevel,
  CommandMiddlewareHost,
  ConsoleCommandMiddleware,
  formatElapsed,
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
} from "./index.ts";

afterEach(() => {
  Datasource.reset();
});

describe("isolation", () => {
  it("names the ANSI levels", () => {
    expect(IsolationLevel).toEqual({
      ReadUncommitted: "readUncommitted",
      ReadCommitted: "readCommitted",
      RepeatableRead: "repeatableRead",
      Serializable: "serializable",
    });
  });
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
  it("rewrites the query and runs afterQuery", async () => {
    const host = new CommandMiddlewareHost();
    const seen: string[] = [];
    let used = "";
    host.use({
      beforeQuery: (query) => {
        seen.push(`before:${query}`);
        return { query: "SELECT 2" };
      },
      afterQuery: (query, _params, results, elapsedMs) => {
        seen.push(`after:${query}:${results.length}:${elapsedMs >= 0}`);
      },
    });
    const result = await host.run("SELECT 1", undefined, async (query) => {
      used = query;
      seen.push("op");
      return { value: 7, rows: [{ id: 1 }] };
    });
    expect(result).toBe(7);
    expect(used).toBe("SELECT 2");
    expect(seen[0]).toBe("before:SELECT 1");
    expect(seen[1]).toBe("op");
    expect(seen[2]?.startsWith("after:SELECT 2:1:true")).toBe(true);
  });

  it("runs with no middleware", async () => {
    const host = new CommandMiddlewareHost();
    expect(await host.run(null, undefined, async () => ({ value: "ok", rows: [] }))).toBe(
      "ok",
    );
  });

  it("skips the query when beforeQuery throws", async () => {
    const host = new CommandMiddlewareHost();
    let ran = false;
    host.use({
      beforeQuery: () => {
        throw new Error("blocked");
      },
      afterQuery: () => {
        ran = true;
      },
    });
    await expect(
      host.run("select", undefined, async () => {
        ran = true;
        return { value: 1, rows: [] };
      }),
    ).rejects.toThrow("blocked");
    expect(ran).toBe(false);
  });

  it("runs afterQuery with error when the query fails", async () => {
    const host = new CommandMiddlewareHost();
    let captured: Error | undefined;
    host.use({
      beforeQuery: () => undefined,
      afterQuery: (_q, _p, _r, _e, error) => {
        captured = error;
      },
    });
    await expect(
      host.run("select", undefined, async () => {
        throw new Error("fail-op");
      }),
    ).rejects.toThrow("fail-op");
    expect(captured?.message).toBe("fail-op");
  });

  it("wraps a non-Error throw for afterQuery", async () => {
    const host = new CommandMiddlewareHost();
    let captured: Error | undefined;
    host.use({
      beforeQuery: () => undefined,
      afterQuery: (_q, _p, _r, _e, error) => {
        captured = error;
      },
    });
    await expect(
      host.run("select", undefined, async () => {
        throw "string-fail";
      }),
    ).rejects.toBe("string-fail");
    expect(captured?.message).toBe("string-fail");
  });
});

describe("console middleware", () => {
  it("formats elapsed time as ms or μs", () => {
    expect(formatElapsed(2)).toBe("2ms");
    expect(formatElapsed(1)).toBe("1ms");
    expect(formatElapsed(0.85)).toBe("850μs");
    expect(formatElapsed(0.4)).toBe("400μs");
  });

  it("logs Start and Finish with an injected clock", async () => {
    const lines: string[] = [];
    const stamps = [
      new Date("2026-08-28T01:54:12.101Z"),
      new Date("2026-08-28T01:54:12.103Z"),
    ];
    const middleware = new ConsoleCommandMiddleware(
      (line) => lines.push(line),
      () => stamps.shift() ?? new Date("2026-08-28T01:54:12.103Z"),
    );
    const host = new CommandMiddlewareHost();
    host.use(middleware);
    await host.run('SELECT * FROM "widget" ORDER BY "id"', undefined, async () => {
      await new Promise((resolve) => setTimeout(resolve, 2));
      return { value: 1, rows: [] };
    });
    expect(lines[0]).toBe(
      '[2026-08-28T01:54:12.101Z]-[datasource][Start]-[SELECT * FROM "widget" ORDER BY "id"]',
    );
    expect(lines[1]).toMatch(
      /^\[2026-08-28T01:54:12.103Z\]-\[datasource\]\[Finish\]-\[SELECT \* FROM "widget" ORDER BY "id"\] \d+(ms|μs)$/,
    );
  });

  it("uses console.log and Date by default", async () => {
    const logged: unknown[] = [];
    const spy = vi.spyOn(console, "log").mockImplementation((...args) => {
      logged.push(args[0]);
    });
    const middleware = new ConsoleCommandMiddleware();
    middleware.beforeQuery("SELECT 1");
    middleware.afterQuery("SELECT 1", undefined, [], 2);
    expect(String(logged[0])).toMatch(
      /^\[\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}\.\d{3}Z\]-\[datasource\]\[Start\]-\[SELECT 1\]$/,
    );
    expect(String(logged[1])).toMatch(/\[Finish\]-\[SELECT 1\] 2ms$/);
    spy.mockRestore();
  });

  it("trims trailing newlines before logging", async () => {
    const lines: string[] = [];
    const host = new CommandMiddlewareHost();
    host.use(new ConsoleCommandMiddleware((line) => lines.push(line)));
    await host.run("SELECT 1\n", undefined, async () => ({ value: 1, rows: [] }));
    expect(lines[0].replace(/^\[[^\]]+\]-/, "")).toBe("[datasource][Start]-[SELECT 1]");
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

    const repo = ds.createRepository(provided);
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
    expect(ds.createReadonlyRepository(provided)).toBeInstanceOf(StubRepository);

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
    const txn = await connection.startTransaction(IsolationLevel.ReadUncommitted);
    expect(txn).toBe(connection.transaction);
    expect(txn.isolation).toBe(IsolationLevel.ReadUncommitted);
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

    command.reader = { current: undefined, next: async () => false };
    expect(await command.executeReader("plain")).toBe(command.reader);

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
    expect(ds.createRepository(provided)).toBeInstanceOf(
      MockRepository,
    );
    expect(ds.createRepository()).toBeInstanceOf(MockRepository);

    ds.command = new MockCommand(ds.connection);
    ds.repository = new MockRepository(ds.connection);
    expect(ds.createCommand()).toBe(ds.command);
    expect(ds.createRepository()).toBe(ds.repository);
    expect(ds.createReadonlyRepository()).toBe(ds.repository);

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

  it("runs command middleware on mock commands", async () => {
    const connection = new MockConnection();
    const command = new MockCommand(connection);
    const seen: string[] = [];
    command.use({
      beforeQuery: (query) => {
        seen.push(`before:${query}`);
      },
      afterQuery: (query) => {
        seen.push(`after:${query}`);
      },
    });
    await command.execute("insert");
    expect(seen).toEqual(["before:insert", "after:insert"]);
  });
});
