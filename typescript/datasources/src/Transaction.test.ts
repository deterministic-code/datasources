import { afterAll, afterEach, beforeEach, describe, expect, it } from "vitest";
import { Datasource, DatasourceFactory, MockDriver } from "./index.ts";

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

describe("Transaction", () => {
  it("commit or abort", async () => {
    const ds = Datasource.default;
    const connection = await ds.createConnection();
    await connection.open();
    const txn = await connection.startTransaction();
    const saved = true;
    if (saved) await txn.commit();
    else await txn.abort();
    expect(txn.isolation).toBe("serializable");
  });
});
