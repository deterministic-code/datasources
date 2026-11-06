import { describe, expect, it } from "vitest";
import { CRUDRepository, ChangeCasingMapper, Datasource, Schema, createCRUDRepositoryOptions, type IConnection } from "@deterministic-code/datasources";
import { applySchema } from "../../../schema-spec.ts";
import { SqliteDriver } from "../src/SqliteDriver.ts";

class Item {
  Id = 0;
  Name = "";
  Created: Date | null = null;
  Version: string | null = null;
}

const driver = new SqliteDriver();
const versionOf = (value: Uint8Array): string => driver.rowTypeConverters.binary.convert(value) as string;

const open = async (): Promise<IConnection> => {
  const ds = new Datasource("sqlite", driver, { DATASOURCE_URL: "sqlite:///:memory:" });
  const connection = await ds.createConnection();
  await connection.open();
  return connection;
};

describe("null write", () => {
  it("omits null datetime and version", async () => {
    const connection = await open();
    await applySchema(connection, "null-write-item");
    const casing = new ChangeCasingMapper("pascalCase", "snakeCase");
    const repo = new CRUDRepository(connection, {
      ...createCRUDRepositoryOptions(Item, driver.rowTypeConverters),
      entityMapper: casing,
      propertyMappings: casing,
      ids: ["Id"],
      schema: new Schema({
        Id: { type: "integer", nullable: false, hasDefault: false, isId: true },
        Name: { type: "string", nullable: false, hasDefault: false },
        Created: { type: "datetime", nullable: false, hasDefault: true },
        Version: { type: "binary", nullable: false, hasDefault: true },
      }),
    });
    const omitted = await repo.create(Object.assign(new Item(), { Name: "ada" }));
    expect(omitted.Id).toBeGreaterThan(0);
    const stored = await repo.readSingleById(omitted.Id);
    stored!.Name = "ada-v2";
    const updated = await repo.update(stored);
    expect(updated.rowsAffected).toBe(1);
    const kept = await repo.readSingleById(omitted.Id);
    const explicit = Object.assign(new Item(), {
      Name: "grace",
      Created: new Date("2024-06-01T00:00:00.000Z"),
      Version: versionOf(new Uint8Array([9, 9])),
    });
    const writtenCreate = await repo.create(explicit);
    expect(writtenCreate.Id).toBeGreaterThan(0);
    const written = await repo.readSingleById(explicit.Id);

    expect(stored?.Created).toEqual(new Date("2020-01-01T00:00:00.000Z"));
    expect(stored?.Version).toBe(versionOf(new Uint8Array([1, 2])));
    expect(kept?.Name).toBe("ada-v2");
    expect(kept?.Created).toEqual(stored?.Created);
    expect(kept?.Version).toBe(stored?.Version);
    expect(written?.Created).toEqual(new Date("2024-06-01T00:00:00.000Z"));
    expect(written?.Version).toBe(versionOf(new Uint8Array([9, 9])));
  });
});
