import { afterEach, describe, expect, it } from "vitest";
import { Datasource, ProviderNotImplementedError } from "@datasources/core";
import { PostgresCommand, PostgresConnection, PostgresDatasource } from "./index.ts";

afterEach(() => Datasource.reset());

describe("postgres", () => {
  it("creates", () => {
    Datasource.use(PostgresDatasource.factory);
    expect(Datasource.create("postgres").provider).toBe("postgres");
  });

  it("creates a named connection that throws on open", async () => {
    const ds = new PostgresDatasource();
    const connection = await ds.createConnection();
    expect(connection).toBeInstanceOf(PostgresConnection);
    await expect(connection.open()).rejects.toThrow(ProviderNotImplementedError);
    const command = ds.createCommand(connection);
    expect(command).toBeInstanceOf(PostgresCommand);
    expect(command.connection).toBe(connection);
    await expect(command.execute()).rejects.toThrow(ProviderNotImplementedError);
    await expect(command.executeReader()).rejects.toThrow(ProviderNotImplementedError);
    const implicit = ds.createCommand();
    expect(implicit).toBeInstanceOf(PostgresCommand);
    await expect(implicit.execute()).rejects.toThrow(ProviderNotImplementedError);
    await expect(implicit.executeReader()).rejects.toThrow(ProviderNotImplementedError);
    ds.createRepository();
    ds.createRepository(connection, { table: "t" });
  });
});
