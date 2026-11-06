import { CRUDRepository, Datasource, Native, Schema, createCRUDRepositoryOptions, type IConnection } from "@deterministic-code/datasources";
import { SqliteDriver } from "@deterministic-code/datasources-sqlite";
import { describe, expect, it } from "vitest";

class UserStamp {
  id = 0;
  username = "";
  created: Date | null = null;
  updated: string | null = null;
}

const driver = new SqliteDriver();
const versionOf = (value: Uint8Array): string =>
  driver.rowTypeConverters.binary.convert(value) as string;

const open = async (): Promise<IConnection> => {
  const ds = new Datasource("sqlite", driver, { DATASOURCE_URL: "sqlite:///:memory:" });
  const connection = await ds.createConnection();
  await connection.open();
  await connection.createCommand().executeNative(new Native(`
    CREATE TABLE users (
      id INTEGER PRIMARY KEY,
      username TEXT NOT NULL,
      created TEXT NOT NULL DEFAULT '2020-01-01T00:00:00.000Z',
      updated BLOB NOT NULL DEFAULT X'0102'
    )
  `));
  return connection;
};

describe("null write", () => {
  it("omits null created and updated", async () => {
    const connection = await open();
    const repo = new CRUDRepository(connection, {
      ...createCRUDRepositoryOptions(UserStamp, driver.rowTypeConverters),
      entity: "users",
      schema: new Schema({
        id: { type: "integer", nullable: false, hasDefault: false, isId: true },
        username: { type: "string", nullable: false, hasDefault: false },
        created: { type: "datetime", nullable: false, hasDefault: true },
        updated: { type: "binary", nullable: false, hasDefault: true },
      }),
    });
    const omitted = Object.assign(new UserStamp(), { username: "ada" });
    await repo.create(omitted);
    const stored = await repo.readSingleById(omitted.id);
    stored!.username = "ada-v2";
    await repo.update(stored);
    const kept = await repo.readSingleById(omitted.id);
    const explicit = Object.assign(new UserStamp(), {
      username: "grace",
      created: new Date("2024-06-01T00:00:00.000Z"),
      updated: versionOf(new Uint8Array([9, 9])),
    });
    await repo.create(explicit);
    const written = await repo.readSingleById(explicit.id);

    expect(stored?.created).toEqual(new Date("2020-01-01T00:00:00.000Z"));
    expect(stored?.updated).toBe(versionOf(new Uint8Array([1, 2])));
    expect(kept?.username).toBe("ada-v2");
    expect(kept?.created).toEqual(stored?.created);
    expect(kept?.updated).toBe(stored?.updated);
    expect(written?.created).toEqual(new Date("2024-06-01T00:00:00.000Z"));
    expect(written?.updated).toBe(versionOf(new Uint8Array([9, 9])));
  });
});
