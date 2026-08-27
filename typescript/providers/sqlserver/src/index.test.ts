import { afterEach, describe, expect, it } from "vitest";
import { Datasource, ProviderNotImplementedError } from "@datasources/core";
import { SqlServerConnection, SqlServerDatasource } from "./index.ts";

afterEach(() => Datasource.reset());

describe("sqlserver", () => {
  it("creates", () => {
    Datasource.use(SqlServerDatasource.factory);
    expect(Datasource.create("sqlserver").provider).toBe("sqlserver");
  });

  it("creates a named connection that throws on open", async () => {
    const ds = new SqlServerDatasource();
    const connection = await ds.createConnection();
    expect(connection).toBeInstanceOf(SqlServerConnection);
    await expect(connection.open()).rejects.toThrow(ProviderNotImplementedError);
    ds.createCommand();
    ds.createCommand(connection);
    ds.createRepository();
    ds.createRepository(connection, { table: "t" });
  });
});
