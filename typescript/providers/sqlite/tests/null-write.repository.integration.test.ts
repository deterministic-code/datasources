import { describe, expect, it } from "vitest";
import { CRUDRepository, ChangeCasingMapper, Datasource, createCRUDRepositoryOptions, type IConnection } from "@deterministic-code/datasources";
import { applySchema } from "../../../schema-spec.ts";
import { SqliteDriver } from "../src/SqliteDriver.ts";

class Item {
  Id = 0;
  Name = "";
  Created: Date | null = null;
  Version: Uint8Array | null = null;
}

const open = async (): Promise<IConnection> => {
  const ds = new Datasource("sqlite", new SqliteDriver(), { DATASOURCE_URL: "sqlite:///:memory:" });
  const connection = await ds.createConnection();
  await connection.open();
  return connection;
};

const bytes = (value: Uint8Array | null): number[] => [...(value ?? new Uint8Array())];

describe("null write", () => {
  it("omits null datetime and version", async () => {
    const connection = await open();
    await applySchema(connection, "null-write-item");
    const casing = new ChangeCasingMapper("pascalCase", "snakeCase");
    const repo = new CRUDRepository(connection, {
      ...createCRUDRepositoryOptions(Item),
      entityMapper: casing,
      propertyMappings: casing,
      ids: ["Id"],
    });
    const omitted = await repo.create(Object.assign(new Item(), { Name: "ada" }));
    expect(omitted.Id).toBeGreaterThan(0);
    const stored = await repo.readSingleById(omitted.Id);
    omitted.Name = "ada-v2";
    const updated = await repo.update(omitted);
    expect(updated.rowsAffected).toBe(1);
    const kept = await repo.readSingleById(omitted.Id);
    const explicit = Object.assign(new Item(), {
      Name: "grace",
      Created: new Date("2024-06-01T00:00:00.000Z"),
      Version: new Uint8Array([9, 9]),
    });
    const writtenCreate = await repo.create(explicit);
    expect(writtenCreate.Id).toBeGreaterThan(0);
    const written = await repo.readSingleById(explicit.Id);

    expect(stored?.Created).toBe("2020-01-01T00:00:00.000Z");
    expect(bytes(stored?.Version ?? null)).toEqual([1, 2]);
    expect(kept?.Name).toBe("ada-v2");
    expect(kept?.Created).toBe(stored?.Created);
    expect(bytes(kept?.Version ?? null)).toEqual(bytes(stored?.Version ?? null));
    expect(written?.Created).toBe("2024-06-01T00:00:00.000Z");
    expect(bytes(written?.Version ?? null)).toEqual([9, 9]);
  });
});
