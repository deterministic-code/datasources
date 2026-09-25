import { describe, expect, it } from "vitest";
import {
  collectAsync,
  createStubDatasource,
  ProviderNotImplementedError,
  StubCommand,
  StubConnection,
  StubDatasource,
  StubReader,
  StubRepository,
  StubTransaction,
  untyped,
  WhereText,
} from "../index.ts";

describe("StubDatasource", () => {
  it("creates stub objects that throw on IO", async () => {
    const ds = createStubDatasource("text");
    expect(ds.provider).toBe("text");
    const connection = await ds.createConnection();
    expect(connection).toBeInstanceOf(StubConnection);
    expect(connection.isOpen).toBe(false);
    expect(connection.createCommand()).toBeInstanceOf(StubCommand);
    await expect(connection.open()).rejects.toThrow(ProviderNotImplementedError);
    await expect(connection.close()).rejects.toThrow(ProviderNotImplementedError);
    await expect(connection.startTransaction()).rejects.toThrow(
      ProviderNotImplementedError,
    );

    const command = ds.createCommand();
    expect(command).toBeInstanceOf(StubCommand);
    await expect(command.execute("")).rejects.toThrow(ProviderNotImplementedError);
    await expect(command.executeReader("select", [])).rejects.toThrow(
      ProviderNotImplementedError,
    );

    const provided = new StubConnection("text");
    const bound = ds.createCommand(provided);
    expect(bound.connection).toBe(provided);

    const repo = ds.createRepository(untyped(), provided);
    expect(repo).toBeInstanceOf(StubRepository);
    await expect(repo.readSingle()).rejects.toThrow(ProviderNotImplementedError);
    await expect(repo.readSingle(new WhereText("1=1"))).rejects.toThrow(
      ProviderNotImplementedError,
    );
    await expect(collectAsync(repo.read())).rejects.toThrow(
      ProviderNotImplementedError,
    );
    await expect(collectAsync(repo.read(new WhereText("1=1")))).rejects.toThrow(
      ProviderNotImplementedError,
    );
    await expect(repo.create({})).rejects.toThrow(ProviderNotImplementedError);
    await expect(repo.update({})).rejects.toThrow(ProviderNotImplementedError);
    await expect(repo.delete({}, new WhereText("1=1"))).rejects.toThrow(
      ProviderNotImplementedError,
    );

    expect(ds.createRepository(untyped())).toBeInstanceOf(StubRepository);
    expect(ds.createReadonlyRepository(untyped(), provided)).toBeInstanceOf(StubRepository);
    expect(() => repo.readSingleById(1)).toThrow(ProviderNotImplementedError);
    await expect(collectAsync(repo.read())).rejects.toThrow(ProviderNotImplementedError);
    await expect(collectAsync(repo.read(new WhereText("1=1")))).rejects.toThrow(
      ProviderNotImplementedError,
    );
    class YieldingStub extends StubRepository<{ id: number }> {
      override async *read(): AsyncIterable<{ id: number }> {
        yield { id: 1 };
      }
    }
    expect(await collectAsync(new YieldingStub("text", provided).read())).toEqual([{ id: 1 }]);

    const txn = new StubTransaction("text", provided);
    await expect(txn.commit()).rejects.toThrow(ProviderNotImplementedError);
    await expect(txn.abort()).rejects.toThrow(ProviderNotImplementedError);

    const reader = new StubReader("text");
    expect(reader.current).toBeUndefined();
    await expect(reader.next()).rejects.toThrow(ProviderNotImplementedError);
    await expect(reader.nextResult()).rejects.toThrow(ProviderNotImplementedError);
    await reader.dispose();

    const named = new StubDatasource("yaml");
    expect(named.createCommand().connection).toBeInstanceOf(StubConnection);
  });
});
