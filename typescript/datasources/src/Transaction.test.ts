import { afterAll, afterEach, beforeEach, describe, expect, it } from "vitest";
import { DatasourceFactory, MockDriver } from "./index.ts";

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
  process.env.DATASOURCE_URL = "sqlite:///:memory:";
});

afterEach(() => DatasourceFactory.reset());

afterAll(() => {
  for (const key of envKeys) {
    if (savedEnv[key] === undefined) delete process.env[key];
    else process.env[key] = savedEnv[key];
  }
});

describe("Transaction", () => {
  it("start from default", async () => {
    const ds = DatasourceFactory.default;
    const connection = await ds.createConnection();
    await connection.open();
    const txn = await connection.startTransaction();
    expect(txn.isolation).toBe("serializable");
  });
});
