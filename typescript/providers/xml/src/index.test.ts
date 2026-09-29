import { afterEach, describe, expect, it } from "vitest";
import { Datasource, DatasourceFactory, ProviderNotImplementedError, StubCommand, untyped } from "@datasources/core";
import { XmlConnection } from "./connection.ts";
import { XmlDriver } from "./XmlDriver.ts";

afterEach(() => DatasourceFactory.reset());

describe("xml", () => {
  it("creates", () => {
    DatasourceFactory.register("xml", new XmlDriver());
    expect(XmlConnection.from()).toBeInstanceOf(XmlConnection);
    expect(Datasource.create({ provider: "xml" }).provider).toBe("xml");
  });

  it("creates a named connection that throws on open", async () => {
    const ds = new Datasource("xml", new XmlDriver());
    const connection = await ds.createConnection();
    expect(connection).toBeInstanceOf(XmlConnection);
    await expect(connection.open()).rejects.toThrow(ProviderNotImplementedError);
    const command = ds.createCommand(connection);
    expect(command).toBeInstanceOf(StubCommand);
    expect(command.connection).toBe(connection);
    await expect(command.execute("")).rejects.toThrow(ProviderNotImplementedError);
    await expect(command.executeReader("")).rejects.toThrow(ProviderNotImplementedError);
    const implicit = ds.createCommand();
    expect(implicit).toBeInstanceOf(StubCommand);
    await expect(implicit.execute("")).rejects.toThrow(ProviderNotImplementedError);
    await expect(implicit.executeReader("")).rejects.toThrow(ProviderNotImplementedError);
    ds.createRepository(untyped());
    ds.createRepository(untyped(), connection);
    ds.createReadonlyRepository(untyped());
    ds.createReadonlyRepository(untyped(), connection);
    expect(new XmlDriver().transaction(connection).isolation).toBe("serializable");
  });
});
