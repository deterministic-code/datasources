import { afterEach, describe, expect, it } from "vitest";
import { Datasource, ProviderNotImplementedError } from "@datasources/core";
import { TextConnection, TextDatasource } from "./index.ts";

afterEach(() => Datasource.reset());

describe("text", () => {
  it("creates", () => {
    Datasource.use(TextDatasource.factory);
    expect(Datasource.create("text").provider).toBe("text");
  });

  it("creates a named connection that throws on open", async () => {
    const ds = new TextDatasource();
    const connection = await ds.createConnection();
    expect(connection).toBeInstanceOf(TextConnection);
    await expect(connection.open()).rejects.toThrow(ProviderNotImplementedError);
    ds.createCommand();
    ds.createCommand(connection);
    ds.createRepository();
    ds.createRepository(connection, { table: "t" });
  });
});
