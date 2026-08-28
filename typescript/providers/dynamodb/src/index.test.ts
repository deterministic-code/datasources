import { afterEach, describe, expect, it } from "vitest";
import { Datasource, ProviderNotImplementedError } from "@datasources/core";
import { DynamoDbCommand, DynamoDbConnection, DynamoDbDatasource } from "./index.ts";

afterEach(() => Datasource.reset());

describe("dynamodb", () => {
  it("creates", () => {
    Datasource.use(DynamoDbDatasource.factory);
    expect(Datasource.create("dynamodb").provider).toBe("dynamodb");
  });

  it("creates a named connection that throws on open", async () => {
    const ds = new DynamoDbDatasource();
    const connection = await ds.createConnection();
    expect(connection).toBeInstanceOf(DynamoDbConnection);
    await expect(connection.open()).rejects.toThrow(ProviderNotImplementedError);
    const command = ds.createCommand(connection);
    expect(command).toBeInstanceOf(DynamoDbCommand);
    expect(command.connection).toBe(connection);
    await expect(command.execute()).rejects.toThrow(ProviderNotImplementedError);
    await expect(command.executeReader()).rejects.toThrow(ProviderNotImplementedError);
    const implicit = ds.createCommand();
    expect(implicit).toBeInstanceOf(DynamoDbCommand);
    await expect(implicit.execute()).rejects.toThrow(ProviderNotImplementedError);
    await expect(implicit.executeReader()).rejects.toThrow(ProviderNotImplementedError);
    ds.createRepository();
    ds.createRepository(connection);
    ds.createReadonlyRepository();
    ds.createReadonlyRepository(connection);
  });
});
