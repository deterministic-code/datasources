import { describe, expect, it } from "vitest";
import { CRUDRepository, Datasource, createRepositoryOptions, type IConnection } from "@datasources/core";
import { SqliteDriver } from "../src/SqliteDriver.ts";

class Item {
  id = 0;
  name = "";
  created: Date | null = null;
  version: Uint8Array | null = null;
}

const sql = `
  CREATE TABLE item (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL,
    created TEXT NOT NULL DEFAULT '2020-01-01T00:00:00.000Z',
    version BLOB NOT NULL DEFAULT X'0102'
  );
`;

const open = async (): Promise<IConnection> => {
  const ds = new Datasource("sqlite", new SqliteDriver(), { filename: ":memory:" });
  const connection = await ds.createConnection();
  await connection.open();
  const command = connection.createCommand();
  for (const statement of sql.split(";").map((part) => part.trim()).filter(Boolean))
    await command.execute(statement);
  return connection;
};

const bytes = (value: Uint8Array | null): number[] => [...(value ?? new Uint8Array())];

describe("null write", () => {
  it("omits null datetime and version", async () => {
    const connection = await open();
    const repo = new CRUDRepository(connection, { ...createRepositoryOptions(Item), entity: "item" });
    const omitted = Object.assign(new Item(), { name: "ada" });
    await repo.create(omitted);
    const stored = await repo.readSingleById(omitted.id);
    omitted.name = "ada-v2";
    await repo.update(omitted);
    const kept = await repo.readSingleById(omitted.id);
    const explicit = Object.assign(new Item(), {
      name: "grace",
      created: new Date("2024-06-01T00:00:00.000Z"),
      version: new Uint8Array([9, 9]),
    });
    await repo.create(explicit);
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
