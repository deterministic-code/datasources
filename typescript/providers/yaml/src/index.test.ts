import { afterEach, describe, expect, it } from "vitest";
import { Datasource, ProviderNotImplementedError } from "@datasources/core";
import { YamlCommand, YamlConnection, YamlDatasource } from "./index.ts";

afterEach(() => Datasource.reset());

describe("yaml", () => {
  it("creates", () => {
    Datasource.use(YamlDatasource.instance);
    expect(Datasource.create("yaml").provider).toBe("yaml");
  });

  it("creates a named connection that throws on open", async () => {
    const ds = new YamlDatasource();
    const connection = await ds.createConnection();
    expect(connection).toBeInstanceOf(YamlConnection);
    await expect(connection.open()).rejects.toThrow(ProviderNotImplementedError);
    const command = ds.createCommand(connection);
    expect(command).toBeInstanceOf(YamlCommand);
    expect(command.connection).toBe(connection);
    await expect(command.execute()).rejects.toThrow(ProviderNotImplementedError);
    await expect(command.executeReader()).rejects.toThrow(ProviderNotImplementedError);
    const implicit = ds.createCommand();
    expect(implicit).toBeInstanceOf(YamlCommand);
    await expect(implicit.execute()).rejects.toThrow(ProviderNotImplementedError);
    await expect(implicit.executeReader()).rejects.toThrow(ProviderNotImplementedError);
    ds.createRepository();
    ds.createRepository(connection);
    ds.createReadonlyRepository();
    ds.createReadonlyRepository(connection);
  });
});
