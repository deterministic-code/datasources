import { describe, expect, it } from "vitest";
import {
  MockCommand,
  MockConnection,
  MockDatasource,
  MockRepository,
  untyped,
} from "../index.ts";

describe("MockDatasource", () => {
  it("builds commands and repositories from a mock datasource", async () => {
    const ds = new MockDatasource();
    expect(ds.provider).toBe("mock");
    expect(await ds.createConnection()).toBe(ds.connection);
    const mockCommand = ds.createCommand();
    expect(mockCommand).toBeInstanceOf(MockCommand);
    await mockCommand.execute("");
    await mockCommand.executeReader("");
    const provided = new MockConnection();
    expect(ds.createCommand(provided).connection).toBe(provided);
    expect(ds.createRepository(untyped(), provided)).toBeInstanceOf(MockRepository);
    expect(ds.createRepository(untyped())).toBeInstanceOf(MockRepository);

    ds.command = new MockCommand(ds.connection);
    ds.repository = new MockRepository(ds.connection);
    expect(ds.createCommand()).toBe(ds.command);
    expect(ds.createRepository(untyped())).toBe(ds.repository);
    expect(ds.createReadonlyRepository(untyped())).toBe(ds.repository);

    const named = new MockDatasource("custom");
    expect(named.provider).toBe("custom");
  });
});
