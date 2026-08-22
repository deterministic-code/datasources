import { describe, expect, it } from "vitest";
import { Datasource, ProviderNotImplementedError } from "@datasources/core";
import PostgresDatasource, { PostgresDatasource as Named } from "./index.ts";

describe("postgres stub", () => {
  it("registers and throws on IO", async () => {
    expect(Named.provider).toBe("postgres");
    expect(PostgresDatasource.provider).toBe("postgres");
    const ds = Datasource.create("postgres");
    await expect(ds.createConnection().open()).rejects.toThrow(
      ProviderNotImplementedError,
    );
  });
});
