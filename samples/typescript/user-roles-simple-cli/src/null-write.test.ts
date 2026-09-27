import { CRUDRepository, Datasource, createRepositoryOptions, type IConnection } from "@datasources/core";
import { SqliteDriver } from "@datasources/sqlite";
import { describe, expect, it } from "vitest";

class UserStamp {
  id = 0;
  username = "";
  created: Date | null = null;
  updated: Uint8Array | null = null;
}

const bytes = (value: Uint8Array | null): number[] => [...(value ?? new Uint8Array())];

const open = async (): Promise<IConnection> => {
  const ds = new Datasource("sqlite", new SqliteDriver(), { filename: ":memory:" });
  const connection = await ds.createConnection();
  await connection.open();
  await connection.createCommand().execute(`
    CREATE TABLE users (
      id INTEGER PRIMARY KEY,
      username TEXT NOT NULL,
      created TEXT NOT NULL DEFAULT '2020-01-01T00:00:00.000Z',
      updated BLOB NOT NULL DEFAULT X'0102'
    )
  `);
  return connection;
};

describe("null write", () => {
  it("omits null created and updated", async () => {
    const connection = await open();
    const repo = new CRUDRepository(connection, { ...createRepositoryOptions(UserStamp), entity: "users" });
    const omitted = Object.assign(new UserStamp(), { username: "ada" });
    await repo.create(omitted);
    const stored = await repo.readSingleById(omitted.id);
    omitted.username = "ada-v2";
    await repo.update(omitted);
    const kept = await repo.readSingleById(omitted.id);
    const explicit = Object.assign(new UserStamp(), {
      username: "grace",
      created: new Date("2024-06-01T00:00:00.000Z"),
      updated: new Uint8Array([9, 9]),
    });
    await repo.create(explicit);
    const written = await repo.readSingleById(explicit.id);

    expect(stored?.created).toBe("2020-01-01T00:00:00.000Z");
    expect(bytes(stored?.updated ?? null)).toEqual([1, 2]);
    expect(kept?.username).toBe("ada-v2");
    expect(kept?.created).toBe(stored?.created);
    expect(bytes(kept?.updated ?? null)).toEqual(bytes(stored?.updated ?? null));
    expect(written?.created).toBe("2024-06-01T00:00:00.000Z");
    expect(bytes(written?.updated ?? null)).toEqual([9, 9]);
  });
});
