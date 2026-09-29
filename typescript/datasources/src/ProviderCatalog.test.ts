import { afterAll, afterEach, beforeEach, describe, expect, it } from "vitest";
import { Datasource, DatasourceError, DatasourceFactory, DriverLoader, ProviderCatalog, ProviderNotInstalledError } from "./index.ts";

const envKeys = ["INSTALLED_PROVIDERS", "DATASOURCE_URL", "PROVIDER"] as const;
const savedEnv = Object.fromEntries(envKeys.map((key) => [key, process.env[key]]));

beforeEach(() => {
  for (const key of envKeys) delete process.env[key];
  DatasourceFactory.reset();
});

afterEach(() => DatasourceFactory.reset());

afterAll(() => {
  for (const key of envKeys) {
    if (savedEnv[key] === undefined) delete process.env[key];
    else process.env[key] = savedEnv[key];
  }
});

describe("ProviderCatalog", () => {
  it("default stores location and module", () => {
    const sqlite = ProviderCatalog.default.sqlite;
    expect(sqlite?.location).toBe("@datasources/sqlite");
    expect(sqlite?.module).toBe("SqliteDriver");
    expect(ProviderCatalog.default.sqlserver?.module).toBe("SqlServerDriver");
    expect(ProviderCatalog.default.mongodb?.module).toBe("MongoDbDriver");
    expect(ProviderCatalog.default.dynamodb?.module).toBe("DynamoDbDriver");
  });

  it("full constructor stores location and module", () => {
    const catalog = new ProviderCatalog("acme", "@acme/data", "AcmeDriver");
    expect(catalog.name).toBe("acme");
    expect(catalog.location).toBe("@acme/data");
    expect(catalog.module).toBe("AcmeDriver");
  });

  it("default lists monorepo providers", () => {
    expect(Object.keys(ProviderCatalog.default)).toEqual([
      "json",
      "memory",
      "sqlite",
      "xml",
      "yaml",
      "mysql",
      "sqlserver",
      "postgres",
      "oracle",
      "mongodb",
      "dynamodb",
    ]);
  });

  it("defaults registers installed providers", () => {
    DatasourceFactory.defaults({
      INSTALLED_PROVIDERS: "json,memory",
      DATASOURCE_URL: "memory://",
    });
    expect(DatasourceFactory.registered()).toEqual(["json", "memory"]);
    expect(
      Datasource.create({ PROVIDER: "memory", DATASOURCE_URL: "memory://" }).provider,
    ).toBe("memory");
  });

  it("create does not take installed providers", () => {
    DatasourceFactory.defaults({ INSTALLED_PROVIDERS: "json,memory" });
    expect(Datasource.create({ PROVIDER: "json", DATASOURCE_URL: "json://" }).provider).toBe(
      "json",
    );
    expect(DatasourceFactory.registered()).toEqual(["json", "memory"]);
  });

  it("create reads connection from the environment", () => {
    process.env.INSTALLED_PROVIDERS = "json,memory";
    process.env.DATASOURCE_URL = "memory://";
    process.env.PROVIDER = "memory";
    expect(Datasource.create().provider).toBe("memory");
    expect(Datasource.default.provider).toBe("memory");
    expect(DatasourceFactory.registered()).toEqual(["json", "memory"]);
  });

  it("missing export throws", () => {
    expect(() =>
      DriverLoader.load(new ProviderCatalog("json", "@datasources/json", "MissingDriver")),
    ).toThrow(ProviderNotInstalledError);
  });

  it("missing package or unknown name", () => {
    expect(() => DatasourceFactory.defaults({ INSTALLED_PROVIDERS: "sqlite" })).toThrow(
      ProviderNotInstalledError,
    );
    expect(() => DatasourceFactory.defaults({ INSTALLED_PROVIDERS: "nope" })).toThrow(
      ProviderNotInstalledError,
    );
  });

  it("default overrides env", () => {
    process.env.INSTALLED_PROVIDERS = "json,memory";
    process.env.PROVIDER = "json";
    process.env.DATASOURCE_URL = "json://";
    expect(Datasource.default.provider).toBe("json");
  });

  it("url without provider throws", () => {
    expect(() => DatasourceFactory.defaults({ DATASOURCE_URL: "not-a-url" })).toThrow(
      new DatasourceError("DATASOURCE_URL has no provider"),
    );
  });
});
