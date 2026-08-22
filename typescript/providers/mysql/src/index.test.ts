import { describe, expect, it } from "vitest";
import { Datasource, ProviderNotImplementedError } from "@datasources/core";
import MysqlDatasource, { MysqlDatasource as Named } from "./index.ts";

describe("mysql stub", () => {
  it("registers and throws on IO", async () => {
    expect(Named.provider).toBe("mysql");
    expect(MysqlDatasource.provider).toBe("mysql");
    const ds = Datasource.create("mysql");
    await expect(ds.createConnection().open()).rejects.toThrow(
      ProviderNotImplementedError,
    );
  });
});
