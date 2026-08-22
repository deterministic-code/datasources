import { describe, expect, it } from "vitest";
import { Datasource, ProviderNotImplementedError } from "@datasources/core";
import YamlDatasource, { YamlDatasource as Named } from "./index.ts";

describe("yaml stub", () => {
  it("registers and throws on IO", async () => {
    expect(Named.provider).toBe("yaml");
    expect(YamlDatasource.provider).toBe("yaml");
    const ds = Datasource.create("yaml");
    await expect(ds.createConnection().open()).rejects.toThrow(
      ProviderNotImplementedError,
    );
  });
});
