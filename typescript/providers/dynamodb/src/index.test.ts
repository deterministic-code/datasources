import { describe, expect, it } from "vitest";
import { Datasource, ProviderNotImplementedError } from "@datasources/core";
import DynamoDbDatasource, { DynamoDbDatasource as Named } from "./index.ts";

describe("dynamodb stub", () => {
  it("registers and throws on IO", async () => {
    expect(Named.provider).toBe("dynamodb");
    expect(DynamoDbDatasource.provider).toBe("dynamodb");
    const ds = Datasource.create("dynamodb");
    await expect(ds.createConnection().open()).rejects.toThrow(
      ProviderNotImplementedError,
    );
  });
});
