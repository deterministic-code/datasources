import { afterEach, describe, expect, it } from "vitest";
import { Datasource, ProviderNotImplementedError } from "@datasources/core";
import { DynamoDbConnection, DynamoDbDatasource } from "./index.ts";

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
    ds.createCommand();
    ds.createCommand(connection);
    ds.createRepository();
    ds.createRepository(connection, { table: "t" });
  });
});
