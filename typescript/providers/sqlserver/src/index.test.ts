import { describe, expect, it } from "vitest";
import { Datasource, ProviderNotImplementedError } from "@datasources/core";
import SqlServerDatasource, { SqlServerDatasource as Named } from "./index.ts";

describe("sqlserver stub", () => {
  it("registers and throws on IO", async () => {
    expect(Named.provider).toBe("sqlserver");
    expect(SqlServerDatasource.provider).toBe("sqlserver");
    const ds = Datasource.create("sqlserver");
    await expect(ds.createConnection().open()).rejects.toThrow(
      ProviderNotImplementedError,
    );
  });
});
