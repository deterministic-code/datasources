import { afterEach, describe, expect, it } from "vitest";
import { Datasource, DatasourceFactory, Native, ProviderNotImplementedError, StubCommand, untyped } from "@datasources/core";
import { SqlServerConnection } from "./connection.ts";
import { SqlServerDriver } from "./SqlServerDriver.ts";

afterEach(() => DatasourceFactory.reset());

describe("sqlserver", () => {
  it("creates", () => {
    DatasourceFactory.register("sqlserver", new SqlServerDriver());
    expect(SqlServerConnection.from()).toBeInstanceOf(SqlServerConnection);
    expect(Datasource.create({ PROVIDER: "sqlserver" }).provider).toBe("sqlserver");
  });

  it("creates a named connection that throws on open", async () => {
    const ds = new Datasource("sqlserver", new SqlServerDriver());
    const connection = await ds.createConnection();
    expect(connection).toBeInstanceOf(SqlServerConnection);
    await expect(connection.open()).rejects.toThrow(ProviderNotImplementedError);
    const command = await ds.createCommand(connection);
    expect(command).toBeInstanceOf(StubCommand);
    expect(command.connection).toBe(connection);
    await expect(command.executeNative(new Native("x"))).rejects.toThrow(ProviderNotImplementedError);
    await expect(command.executeReaderNative(new Native("x"))).rejects.toThrow(ProviderNotImplementedError);
    await expect(ds.createCommand()).rejects.toThrow(ProviderNotImplementedError);
    ds.createRepository(untyped());
    ds.createRepository(untyped(), connection);
    ds.createReadonlyRepository(untyped());
    ds.createReadonlyRepository(untyped(), connection);
    expect(new SqlServerDriver().transaction(connection).isolation).toBe("readCommitted");
  });
});
