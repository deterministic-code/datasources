import { afterEach, describe, expect, it } from "vitest";
import { Datasource, ProviderNotImplementedError } from "@datasources/core";
import { YamlConnection, YamlDatasource } from "./index.ts";

afterEach(() => Datasource.reset());

describe("yaml", () => {
  it("creates", () => {
    Datasource.use(YamlDatasource.factory);
    expect(Datasource.create("yaml").provider).toBe("yaml");
  });

  it("creates a named connection that throws on open", async () => {
    const ds = new YamlDatasource();
    const connection = await ds.createConnection();
    expect(connection).toBeInstanceOf(YamlConnection);
    await expect(connection.open()).rejects.toThrow(ProviderNotImplementedError);
    ds.createCommand();
    ds.createCommand(connection);
    ds.createRepository();
    ds.createRepository(connection, { table: "t" });
  });
});
