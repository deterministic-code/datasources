import {
  Datasource,
  DatasourceFactory,
  ProviderNotInstalledError,
} from "@datasources/core";
import { afterAll, afterEach, beforeEach, describe, expect, it } from "vitest";
import { from, registerListed } from "./install.ts";
import { providerNames } from "./providerNames.ts";

const envKeys = ["INSTALLED_PROVIDERS", "DATASOURCE_URL", "DATABASE_SCHEMA", "DEFAULT_PROVIDER", "DEFAULT_DATASOURCE_URL", "PROVIDER"] as const;
const savedEnv = Object.fromEntries(envKeys.map((key) => [key, process.env[key]]));

beforeEach(() => {
  for (const key of envKeys) delete process.env[key];
});

afterEach(() => DatasourceFactory.reset());

afterAll(() => {
  for (const key of envKeys) {
    if (savedEnv[key] === undefined) delete process.env[key];
    else process.env[key] = savedEnv[key];
  }
});

describe("providerNames", () => {
  it("splits, de-dupes, and appends the default provider", () => {
    expect(
      providerNames({
        INSTALLED_PROVIDERS: "sqlite, json, sqlite",
        DATASOURCE_URL: "postgres://localhost",
      }),
    ).toEqual(["sqlite", "json", "postgres"]);
    expect(
      providerNames({
        INSTALLED_PROVIDERS: "json,memory",
        DATASOURCE_URL: "json://",
      }),
    ).toEqual(["json", "memory"]);
    expect(providerNames({ INSTALLED_PROVIDERS: " sqlite " })).toEqual(["sqlite"]);
    expect(providerNames({})).toEqual([]);
  });
});

describe("registerListed", () => {
  it("registers listed factories", () => {
    from({
      INSTALLED_PROVIDERS: "json,memory",
      DATASOURCE_URL: "memory://",
    });
    expect(DatasourceFactory.registered()).toEqual(["json", "memory"]);
    expect(
      Datasource.create({
        provider: "memory",
        database_url: "memory://",
      }).provider,
    ).toBe("memory");
    expect(Datasource.create({ provider: "json" }).provider).toBe("json");
  });

  it("defaults registers installed providers", () => {
    DatasourceFactory.defaults({
      INSTALLED_PROVIDERS: "json,memory",
      DATASOURCE_URL: "memory://",
    });
    expect(DatasourceFactory.registered()).toEqual(["json", "memory"]);
    expect(Datasource.create({ database_url: "memory://" }).provider).toBe("memory");
  });

  it("create does not take INSTALLED_PROVIDERS", () => {
    registerListed({ INSTALLED_PROVIDERS: "json,memory" });
    expect(
      Datasource.create({
        provider: "json",
        database_url: "json://",
      }).provider,
    ).toBe("json");
    expect(DatasourceFactory.registered()).toEqual(["json", "memory"]);
  });

  it("create reads connection from the environment", () => {
    process.env.INSTALLED_PROVIDERS = "json,memory";
    process.env.DATASOURCE_URL = "memory://";
    expect(Datasource.create().provider).toBe("memory");
    expect(DatasourceFactory.default.provider).toBe("memory");
    process.env.DEFAULT_PROVIDER = "json";
    process.env.DEFAULT_DATASOURCE_URL = "json://";
    expect(DatasourceFactory.default.provider).toBe("json");
    expect(DatasourceFactory.registered()).toEqual(["json", "memory"]);
  });

  it("throws when the package is missing or the name is unknown", () => {
    expect(() => registerListed({ INSTALLED_PROVIDERS: "sqlite" })).toThrow(
      ProviderNotInstalledError,
    );
    expect(() => registerListed({ INSTALLED_PROVIDERS: "nope" })).toThrow(
      ProviderNotInstalledError,
    );
  });
});
