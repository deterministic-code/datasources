import { afterEach, describe, expect, it } from "vitest";
import { Datasource, DatasourceFactory, ProviderNotImplementedError } from "@datasources/core";
import { XmlCommand, XmlConnection } from "./index.ts";

afterEach(() => DatasourceFactory.reset());

describe("xml", () => {
  it("creates", () => {
    DatasourceFactory.register("xml", XmlConnection.from, XmlConnection.repository);
    expect(XmlConnection.from()).toBeInstanceOf(XmlConnection);
    expect(Datasource.create({ provider: "xml" }).provider).toBe("xml");
  });

  it("creates a named connection that throws on open", async () => {
    const ds = new Datasource("xml", () => new XmlConnection(), XmlConnection.repository);
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
