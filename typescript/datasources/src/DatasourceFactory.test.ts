import { afterAll, afterEach, beforeEach, describe, expect, it } from "vitest";
import {
  CRUDRepository,
  Datasource,
  DatasourceError,
  DatasourceFactory,
  MockDriver,
  ProviderNotInstalledError,
  ReadonlyRepository,
  untyped,
} from "./index.ts";
import type { IConnection } from "./IConnection.ts";
import { MockCommand } from "./mocks/mock-command.ts";
import { MockConnection } from "./mocks/mock-connection.ts";

const selectText = async (
  repo: { read(): AsyncIterable<unknown> },
  connection: IConnection,
): Promise<string | undefined> => {
  if (!(connection instanceof MockConnection)) throw new Error("expected mock connection");
  const command = new MockCommand(connection);
  connection.command = command;
  for await (const item of repo.read()) void item;
  return command.executeReaderCalls[0]?.text;
};

const envKeys = [
  "INSTALLED_PROVIDERS",
  "DATASOURCE_URL",
  "PROVIDER",
] as const;
const savedEnv = Object.fromEntries(envKeys.map((key) => [key, process.env[key]]));

beforeEach(() => {
  for (const key of envKeys) delete process.env[key];
});

afterEach(() => {
  DatasourceFactory.reset();
});

afterAll(() => {
  for (const key of envKeys) {
    if (savedEnv[key] === undefined) delete process.env[key];
    else process.env[key] = savedEnv[key];
  }
});

describe("DatasourceFactory", () => {
  it("default from environment", () => {
    DatasourceFactory.register("sqlite", new MockDriver());
    process.env.PROVIDER = "sqlite";
    const ds = Datasource.default;
    expect(ds.provider).toBe("sqlite");
  });

  it("skips a repeated installed provider", () => {
    DatasourceFactory.defaults({ INSTALLED_PROVIDERS: "memory,memory" });
    expect(DatasourceFactory.registered()).toEqual(["memory"]);
  });

  it("missing provider throws", () => {
    expect(() => DatasourceFactory.create({ PROVIDER: "missing" })).toThrow(
      ProviderNotInstalledError,
    );
  });

  it("mock driver builds command and transaction", () => {
    const driver = new MockDriver();
    expect(driver.rowTypeConverters).toEqual({});
    const connection = driver.connection();
    expect(driver.command(connection)).toBeInstanceOf(MockCommand);
    expect(driver.transaction(connection).isolation).toBe("readCommitted");
    expect(driver.transaction(connection, "readCommitted").isolation).toBe(
      "readCommitted",
    );
  });
});

describe("Datasource registry", () => {
  it("registers, lists, creates, and resets", () => {
    expect(DatasourceFactory.registered()).toEqual([]);
    DatasourceFactory.register("json", new MockDriver());
    DatasourceFactory.register("memory", new MockDriver());
    expect(DatasourceFactory.registered()).toEqual(["json", "memory"]);
    const created = Datasource.create({ PROVIDER: "json", DATASOURCE_URL: "json://x" });
    expect(created.provider).toBe("json");
    DatasourceFactory.reset();
    expect(DatasourceFactory.registered()).toEqual([]);
    expect(() => Datasource.create({ PROVIDER: "json" })).toThrow(ProviderNotInstalledError);
  });

  it("requires consumer defaults when provider is omitted", async () => {
    expect(() => Datasource.create()).toThrow(DatasourceError);
    DatasourceFactory.register("sqlite", new MockDriver());
    const ds = Datasource.create({
      PROVIDER: "sqlite",
      DATASOURCE_URL: "sqlite:///:memory:",
    });
    expect(ds.provider).toBe("sqlite");
    const connection = await ds.createConnection();
    expect(connection.isOpen).toBe(false);
    expect(ds.createCommand().connection).toBe(connection);
    expect(ds.createRepository(untyped())).toBeInstanceOf(CRUDRepository);
    expect(ds.createReadonlyRepository(untyped()).constructor).toBe(ReadonlyRepository);
    class Item {
      id = 0;
      name = "";
    }
    const typed = ds.createRepository(Item, connection);
    const typedRead = ds.createReadonlyRepository(Item);
    expect(typed).toBeInstanceOf(CRUDRepository);
    expect(typed.type).toBe(Item);
    expect(await selectText(typed, connection)).toBe("SELECT id, name FROM Item");
    expect(typedRead.constructor).toBe(ReadonlyRepository);
    expect(typedRead.type).toBe(Item);
    expect(await selectText(typedRead, connection)).toBe("SELECT id, name FROM Item");
    const enriched = ds.createReadonlyRepository(Item) as ReadonlyRepository<Item>;
    expect(enriched).toBeInstanceOf(ReadonlyRepository);
    typed.addLinked({ destinationProperty: "name", sourceProperty: "id", relatedKey: "id", association: { parent: "id", related: "name" } }, typedRead, typed);
    const withWrite = ds.createRepository(Item, connection, {
      write: { useTransaction: false, persist: async () => {} },
      ids: ["id"],
    });
    expect(withWrite).toBeInstanceOf(CRUDRepository);
    const withCrudIds = ds.createRepository(Item, connection, {
      ids: undefined as never,
      entity: "Item",
    });
    expect(withCrudIds).toBeInstanceOf(CRUDRepository);
    const withReadOpts = ds.createRepository(Item, connection, {
      entity: "items",
    });
    expect(withReadOpts).toBeInstanceOf(CRUDRepository);
    expect(new DatasourceFactory().create({
      PROVIDER: "sqlite",
    }).provider).toBe("sqlite");
    process.env.DATASOURCE_URL = "  ";
    expect(() => Datasource.create()).toThrow(DatasourceError);
    process.env.DATASOURCE_URL = "sqlite:///:memory:";
    expect(() => Datasource.create()).toThrow(ProviderNotInstalledError);
    DatasourceFactory.defaults({ INSTALLED_PROVIDERS: "json" });
    expect(DatasourceFactory.registered()).toEqual(["json", "sqlite"]);
    DatasourceFactory.reset();
    for (const key of envKeys) delete process.env[key];
    expect(() => Datasource.create()).toThrow("Datasource defaults are not specified");
    expect(() => Datasource.default).toThrow("Datasource defaults are not specified");
    DatasourceFactory.register("sqlite", new MockDriver());
    DatasourceFactory.register("memory", new MockDriver());
    process.env.DATASOURCE_URL = "sqlite:///:memory:";
    expect(() => Datasource.default).toThrow(ProviderNotInstalledError);
    delete process.env.DATASOURCE_URL;
    process.env.PROVIDER = "sqlite";
    expect(Datasource.default.provider).toBe("sqlite");
    process.env.PROVIDER = "memory";
    process.env.DATASOURCE_URL = "memory://";
    expect(Datasource.default.provider).toBe("memory");
  });
});
