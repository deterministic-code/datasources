import { afterEach, describe, expect, it } from "vitest";
import { Datasource, ProviderNotImplementedError } from "@datasources/core";
import { MongoDbCommand, MongoDbConnection, MongoDbDatasource } from "./index.ts";

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
    const command = ds.createCommand(connection);
    expect(command).toBeInstanceOf(MongoDbCommand);
    expect(command.connection).toBe(connection);
    await expect(command.execute()).rejects.toThrow(ProviderNotImplementedError);
    await expect(command.executeReader()).rejects.toThrow(ProviderNotImplementedError);
    const implicit = ds.createCommand();
    expect(implicit).toBeInstanceOf(MongoDbCommand);
    await expect(implicit.execute()).rejects.toThrow(ProviderNotImplementedError);
    await expect(implicit.executeReader()).rejects.toThrow(ProviderNotImplementedError);
    ds.createRepository();
    ds.createRepository(connection);
  });
});
