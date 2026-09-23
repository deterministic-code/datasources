import { afterEach, describe, expect, it } from "vitest";
import { Datasource, DatasourceFactory, ProviderNotImplementedError, untyped } from "@datasources/core";
import { XmlCommand } from "./command.ts";
import { XmlConnection } from "./connection.ts";
import { XmlProvider } from "./XmlProvider.ts";

afterEach(() => DatasourceFactory.reset());

describe("xml", () => {
  it("creates", () => {
    DatasourceFactory.register("xml", new XmlProvider());
    expect(XmlConnection.from()).toBeInstanceOf(XmlConnection);
    expect(Datasource.create({ provider: "xml" }).provider).toBe("xml");
  });

  it("creates a named connection that throws on open", async () => {
    const ds = new Datasource("xml", new XmlProvider());
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
    ds.createRepository(untyped());
    ds.createRepository(untyped(), connection);
    ds.createReadonlyRepository(untyped());
    ds.createReadonlyRepository(untyped(), connection);
    expect(new XmlProvider().transaction(connection).isolation).toBe("serializable");
  });
});
