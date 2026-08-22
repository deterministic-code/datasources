import { describe, expect, it } from "vitest";
import { Datasource, ProviderNotImplementedError } from "@datasources/core";
import MongoDbDatasource, { MongoDbDatasource as Named } from "./index.ts";

describe("mongodb stub", () => {
  it("registers and throws on IO", async () => {
    expect(Named.provider).toBe("mongodb");
    expect(MongoDbDatasource.provider).toBe("mongodb");
    const ds = Datasource.create("mongodb");
    await expect(ds.createConnection().open()).rejects.toThrow(
      ProviderNotImplementedError,
    );
  });
});
