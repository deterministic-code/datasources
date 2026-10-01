import { afterEach, describe, expect, it } from "vitest";
import { Datasource, DatasourceFactory, ProviderNotImplementedError, StubCommand, untyped } from "@datasources/core";
import { DynamoDbConnection } from "./connection.ts";
import { DynamoDbDriver } from "./DynamoDbDriver.ts";

afterEach(() => DatasourceFactory.reset());

describe("dynamodb", () => {
  it("creates", () => {
    DatasourceFactory.register("dynamodb", new DynamoDbDriver());
    expect(DynamoDbConnection.from()).toBeInstanceOf(DynamoDbConnection);
    expect(Datasource.create({ PROVIDER: "dynamodb" }).provider).toBe("dynamodb");
  });

  it("creates a named connection that throws on open", async () => {
    const ds = new Datasource("dynamodb", new DynamoDbDriver());
    const connection = await ds.createConnection();
    expect(connection).toBeInstanceOf(DynamoDbConnection);
    await expect(connection.open()).rejects.toThrow(ProviderNotImplementedError);
    const command = await ds.createCommand(connection);
    expect(command).toBeInstanceOf(StubCommand);
    expect(command.connection).toBe(connection);
    await expect(command.execute("")).rejects.toThrow(ProviderNotImplementedError);
    await expect(command.executeReader("")).rejects.toThrow(ProviderNotImplementedError);
    await expect(ds.createCommand()).rejects.toThrow(ProviderNotImplementedError);
    ds.createRepository(untyped());
    ds.createRepository(untyped(), connection);
    ds.createReadonlyRepository(untyped());
    ds.createReadonlyRepository(untyped(), connection);
    expect(new DynamoDbDriver().transaction(connection).isolation).toBe("readCommitted");
  });
});
