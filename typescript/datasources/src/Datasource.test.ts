import { afterAll, afterEach, beforeEach, describe, expect, it } from "vitest";
import { Datasource, DatasourceFactory, MockConnection, MockDriver, MockRepository, RepositoryNotRegisteredError } from "./index.ts";

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
  it("creates from plural name", () => {
    const ds = new Datasource("mock", new MockDriver());
    const connection = new MockConnection();
    class Item {}
    const crud = new MockRepository<Item>(connection, Item);
    const read = new MockRepository<Item>(connection, Item);
    ds.repositories.register("Contact", "contacts", () => crud, () => read);
    const dsDynamic = ds as Datasource & { contacts: { create: () => unknown; createReadonly: () => unknown } };

    expect(dsDynamic.contacts.create()).toBe(crud);
    expect(dsDynamic.contacts.createReadonly()).toBe(read);
    expect(ds.createRepository("Contact")).toBe(crud);
    expect(ds.createReadonlyRepository("contacts")).toBe(read);
    expect(() => (ds as Datasource & { missing: unknown }).missing).toThrow(RepositoryNotRegisteredError);
    expect((ds as unknown as Record<symbol, unknown>)[Symbol.toStringTag]).toBeUndefined();
  });

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
    expect(ds.createRepository(Item, undefined, { entity: "items", ids: ["id"] })).toBeTruthy();
    expect(ds.createReadonlyRepository(Item, undefined, { entity: "items" })).toBeTruthy();
    expect(ds.createReadonlyRepository(Item)).toBeTruthy();
  });
});
