import { describe, expect, it } from "vitest";
import { Datasource, ProviderNotImplementedError } from "@datasources/core";
import OracleDatasource, { OracleDatasource as Named } from "./index.ts";

describe("oracle stub", () => {
  it("registers and throws on IO", async () => {
    expect(Named.provider).toBe("oracle");
    expect(OracleDatasource.provider).toBe("oracle");
    const ds = Datasource.create("oracle");
    await expect(ds.createConnection().open()).rejects.toThrow(
      ProviderNotImplementedError,
    );
  });
});
