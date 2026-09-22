import { afterAll, afterEach, beforeEach, describe, expect, it } from "vitest";
import {
  DatasourceFactory,
  MockCommand,
  MockDriver,
  ProviderNotInstalledError,
} from "./index.ts";

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

afterEach(() => DatasourceFactory.reset());

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
    expect(driver.types).toEqual({});
    const connection = driver.connection();
    expect(driver.command(connection)).toBeInstanceOf(MockCommand);
    expect(driver.transaction(connection).isolation).toBe("serializable");
    expect(driver.transaction(connection, "readCommitted").isolation).toBe(
      "readCommitted",
    );
  });
});
