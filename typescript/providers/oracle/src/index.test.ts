import { afterEach, describe, expect, it } from "vitest";
import { Datasource, DatasourceFactory, ProviderNotImplementedError, StubCommand, untyped } from "@datasources/core";
import { OracleConnection } from "./connection.ts";
import { OracleDriver } from "./OracleDriver.ts";

afterEach(() => DatasourceFactory.reset());

describe("oracle", () => {
  it("creates", () => {
    DatasourceFactory.register("oracle", new OracleDriver());
    expect(OracleConnection.from()).toBeInstanceOf(OracleConnection);
    expect(Datasource.create({ PROVIDER: "oracle" }).provider).toBe("oracle");
  });

  it("creates a named connection that throws on open", async () => {
    const ds = new Datasource("oracle", new OracleDriver());
    const connection = await ds.createConnection();
    expect(connection).toBeInstanceOf(OracleConnection);
    await expect(connection.open()).rejects.toThrow(ProviderNotImplementedError);
    const command = await ds.createCommand(connection);
    expect(command).toBeInstanceOf(StubCommand);
    expect(command.connection).toBe(connection);
    await expect(command.execute("")).rejects.toThrow(ProviderNotImplementedError);
    await expect(command.executeReader("")).rejects.toThrow(ProviderNotImplementedError);
    await expect(ds.createCommand()).rejects.toThrow(ProviderNotImplementedError);
    ds.createRepository(untyped());
    ds.createRepository(untyped(), connection);
    ds.createReadonlyRepository(untyped());
    ds.createReadonlyRepository(untyped(), connection);
    expect(new OracleDriver().transaction(connection).isolation).toBe("readCommitted");
  });
});
