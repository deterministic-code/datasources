import { describe, expect, it } from "vitest";
import { Datasource, ProviderNotImplementedError } from "@datasources/core";
import XmlDatasource, { XmlDatasource as Named } from "./index.ts";

describe("xml stub", () => {
  it("registers and throws on IO", async () => {
    expect(Named.provider).toBe("xml");
    expect(XmlDatasource.provider).toBe("xml");
    const ds = Datasource.create("xml");
    await expect(ds.createConnection().open()).rejects.toThrow(
      ProviderNotImplementedError,
    );
  });
});
