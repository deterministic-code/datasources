import { afterEach, describe, expect, it } from "vitest";
import { Datasource, DatasourceFactory, ProviderNotImplementedError } from "@datasources/core";
import { MongoDbCommand, MongoDbConnection } from "./index.ts";

afterEach(() => DatasourceFactory.reset());

describe("mongodb", () => {
  it("creates", () => {
    DatasourceFactory.register("mongodb", MongoDbConnection.from, MongoDbConnection.repository);
    expect(MongoDbConnection.from()).toBeInstanceOf(MongoDbConnection);
    expect(Datasource.create({ provider: "mongodb" }).provider).toBe("mongodb");
  });

  it("creates a named connection that throws on open", async () => {
    const ds = new Datasource("mongodb", () => new MongoDbConnection(), MongoDbConnection.repository);
    const connection = await ds.createConnection();
    expect(connection).toBeInstanceOf(MongoDbConnection);
    await expect(connection.open()).rejects.toThrow(ProviderNotImplementedError);
    const command = ds.createCommand(connection);
    expect(command).toBeInstanceOf(MongoDbCommand);
    expect(command.connection).toBe(connection);
    await expect(command.execute("")).rejects.toThrow(ProviderNotImplementedError);
    await expect(command.executeReader("")).rejects.toThrow(ProviderNotImplementedError);
    const implicit = ds.createCommand();
    expect(implicit).toBeInstanceOf(MongoDbCommand);
    await expect(implicit.execute("")).rejects.toThrow(ProviderNotImplementedError);
    await expect(implicit.executeReader("")).rejects.toThrow(ProviderNotImplementedError);
    ds.createRepository();
    ds.createRepository(connection);
    ds.createReadonlyRepository();
    ds.createReadonlyRepository(connection);
  });
});
