import { describe, expect, it } from "vitest";
import {
  ChangeCasingMapper, Datasource,
  DictionaryMapper, EnrichedReadonlyRepository,
  PluralizerMapper,
  WhereClause,
  eq,
  gt,
  type IConnection,
  type IDatasource,
} from "@datasources/core";
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
    const ds = new Datasource("sqlite", new SqliteDriver(), { filename: ":memory:" });
    const connection = await open(ds, `
      CREATE TABLE users (
        id INTEGER PRIMARY KEY,
        username TEXT NOT NULL,
        is_enabled INTEGER NOT NULL
      );
      INSERT INTO users (id, username, is_enabled) VALUES
        (1, 'ada', 1),
        (2, 'grace', 0);
    `);
    const repo = ds.createReadonlyRepository(User, connection, {
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
    const ds = new Datasource("sqlite", new SqliteDriver(), { filename: ":memory:" });
    const connection = await open(ds, `
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
    const repo = ds.createReadonlyRepository(UserAccount, connection, {
      entity: "user",
      columnMappings: new DictionaryMapper([
        ["Key", "id"],
        ["Login", "username"],
        ["Active", "is_enabled"],
        ["Opened", "created"],
      ]),
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

describe("user view", () => {
  class UserAccount {
    Key = "";
    Login = "";
    Active = 0;
    Opened = "";
    UserTypeId = 0;
    UserType = "";
  }

  class UserType {
    Id = 0;
    Name = "";
  }

  class UserView {
    Key = "";
    Login = "";
    Active = 0;
    Opened = "";
    UserType = "";
  }

  it("fills user type on a user view", async () => {
    const ds = new Datasource("sqlite", new SqliteDriver(), { filename: ":memory:" });
    const connection = await open(ds, `
      CREATE TABLE user_type (
        id INTEGER PRIMARY KEY,
        name TEXT NOT NULL
      );
      INSERT INTO user_type (id, name) VALUES
        (1, 'admin'),
        (2, 'user'),
        (3, 'service');
      CREATE TABLE user (
        id TEXT PRIMARY KEY,
        username TEXT NOT NULL,
        is_enabled INTEGER NOT NULL,
        created TEXT NOT NULL,
        user_type_id INTEGER NOT NULL
      );
      INSERT INTO user (id, username, is_enabled, created, user_type_id) VALUES
        ('${ada}', 'ada', 1, '2020-01-01T00:00:00.000Z', 1),
        ('${grace}', 'grace', 1, '2024-06-01T00:00:00.000Z', 2),
        ('${lin}', 'lin', 0, '2024-06-01T00:00:00.000Z', 3);
    `);
    const users = ds.createReadonlyRepository(UserAccount, connection, {
      entity: "user",
      readColumns: ["Key", "Login", "Active", "Opened", "UserTypeId"],
      columnMappings: new DictionaryMapper([
        ["Key", "id"],
        ["Login", "username"],
        ["Active", "is_enabled"],
        ["Opened", "created"],
        ["UserTypeId", "user_type_id"],
      ]),
    });
    const casing = new ChangeCasingMapper("pascalCase", "snakeCase");
    const types = ds.createReadonlyRepository(UserType, connection, {
      entity: "UserType",
      entityMapper: casing,
      columnMappings: casing,
    });
    const enriched = new EnrichedReadonlyRepository(users);
    enriched.addEnriched("UserType", "UserTypeId", types, "Id", "Name");
    const account = await enriched.readSingle(new WhereClause("Key", eq, ada));
    const usersView = new UserView();
    usersView.Key = account?.Key ?? "";
    usersView.Login = account?.Login ?? "";
    usersView.Active = account?.Active ?? 0;
    usersView.Opened = account?.Opened ?? "";
    usersView.UserType = account?.UserType ?? "";
    expect(account?.UserTypeId).toBe(1);
    expect(usersView.UserType).toBe("admin");
    expect(usersView.Login).toBe("ada");
    expect(usersView.Key).toBe(ada);
    expect(usersView.Active).toBe(1);
    expect(usersView.Opened).toBe("2020-01-01T00:00:00.000Z");
    expect("UserTypeId" in usersView).toBe(false);
  });
});

const open = async (ds: IDatasource, sql: string): Promise<IConnection> => {
  const connection = await ds.createConnection();
  await connection.open();
  const command = connection.createCommand();
  for (const statement of sql.split(";").map((part) => part.trim()).filter(Boolean))
    await command.execute(statement);
  return connection;
};
