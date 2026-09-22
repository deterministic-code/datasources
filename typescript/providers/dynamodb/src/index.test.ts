import { afterEach, describe, expect, it } from "vitest";
import { Datasource, DatasourceFactory, ProviderNotImplementedError } from "@datasources/core";
import { DynamoDbCommand, DynamoDbConnection, DynamoDbProvider } from "./index.ts";

afterEach(() => DatasourceFactory.reset());

describe("dynamodb", () => {
  it("creates", () => {
    DatasourceFactory.register("dynamodb", new DynamoDbProvider());
    expect(DynamoDbConnection.from()).toBeInstanceOf(DynamoDbConnection);
    expect(Datasource.create({ provider: "dynamodb" }).provider).toBe("dynamodb");
  });

  it("creates a named connection that throws on open", async () => {
    const ds = new Datasource("dynamodb", new DynamoDbProvider());
    const connection = await ds.createConnection();
    expect(connection).toBeInstanceOf(DynamoDbConnection);
    await expect(connection.open()).rejects.toThrow(ProviderNotImplementedError);
    const command = ds.createCommand(connection);
    expect(command).toBeInstanceOf(DynamoDbCommand);
    expect(command.connection).toBe(connection);
    await expect(command.execute("")).rejects.toThrow(ProviderNotImplementedError);
    await expect(command.executeReader("")).rejects.toThrow(ProviderNotImplementedError);
    const implicit = ds.createCommand();
    expect(implicit).toBeInstanceOf(DynamoDbCommand);
    await expect(implicit.execute("")).rejects.toThrow(ProviderNotImplementedError);
    await expect(implicit.executeReader("")).rejects.toThrow(ProviderNotImplementedError);
    ds.createRepository();
    ds.createRepository(connection);
    ds.createReadonlyRepository();
    ds.createReadonlyRepository(connection);
    expect(new DynamoDbProvider().transaction(connection).isolation).toBe("serializable");
  });
});
