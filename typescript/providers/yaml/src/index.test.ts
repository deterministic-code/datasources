import { afterEach, describe, expect, it } from "vitest";
import { Datasource, DatasourceFactory, ProviderNotImplementedError } from "@datasources/core";
import { YamlCommand, YamlConnection } from "./index.ts";

afterEach(() => DatasourceFactory.reset());

describe("yaml", () => {
  it("creates", () => {
    DatasourceFactory.register("yaml", YamlConnection.from, YamlConnection.repository);
    expect(YamlConnection.from()).toBeInstanceOf(YamlConnection);
    expect(Datasource.create({ provider: "yaml" }).provider).toBe("yaml");
  });

  it("creates a named connection that throws on open", async () => {
    const ds = new Datasource("yaml", () => new YamlConnection(), YamlConnection.repository);
    const connection = await ds.createConnection();
    expect(connection).toBeInstanceOf(YamlConnection);
    await expect(connection.open()).rejects.toThrow(ProviderNotImplementedError);
    const command = ds.createCommand(connection);
    expect(command).toBeInstanceOf(YamlCommand);
    expect(command.connection).toBe(connection);
    await expect(command.execute("")).rejects.toThrow(ProviderNotImplementedError);
    await expect(command.executeReader("")).rejects.toThrow(ProviderNotImplementedError);
    const implicit = ds.createCommand();
    expect(implicit).toBeInstanceOf(YamlCommand);
    await expect(implicit.execute("")).rejects.toThrow(ProviderNotImplementedError);
    await expect(implicit.executeReader("")).rejects.toThrow(ProviderNotImplementedError);
    ds.createRepository();
    ds.createRepository(connection);
    ds.createReadonlyRepository();
    ds.createReadonlyRepository(connection);
  });
});
