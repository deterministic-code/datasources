import { afterEach, describe, expect, it } from "vitest";
import { Datasource, ProviderNotImplementedError } from "@datasources/core";
import { TextCommand, TextConnection, TextDatasource } from "./index.ts";

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
    const command = ds.createCommand(connection);
    expect(command).toBeInstanceOf(TextCommand);
    expect(command.connection).toBe(connection);
    await expect(command.execute()).rejects.toThrow(ProviderNotImplementedError);
    await expect(command.executeReader()).rejects.toThrow(ProviderNotImplementedError);
    const implicit = ds.createCommand();
    expect(implicit).toBeInstanceOf(TextCommand);
    await expect(implicit.execute()).rejects.toThrow(ProviderNotImplementedError);
    await expect(implicit.executeReader()).rejects.toThrow(ProviderNotImplementedError);
    ds.createRepository();
    ds.createRepository(connection);
  });
});
