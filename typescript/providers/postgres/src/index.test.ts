import { afterEach, describe, expect, it } from "vitest";
import { Datasource, ProviderNotImplementedError } from "@datasources/core";
import { PostgresConnection, PostgresDatasource } from "./index.ts";

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
    ds.createCommand();
    ds.createCommand(connection);
    ds.createRepository();
    ds.createRepository(connection, { table: "t" });
  });
});
