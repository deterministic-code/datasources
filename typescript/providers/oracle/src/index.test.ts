import { afterEach, describe, expect, it } from "vitest";
import { Datasource, ProviderNotImplementedError } from "@datasources/core";
import { OracleConnection, OracleDatasource } from "./index.ts";

afterEach(() => Datasource.reset());

describe("oracle", () => {
  it("creates", () => {
    Datasource.use(OracleDatasource.factory);
    expect(Datasource.create("oracle").provider).toBe("oracle");
  });

  it("creates a named connection that throws on open", async () => {
    const ds = new OracleDatasource();
    const connection = await ds.createConnection();
    expect(connection).toBeInstanceOf(OracleConnection);
    await expect(connection.open()).rejects.toThrow(ProviderNotImplementedError);
    ds.createCommand();
    ds.createCommand(connection);
    ds.createRepository();
    ds.createRepository(connection, { table: "t" });
  });
});
