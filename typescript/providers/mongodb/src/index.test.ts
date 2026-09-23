import { afterEach, describe, expect, it } from "vitest";
import { Datasource, DatasourceFactory, ProviderNotImplementedError, untyped } from "@datasources/core";
import { MongoDbCommand } from "./command.ts";
import { MongoDbConnection } from "./connection.ts";
import { MongoDbProvider } from "./MongoDbProvider.ts";

afterEach(() => DatasourceFactory.reset());

describe("mongodb", () => {
  it("creates", () => {
    DatasourceFactory.register("mongodb", new MongoDbProvider());
    expect(MongoDbConnection.from()).toBeInstanceOf(MongoDbConnection);
    expect(Datasource.create({ provider: "mongodb" }).provider).toBe("mongodb");
  });

  it("creates a named connection that throws on open", async () => {
    const ds = new Datasource("mongodb", new MongoDbProvider());
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
    ds.createRepository(untyped());
    ds.createRepository(untyped(), connection);
    ds.createReadonlyRepository(untyped());
    ds.createReadonlyRepository(untyped(), connection);
    expect(new MongoDbProvider().transaction(connection).isolation).toBe("serializable");
  });
});
