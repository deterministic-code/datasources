import { afterEach, describe, expect, it } from "vitest";
import { Datasource, DatasourceFactory, ProviderNotImplementedError } from "@datasources/core";
import { SqlServerCommand, SqlServerConnection } from "./index.ts";

afterEach(() => DatasourceFactory.reset());

describe("sqlserver", () => {
  it("creates", () => {
    DatasourceFactory.register("sqlserver", SqlServerConnection.from, SqlServerConnection.repository);
    expect(SqlServerConnection.from()).toBeInstanceOf(SqlServerConnection);
    expect(Datasource.create({ provider: "sqlserver" }).provider).toBe("sqlserver");
  });

  it("creates a named connection that throws on open", async () => {
    const ds = new Datasource("sqlserver", () => new SqlServerConnection(), SqlServerConnection.repository);
    const connection = await ds.createConnection();
    expect(connection).toBeInstanceOf(SqlServerConnection);
    await expect(connection.open()).rejects.toThrow(ProviderNotImplementedError);
    const command = ds.createCommand(connection);
    expect(command).toBeInstanceOf(SqlServerCommand);
    expect(command.connection).toBe(connection);
    await expect(command.execute("")).rejects.toThrow(ProviderNotImplementedError);
    await expect(command.executeReader("")).rejects.toThrow(ProviderNotImplementedError);
    const implicit = ds.createCommand();
    expect(implicit).toBeInstanceOf(SqlServerCommand);
    await expect(implicit.execute("")).rejects.toThrow(ProviderNotImplementedError);
    await expect(implicit.executeReader("")).rejects.toThrow(ProviderNotImplementedError);
    ds.createRepository();
    ds.createRepository(connection);
    ds.createReadonlyRepository();
    ds.createReadonlyRepository(connection);
  });
});
