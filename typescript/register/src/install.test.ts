import {
  Datasource,
  ProviderNotInstalledError,
} from "@datasources/core";
import { JsonDatasource } from "@datasources/json";
import { MemoryDatasource } from "@datasources/memory";
import { afterAll, afterEach, beforeEach, describe, expect, it } from "vitest";
import { install } from "./install.ts";
import { providerNames } from "./providerNames.ts";

const envKeys = [
  "INSTALLED_PROVIDERS",
  "providers",
  "DATABASE_URL",
  "url",
  "DATABASE_SCHEMA",
  "provider",
] as const;
const savedEnv = Object.fromEntries(envKeys.map((key) => [key, process.env[key]]));

beforeEach(() => {
  for (const key of envKeys) delete process.env[key];
});

afterEach(() => Datasource.reset());

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
        providers: "sqlite, json, sqlite",
        provider: "postgres",
      }),
    ).toEqual(["sqlite", "json", "postgres"]);
    expect(
      providerNames({
        INSTALLED_PROVIDERS: "json,memory",
        url: "json://",
      }),
    ).toEqual(["json", "memory"]);
    expect(providerNames({ provider: " sqlite " })).toEqual(["sqlite"]);
    expect(providerNames({ providers: {} })).toEqual([]);
    expect(providerNames({})).toEqual([]);
  });
});

describe("install", () => {
  it("registers listed factories and sets defaults", async () => {
    await install({
      providers: "json,memory",
      provider: "memory",
      filename: ":memory:",
    });
    expect(Datasource.registered().sort()).toEqual(["json", "memory"]);
    expect(Datasource.create().provider).toBe("memory");
    expect(Datasource.create("json").provider).toBe("json");
  });

  it("registers INSTALLED_PROVIDERS", async () => {
    await install({
      INSTALLED_PROVIDERS: "json,memory",
      provider: "memory",
    });
    expect(Datasource.registered().sort()).toEqual(["json", "memory"]);
    expect(Datasource.create().provider).toBe("memory");
  });

  it("creates from INSTALLED_PROVIDERS in the environment", () => {
    process.env.INSTALLED_PROVIDERS = "json,memory";
    process.env.provider = "memory";
    expect(Datasource.create().provider).toBe("memory");
    expect(Datasource.registered().sort()).toEqual(["json", "memory"]);
  });

  it("throws when the env package cannot be required", () => {
    process.env.provider = "sqlite";
    expect(() => Datasource.create()).toThrow(ProviderNotInstalledError);
  });

  it("throws when the name is unknown", async () => {
    await expect(install({ provider: "nope" })).rejects.toBeInstanceOf(
      ProviderNotInstalledError,
    );
  });

  it("throws when the package cannot be loaded", async () => {
    await expect(
      install({ provider: "json" }, async () => {
        throw new Error("missing");
      }),
    ).rejects.toBeInstanceOf(ProviderNotInstalledError);
  });

  it("rethrows ProviderNotInstalledError from a loader", async () => {
    await expect(
      install({ provider: "json" }, async () => {
        throw new ProviderNotInstalledError("json");
      }),
    ).rejects.toBeInstanceOf(ProviderNotInstalledError);
  });

  it("throws when the export has no factory", async () => {
    await expect(install({ provider: "json" }, async () => ({}))).rejects.toBeInstanceOf(
      ProviderNotInstalledError,
    );
  });

  it("accepts a matching factory export", async () => {
    await install({ provider: "json" }, async () => ({
      JsonDatasource: { instance: JsonDatasource.instance },
    }));
    expect(Datasource.create("json").provider).toBe("json");
    await install({ provider: "memory" }, async () => ({
      MemoryDatasource: { instance: MemoryDatasource.instance },
    }));
    expect(Datasource.create("memory").provider).toBe("memory");
  });
});
