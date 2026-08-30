import { afterEach, describe, expect, it } from "vitest";
import { Datasource, ProviderNotImplementedError } from "@datasources/core";
import { XmlCommand, XmlConnection, XmlDatasource } from "./index.ts";

afterEach(() => Datasource.reset());

describe("xml", () => {
  it("creates", () => {
    Datasource.use(XmlDatasource.instance);
    expect(Datasource.create("xml").provider).toBe("xml");
  });

  it("creates a named connection that throws on open", async () => {
    const ds = new XmlDatasource();
    const connection = await ds.createConnection();
    expect(connection).toBeInstanceOf(XmlConnection);
    await expect(connection.open()).rejects.toThrow(ProviderNotImplementedError);
    const command = ds.createCommand(connection);
    expect(command).toBeInstanceOf(XmlCommand);
    expect(command.connection).toBe(connection);
    await expect(command.execute("")).rejects.toThrow(ProviderNotImplementedError);
    await expect(command.executeReader("")).rejects.toThrow(ProviderNotImplementedError);
    const implicit = ds.createCommand();
    expect(implicit).toBeInstanceOf(XmlCommand);
    await expect(implicit.execute("")).rejects.toThrow(ProviderNotImplementedError);
    await expect(implicit.executeReader("")).rejects.toThrow(ProviderNotImplementedError);
    ds.createRepository();
    ds.createRepository(connection);
    ds.createReadonlyRepository();
    ds.createReadonlyRepository(connection);
  });
});
