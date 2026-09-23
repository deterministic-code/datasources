import { describe, expect, it } from "vitest";
import {
  DictionaryMapper,
  PluralizerMapper,
  ReadonlyRepository,
  WhereClause,
  eq,
  gt,
} from "@datasources/core";
import { SqliteConnection } from "../src/connection.ts";
import { SqliteDriver } from "../src/SqliteDriver.ts";

const ada = "11111111-1111-1111-1111-111111111111";
const grace = "22222222-2222-2222-2222-222222222222";
const lin = "33333333-3333-3333-3333-333333333333";

class User {
  id = 0;
  username = "";
  is_enabled = 0;
}

class UserAccount {
  Key = "";
  Login = "";
  Active = 0;
  Opened = "";
}

describe("readonly repository", () => {
  it("reads users by id and by username and enabled", async () => {
    const connection = await open(`
      CREATE TABLE users (
        id INTEGER PRIMARY KEY,
        username TEXT NOT NULL,
        is_enabled INTEGER NOT NULL
      );
      INSERT INTO users (id, username, is_enabled) VALUES
        (1, 'ada', 1),
        (2, 'grace', 0);
    `);
    const repo = new ReadonlyRepository(connection, {
      type: User,
      entityMapper: new PluralizerMapper(),
    });
    const byId = await repo.readSingle(new WhereClause("id", eq, 1));
    const disabled: User[] = [];
    for await (const row of repo.read(new WhereClause("username", eq, "grace").and("is_enabled", eq, 0)))
      disabled.push(row);
    expect(byId?.username).toBe("ada");
    expect(byId?.is_enabled).toBe(1);
    expect(disabled.length).toBe(1);
    expect(disabled[0]?.id).toBe(2);
    expect(disabled[0]?.is_enabled).toBe(0);
  });

  it("reads user by guid username and created", async () => {
    const connection = await open(`
      CREATE TABLE user (
        id TEXT PRIMARY KEY,
        username TEXT NOT NULL,
        is_enabled INTEGER NOT NULL,
        created TEXT NOT NULL
      );
      INSERT INTO user (id, username, is_enabled, created) VALUES
        ('${ada}', 'ada', 1, '2020-01-01T00:00:00.000Z'),
        ('${grace}', 'grace', 1, '2024-06-01T00:00:00.000Z'),
        ('${lin}', 'lin', 0, '2024-06-01T00:00:00.000Z');
    `);
    const repo = new ReadonlyRepository(connection, {
      type: UserAccount,
      entity: "user",
      columnMappings: new DictionaryMapper([
        ["Key", "id"],
        ["Login", "username"],
        ["Active", "is_enabled"],
        ["Opened", "created"],
      ]),
      rowTypeConverters: new SqliteDriver().rowTypeConverters,
    });
    const opened = new Date("2024-01-01T00:00:00.000Z");
    const byId = await repo.readSingle(new WhereClause("Key", eq, ada));
    const byLogin: UserAccount[] = [];
    for await (const row of repo.read(new WhereClause("Login", eq, "ada").and("Active", eq, true)))
      byLogin.push(row);
    const later: UserAccount[] = [];
    for await (const row of repo.read(new WhereClause("Active", eq, true).and("Opened", gt, opened)))
      later.push(row);
    expect(byId?.Login).toBe("ada");
    expect(byId?.Key).toBe(ada);
    expect(byId?.Active).toBe(1);
    expect(byId?.Opened).toBe("2020-01-01T00:00:00.000Z");
    expect(byLogin.length).toBe(1);
    expect(byLogin[0]?.Login).toBe("ada");
    expect(later.length).toBe(1);
    expect(later[0]?.Login).toBe("grace");
  });
});

const open = async (sql: string): Promise<SqliteConnection> => {
  const connection = SqliteConnection.from({ filename: ":memory:" });
  await connection.open();
  const command = connection.createCommand();
  for (const statement of sql.split(";").map((part) => part.trim()).filter(Boolean))
    await command.execute(statement);
  return connection;
};
