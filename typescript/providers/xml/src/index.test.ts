import { afterEach, describe, expect, it } from "vitest";
import { Datasource, DatasourceFactory, Native, ProviderNotImplementedError, StubCommand, untyped } from "@datasources/core";
import { XmlConnection } from "./connection.ts";
import { XmlDriver } from "./XmlDriver.ts";

afterEach(() => DatasourceFactory.reset());

describe("xml", () => {
  it("creates", () => {
    DatasourceFactory.register("xml", new XmlDriver());
    expect(XmlConnection.from()).toBeInstanceOf(XmlConnection);
    expect(Datasource.create({ PROVIDER: "xml" }).provider).toBe("xml");
  });

  it("creates a named connection that throws on open", async () => {
    const ds = new Datasource("xml", new XmlDriver());
    const connection = await ds.createConnection();
    expect(connection).toBeInstanceOf(XmlConnection);
    await expect(connection.open()).rejects.toThrow(ProviderNotImplementedError);
    const command = await ds.createCommand(connection);
    expect(command).toBeInstanceOf(StubCommand);
    expect(command.connection).toBe(connection);
    await expect(command.executeNative(new Native("x"))).rejects.toThrow(ProviderNotImplementedError);
    await expect(command.executeReaderNative(new Native("x"))).rejects.toThrow(ProviderNotImplementedError);
    await expect(ds.createCommand()).rejects.toThrow(ProviderNotImplementedError);
    ds.createRepository(untyped());
    ds.createRepository(untyped(), connection);
    ds.createReadonlyRepository(untyped());
    ds.createReadonlyRepository(untyped(), connection);
    expect(new XmlDriver().transaction(connection).isolation).toBe("readCommitted");
  });
});
