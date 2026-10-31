import { afterAll, afterEach, beforeEach, describe, expect, it } from "vitest";
import { Datasource, DatasourceFactory, MockConnection, MockDriver, RepositoryNotRegisteredError, RowTypeConverter, Schema } from "./index.ts";

const envKeys = [
  "INSTALLED_PROVIDERS",
  "DATASOURCE_URL",
  "PROVIDER",
] as const;
const savedEnv = Object.fromEntries(envKeys.map((key) => [key, process.env[key]]));

beforeEach(() => {
  for (const key of envKeys) delete process.env[key];
  DatasourceFactory.register("sqlite", new MockDriver());
  process.env.PROVIDER = "sqlite";
});

afterEach(() => DatasourceFactory.reset());

afterAll(() => {
  for (const key of envKeys) {
    if (savedEnv[key] === undefined) delete process.env[key];
    else process.env[key] = savedEnv[key];
  }
});

describe("Datasource", () => {
  it("create default from env settings", async () => {
    const ds = Datasource.default;
    const command = await ds.createCommand();
    expect(ds.provider).toBe("sqlite");
    expect(command.connection).toBe(await ds.createConnection());
    expect(command.connection.isOpen).toBe(true);
    class Wide {
      id = 0;
      on = false;
      when = new Date(0);
      blob = new Uint8Array();
      tags: string[] = [];
      name = "";
    }
    expect(ds.createRepository(Wide)).toBeTruthy();
  });

  it("leaves a passed connection closed", async () => {
    const ds = Datasource.default;
    const given = new MockConnection();
    expect((await ds.createCommand(given)).connection).toBe(given);
    expect(given.isOpen).toBe(false);
    await ds.createCommand();
    await expect(ds.createCommand()).rejects.toThrow("Connection is already open");
  });

  it("applies repository options for crud and readonly", () => {
    const ds = Datasource.default;
    class Item {
      id = 0;
      name = "";
    }
    const schema = new Schema({ id: { type: "integer", nullable: false, hasDefault: false } });
    const rowConverter = RowTypeConverter.from({}, schema.types());
    expect(ds.createRepository(Item, undefined, { entity: "items", ids: ["id"], schema, rowConverter })).toBeTruthy();
    expect(ds.createReadonlyRepository(Item, undefined, { entity: "items", schema, rowConverter })).toBeTruthy();
    expect(ds.createReadonlyRepository(Item)).toBeTruthy();
    ds.repositories.register(
      "Item",
      "items",
      (connection) => ds.createRepository(Item, connection),
      (connection) => ds.createReadonlyRepository(Item, connection),
    );
    expect((ds as Datasource & { items: object }).items).toBeTruthy();
    expect(ds.createRepository("Item")).toBeTruthy();
    expect(ds.createReadonlyRepository("items")).toBeTruthy();
    expect(() => (ds as Datasource & { missing: object }).missing).toThrow(RepositoryNotRegisteredError);
  });

  it("closes the default connection", async () => {
    await using ds = Datasource.default;
    await ds.createCommand();
    expect((await ds.createConnection()).isOpen).toBe(true);
    await ds.close();
    expect((await ds.createConnection()).isOpen).toBe(false);
  });
});
