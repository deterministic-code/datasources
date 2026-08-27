import { afterEach, describe, expect, it } from "vitest";
import { Datasource, ProviderNotImplementedError } from "@datasources/core";
import { MongoDbConnection, MongoDbDatasource } from "./index.ts";

afterEach(() => Datasource.reset());

describe("mongodb", () => {
  it("creates", () => {
    Datasource.use(MongoDbDatasource.factory);
    expect(Datasource.create("mongodb").provider).toBe("mongodb");
  });

  it("creates a named connection that throws on open", async () => {
    const ds = new MongoDbDatasource();
    const connection = await ds.createConnection();
    expect(connection).toBeInstanceOf(MongoDbConnection);
    await expect(connection.open()).rejects.toThrow(ProviderNotImplementedError);
    ds.createCommand();
    ds.createCommand(connection);
    ds.createRepository();
    ds.createRepository(connection, { table: "t" });
  });
});
