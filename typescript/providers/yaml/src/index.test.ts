import { afterEach, describe, expect, it } from "vitest";
import { Datasource, DatasourceFactory, ProviderNotImplementedError, StubCommand, untyped } from "@datasources/core";
import { YamlConnection } from "./connection.ts";
import { YamlDriver } from "./YamlDriver.ts";

afterEach(() => DatasourceFactory.reset());

describe("yaml", () => {
  it("creates", () => {
    DatasourceFactory.register("yaml", new YamlDriver());
    expect(YamlConnection.from()).toBeInstanceOf(YamlConnection);
    expect(Datasource.create({ PROVIDER: "yaml" }).provider).toBe("yaml");
  });

  it("creates a named connection that throws on open", async () => {
    const ds = new Datasource("yaml", new YamlDriver());
    const connection = await ds.createConnection();
    expect(connection).toBeInstanceOf(YamlConnection);
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
    expect(new YamlDriver().transaction(connection).isolation).toBe("readCommitted");
  });
});
