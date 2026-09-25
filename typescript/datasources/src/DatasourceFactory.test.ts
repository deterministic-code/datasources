import { afterAll, afterEach, beforeEach, describe, expect, it } from "vitest";
import {
  bindInstall,
  CRUDRepository,
  Datasource,
  DatasourceError,
  DatasourceFactory,
  MockCommand,
  MockDriver,
  ProviderNotInstalledError,
  ReadonlyRepository,
  untyped,
} from "./index.ts";

class SelectText<T> extends ReadonlyRepository<T> {
  static of(repo: object): string | undefined {
    return (repo as SelectText<T>).statements?.query().getText();
  }
}

const envKeys = [
  "INSTALLED_PROVIDERS",
  "DATASOURCE_URL",
  "DATABASE_SCHEMA",
  "DEFAULT_PROVIDER",
  "DEFAULT_DATASOURCE_URL",
  "PROVIDER",
] as const;
const savedEnv = Object.fromEntries(envKeys.map((key) => [key, process.env[key]]));

beforeEach(() => {
  for (const key of envKeys) delete process.env[key];
});

afterEach(() => {
  DatasourceFactory.reset();
  bindInstall();
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
    process.env.DATASOURCE_URL = "sqlite:///:memory:";
    const ds = DatasourceFactory.default;
    expect(ds.provider).toBe("sqlite");
  });

  it("missing provider throws", () => {
    expect(() => DatasourceFactory.create({ provider: "missing" })).toThrow(
      ProviderNotInstalledError,
    );
  });

  it("mock driver builds command and transaction", () => {
    const driver = new MockDriver();
    expect(driver.rowTypeConverters).toEqual({});
    const connection = driver.connection();
    expect(driver.command(connection)).toBeInstanceOf(MockCommand);
    expect(driver.transaction(connection).isolation).toBe("serializable");
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
    const created = Datasource.create({ provider: "json", path: "x" });
    expect(created.provider).toBe("json");
    DatasourceFactory.reset();
    expect(DatasourceFactory.registered()).toEqual([]);
    expect(() => Datasource.create({ provider: "json" })).toThrow(ProviderNotInstalledError);
  });

  it("requires consumer defaults when provider is omitted", async () => {
    expect(() => Datasource.create()).toThrow(DatasourceError);
    DatasourceFactory.register("sqlite", new MockDriver());
    const ds = Datasource.create({
      provider: "sqlite",
      database_url: "sqlite:///:memory:",
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
    expect(typed instanceof CRUDRepository && SelectText.of(typed)).toBe("SELECT id, name FROM Item");
    expect(typedRead.constructor).toBe(ReadonlyRepository);
    expect(typedRead.type).toBe(Item);
    expect(typedRead instanceof ReadonlyRepository && SelectText.of(typedRead)).toBe(
      "SELECT id, name FROM Item",
    );
    const enriched = ds.createReadonlyRepository(Item) as ReadonlyRepository<Item>;
    expect(enriched).toBeInstanceOf(ReadonlyRepository);
    enriched.addLinked({ into: "name", from: "id", by: "id", association: { parent: "id", related: "name" } }, typedRead, typed);
    const withWrite = ds.createRepository(Item, connection, {
      write: { persist: async () => {} },
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
      provider: "sqlite",
      database_schema: "main",
    }).provider).toBe("sqlite");
    process.env.DATASOURCE_URL = "  ";
    process.env.DATABASE_SCHEMA = "  ";
    expect(() => Datasource.create()).toThrow(DatasourceError);
    process.env.DATASOURCE_URL = "sqlite:///:memory:";
    process.env.DATABASE_SCHEMA = "main";
    expect(Datasource.create().provider).toBe("sqlite");
    bindInstall((options) => {
      if (options.INSTALLED_PROVIDERS?.includes("json"))
        DatasourceFactory.register("json", new MockDriver());
    });
    DatasourceFactory.defaults({ INSTALLED_PROVIDERS: "json" });
    expect(DatasourceFactory.registered()).toEqual(["json", "sqlite"]);
    DatasourceFactory.reset();
    for (const key of envKeys) delete process.env[key];
    expect(() => Datasource.create()).toThrow("Datasource defaults are not specified");
    expect(() => DatasourceFactory.default).toThrow("Datasource defaults are not specified");
    DatasourceFactory.register("sqlite", new MockDriver());
    DatasourceFactory.register("memory", new MockDriver());
    process.env.DATASOURCE_URL = "sqlite:///:memory:";
    expect(DatasourceFactory.default.provider).toBe("sqlite");
    expect(Datasource.default.provider).toBe("sqlite");
    delete process.env.DATASOURCE_URL;
    process.env.PROVIDER = "sqlite";
    expect(DatasourceFactory.default.provider).toBe("sqlite");
    process.env.DEFAULT_PROVIDER = "memory";
    process.env.DEFAULT_DATASOURCE_URL = "memory://";
    expect(DatasourceFactory.default.provider).toBe("memory");
  });
});
