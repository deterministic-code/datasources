import { describe, expect, it } from "vitest";
import { Datasource, ProviderNotImplementedError } from "@datasources/core";
import TextDatasource, { TextDatasource as Named } from "./index.ts";

describe("text stub", () => {
  it("registers and throws on IO", async () => {
    expect(Named.provider).toBe("text");
    expect(TextDatasource.provider).toBe("text");
    const ds = Datasource.create("text");
    await expect(ds.createConnection().open()).rejects.toThrow(
      ProviderNotImplementedError,
    );
  });
});
