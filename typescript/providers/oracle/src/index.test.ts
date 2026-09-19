import { afterEach, describe, expect, it } from "vitest";
import { Datasource, DatasourceFactory, ProviderNotImplementedError } from "@datasources/core";
import { OracleCommand, OracleConnection } from "./index.ts";

afterEach(() => DatasourceFactory.reset());

describe("oracle", () => {
  it("creates", () => {
    DatasourceFactory.register("oracle", OracleConnection.from, OracleConnection.repository);
    expect(OracleConnection.from()).toBeInstanceOf(OracleConnection);
    expect(Datasource.create({ provider: "oracle" }).provider).toBe("oracle");
  });

  it("creates a named connection that throws on open", async () => {
    const ds = new Datasource("oracle", () => new OracleConnection(), OracleConnection.repository);
    const connection = await ds.createConnection();
    expect(connection).toBeInstanceOf(OracleConnection);
    await expect(connection.open()).rejects.toThrow(ProviderNotImplementedError);
    const command = ds.createCommand(connection);
    expect(command).toBeInstanceOf(OracleCommand);
    expect(command.connection).toBe(connection);
    await expect(command.execute("")).rejects.toThrow(ProviderNotImplementedError);
    await expect(command.executeReader("")).rejects.toThrow(ProviderNotImplementedError);
    const implicit = ds.createCommand();
    expect(implicit).toBeInstanceOf(OracleCommand);
    await expect(implicit.execute("")).rejects.toThrow(ProviderNotImplementedError);
    await expect(implicit.executeReader("")).rejects.toThrow(ProviderNotImplementedError);
    ds.createRepository();
    ds.createRepository(connection);
    ds.createReadonlyRepository();
    ds.createReadonlyRepository(connection);
  });
});
