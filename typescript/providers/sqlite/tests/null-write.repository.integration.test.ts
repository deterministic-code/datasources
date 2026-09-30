import { describe, expect, it } from "vitest";
import { CRUDRepository, Datasource, createCRUDRepositoryOptions, type IConnection } from "@datasources/core";
import { applySchema } from "../../../schema-spec.ts";
import { SqliteDriver } from "../src/SqliteDriver.ts";

class Item {
  id = 0;
  name = "";
  created: Date | null = null;
  version: Uint8Array | null = null;
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
    const repo = new CRUDRepository(connection, { ...createCRUDRepositoryOptions(Item), entity: "item" });
    const omitted = await repo.create(Object.assign(new Item(), { name: "ada" }));
    expect(omitted.id).toBeGreaterThan(0);
    const stored = await repo.readSingleById(omitted.id);
    omitted.name = "ada-v2";
    const updated = await repo.update(omitted);
    expect(updated.rowsAffected).toBe(1);
    const kept = await repo.readSingleById(omitted.id);
    const explicit = Object.assign(new Item(), {
      name: "grace",
      created: new Date("2024-06-01T00:00:00.000Z"),
      version: new Uint8Array([9, 9]),
    });
    const writtenCreate = await repo.create(explicit);
    expect(writtenCreate.id).toBeGreaterThan(0);
    const written = await repo.readSingleById(explicit.id);

    expect(stored?.created).toBe("2020-01-01T00:00:00.000Z");
    expect(bytes(stored?.version ?? null)).toEqual([1, 2]);
    expect(kept?.name).toBe("ada-v2");
    expect(kept?.created).toBe(stored?.created);
    expect(bytes(kept?.version ?? null)).toEqual(bytes(stored?.version ?? null));
    expect(written?.created).toBe("2024-06-01T00:00:00.000Z");
    expect(bytes(written?.version ?? null)).toEqual([9, 9]);
  });
});
