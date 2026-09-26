import { afterEach, describe, expect, it } from "vitest";
import { Datasource, DatasourceFactory, ProviderNotImplementedError, StubCommand, untyped } from "@datasources/core";
import { DynamoDbConnection } from "./connection.ts";
import { DynamoDbProvider } from "./DynamoDbProvider.ts";

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
    expect(command).toBeInstanceOf(StubCommand);
    expect(command.connection).toBe(connection);
    await expect(command.execute("")).rejects.toThrow(ProviderNotImplementedError);
    await expect(command.executeReader("")).rejects.toThrow(ProviderNotImplementedError);
    const implicit = ds.createCommand();
    expect(implicit).toBeInstanceOf(StubCommand);
    await expect(implicit.execute("")).rejects.toThrow(ProviderNotImplementedError);
    await expect(implicit.executeReader("")).rejects.toThrow(ProviderNotImplementedError);
    ds.createRepository(untyped());
    ds.createRepository(untyped(), connection);
    ds.createReadonlyRepository(untyped());
    ds.createReadonlyRepository(untyped(), connection);
    expect(new DynamoDbProvider().transaction(connection).isolation).toBe("serializable");
  });
});
