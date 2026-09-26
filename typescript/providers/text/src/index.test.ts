import { afterEach, describe, expect, it } from "vitest";
import { Datasource, DatasourceFactory, ProviderNotImplementedError, StubCommand, untyped } from "@datasources/core";
import { TextConnection } from "./connection.ts";
import { TextProvider } from "./TextProvider.ts";

afterEach(() => DatasourceFactory.reset());

describe("text", () => {
  it("creates", () => {
    DatasourceFactory.register("text", new TextProvider());
    expect(TextConnection.from()).toBeInstanceOf(TextConnection);
    expect(Datasource.create({ provider: "text" }).provider).toBe("text");
  });

  it("creates a named connection that throws on open", async () => {
    const ds = new Datasource("text", new TextProvider());
    const connection = await ds.createConnection();
    expect(connection).toBeInstanceOf(TextConnection);
    await expect(connection.open()).rejects.toThrow(ProviderNotImplementedError);
    const command = ds.createCommand(connection);
    expect(command).toBeInstanceOf(StubCommand);
    expect(command.connection).toBe(connection);
    await expect(command.execute("")).rejects.toThrow(ProviderNotImplementedError);
    await expect(command.executeReader("")).rejects.toThrow(ProviderNotImplementedError);
    const implicit = ds.createCommand();
    expect(implicit).toBeInstanceOf(StubCommand);
    await expect(implicit.execute("")).rejects.toThrow(ProviderNotImplementedError);
    await expect(implicit.executeReader("")).rejects.toThrow(ProviderNotImplementedError);
    ds.createRepository(untyped());
    ds.createRepository(untyped(), connection);
    ds.createReadonlyRepository(untyped());
    ds.createReadonlyRepository(untyped(), connection);
    expect(new TextProvider().transaction(connection).isolation).toBe("serializable");
  });
});
