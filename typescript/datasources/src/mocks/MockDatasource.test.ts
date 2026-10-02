import { describe, expect, it } from "vitest";
import {
  MockCommand,
  MockConnection,
  MockDatasource,
  MockRepository,
  Native,
  untyped,
} from "../index.ts";

describe("MockDatasource", () => {
  it("builds commands and repositories from a mock datasource", async () => {
    const ds = new MockDatasource();
    expect(ds.provider).toBe("mock");
    expect(await ds.createConnection()).toBe(ds.connection);
    const mockCommand = await ds.createCommand();
    expect(mockCommand).toBeInstanceOf(MockCommand);
    await mockCommand.executeNative(new Native(""));
    await mockCommand.executeReaderNative(new Native(""));
    const provided = new MockConnection();
    expect((await ds.createCommand(provided)).connection).toBe(provided);
    expect(ds.createRepository(untyped(), provided)).toBeInstanceOf(MockRepository);
    expect(ds.createRepository(untyped())).toBeInstanceOf(MockRepository);

    await ds.connection.close();
    ds.command = new MockCommand(ds.connection);
    ds.repository = new MockRepository(ds.connection);
    expect(await ds.createCommand()).toBe(ds.command);
    await expect(ds.createCommand()).rejects.toThrow("Connection is already open");
    expect(ds.createRepository(untyped())).toBe(ds.repository);
    expect(ds.createReadonlyRepository(untyped())).toBe(ds.repository);

    const named = new MockDatasource("custom");
    expect(named.provider).toBe("custom");
  });
});
