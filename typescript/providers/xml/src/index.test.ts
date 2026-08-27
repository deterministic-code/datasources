import { afterEach, describe, expect, it } from "vitest";
import { Datasource, ProviderNotImplementedError } from "@datasources/core";
import { XmlConnection, XmlDatasource } from "./index.ts";

afterEach(() => Datasource.reset());

describe("xml", () => {
  it("creates", () => {
    Datasource.use(XmlDatasource.factory);
    expect(Datasource.create("xml").provider).toBe("xml");
  });

  it("creates a named connection that throws on open", async () => {
    const ds = new XmlDatasource();
    const connection = await ds.createConnection();
    expect(connection).toBeInstanceOf(XmlConnection);
    await expect(connection.open()).rejects.toThrow(ProviderNotImplementedError);
    ds.createCommand();
    ds.createCommand(connection);
    ds.createRepository();
    ds.createRepository(connection, { table: "t" });
  });
});
