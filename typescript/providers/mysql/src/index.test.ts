import { afterEach, describe, expect, it } from "vitest";
import { Datasource, ProviderNotImplementedError } from "@datasources/core";
import { MysqlConnection, MysqlDatasource } from "./index.ts";

afterEach(() => Datasource.reset());

describe("mysql", () => {
  it("creates", () => {
    Datasource.use(MysqlDatasource.factory);
    expect(Datasource.create("mysql").provider).toBe("mysql");
  });

  it("creates a named connection that throws on open", async () => {
    const ds = new MysqlDatasource();
    const connection = await ds.createConnection();
    expect(connection).toBeInstanceOf(MysqlConnection);
    await expect(connection.open()).rejects.toThrow(ProviderNotImplementedError);
    ds.createCommand();
    ds.createCommand(connection);
    ds.createRepository();
    ds.createRepository(connection, { table: "t" });
  });
});
