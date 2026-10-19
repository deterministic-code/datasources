import { SqliteDriver } from "@deterministic-code/datasources-sqlite";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { describe, expect, it } from "vitest";
import { migrate } from "./migrate.ts";
import { sqlPath } from "./schema.ts";

describe("migrate", () => {
  it("applies generated sqlite sql", async () => {
    const connection = new SqliteDriver().connection({ DATASOURCE_URL: "sqlite:///:memory:" });
    await connection.open();
    await migrate(connection.createCommand());
    expect(sqlPath(resolve(dirname(fileURLToPath(import.meta.url)), "../../../.."))).toMatch(/migrations$/);
  });
});
