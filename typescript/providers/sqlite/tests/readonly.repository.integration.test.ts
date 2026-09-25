import { describe, expect, it } from "vitest";
import {
  CRUDRepository,
  ChangeCasingMapper, Datasource,
  DictionaryMapper,
  PluralizerMapper,
  ReadonlyRepository,
  Where,
  createRepositoryOptions,
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
    const byId = await repo.readSingle(new Where("id", eq, 1));
    const disabled: User[] = [];
    for await (const row of repo.read(new Where("username", eq, "grace").and("is_enabled", eq, 0)))
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
    const byId = await repo.readSingle(new Where("Key", eq, ada));
    const byLogin: UserAccount[] = [];
    for await (const row of repo.read(new Where("Login", eq, "ada").and("Active", eq, true)))
      byLogin.push(row);
    const later: UserAccount[] = [];
    for await (const row of repo.read(new Where("Active", eq, true).and("Opened", gt, opened)))
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
  class UserType {
    Id = 0;
    Name = "";
  }

  class UsersView {
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
    const casing = new ChangeCasingMapper("pascalCase", "snakeCase");
    const usersRepo = ds.createReadonlyRepository(UsersView, connection, {
      entityMapper: new DictionaryMapper([["UsersView", "user"]]),
      columnMappings: new DictionaryMapper([
        ["Key", "id"],
        ["Login", "username"],
        ["Active", "is_enabled"],
        ["Opened", "created"],
        ["UserType", "user_type_id"],
      ]),
    }) as ReadonlyRepository<UsersView>;
    const typesRepo = ds.createReadonlyRepository(UserType, connection, {
      entityMapper: casing,
      columnMappings: casing,
    });
    usersRepo.addEnriched("UserType", "UserType", typesRepo, "Id", "Name");
    const usersView = await usersRepo.readSingle(new Where("Key", eq, ada));
    expect(usersView?.UserType).toBe("admin");
    expect(usersView?.Login).toBe("ada");
    expect(usersView?.Key).toBe(ada);
    expect(usersView?.Active).toBe(1);
    expect(usersView?.Opened).toBe("2020-01-01T00:00:00.000Z");
    expect(usersView === undefined || !("UserTypeId" in usersView)).toBe(true);
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

describe("user crud", () => {
  it("writes a user from the type", async () => {
    const ds = new Datasource("sqlite", new SqliteDriver(), { filename: ":memory:" });
    const connection = await open(ds, `
      CREATE TABLE User (
        id INTEGER PRIMARY KEY,
        is_enabled INTEGER NOT NULL,
        username TEXT NOT NULL
      );
    `);
    const usersRepo = new CRUDRepository(User, connection);
    const created = Object.assign(new User(), { username: "ada", is_enabled: true });
    await usersRepo.create(created);
    const byId = await usersRepo.readSingleById(created.id);
    const matches: User[] = [];
    for await (const row of usersRepo.read(new Where("username", eq, "ada").and("is_enabled", eq, true)))
      matches.push(row);
    created.username = "ada-v2";
    await usersRepo.update(created);
    const renamed = await usersRepo.readSingleById(created.id);
    await usersRepo.delete(created);
    const gone = await usersRepo.readSingleById(created.id);
    expect(created.id).toBe(1);
    expect(byId).toEqual({ id: 1, username: "ada", is_enabled: 1 });
    expect(matches.map((row) => row.username)).toEqual(["ada"]);
    expect(renamed?.username).toBe("ada-v2");
    expect(gone).toBeUndefined();
  });
});

describe("users view crud", () => {
  class UserType {
    Id = 0;
    Name = "";
  }

  class UserRole {
    UserId = 0;
    Name = "";
  }

  class UsersView {
    Key = 0;
    Login = "";
    Active = false;
    Opened = "";
    UserTypeId = 0;
    UserType = "";
    Version = 0;
    Roles: UserRole[] = [];
  }

  it("updates a user view when the version matches", async () => {
    const ds = new Datasource("sqlite", new SqliteDriver(), { filename: ":memory:" });
    const connection = await open(ds, `
      CREATE TABLE user_type (
        id INTEGER PRIMARY KEY,
        name TEXT NOT NULL
      );
      CREATE TABLE user (
        id INTEGER PRIMARY KEY,
        username TEXT NOT NULL,
        is_enabled INTEGER NOT NULL,
        created TEXT NOT NULL,
        user_type_id INTEGER NOT NULL,
        version INTEGER NOT NULL
      );
      CREATE TABLE user_role (
        user_id INTEGER NOT NULL,
        name TEXT NOT NULL
      );
    `);
    const casing = new ChangeCasingMapper("pascalCase", "snakeCase");
    const usersRepo = new CRUDRepository(connection, {
      ...createRepositoryOptions(UsersView),
      ids: ["Key"],
      readProperties: ["Key", "Login", "Active", "Opened", "UserTypeId", "Version"],
      entityMapper: new DictionaryMapper([["UsersView", "user"]]),
      columnMappings: new DictionaryMapper([
        ["Key", "id"],
        ["Login", "username"],
        ["Active", "is_enabled"],
        ["Opened", "created"],
        ["UserTypeId", "user_type_id"],
        ["Version", "version"],
      ]),
    });
    const userTypesRepo = new CRUDRepository(connection, {
      ...createRepositoryOptions(UserType),
      entityMapper: casing,
      columnMappings: casing,
    });
    const userRolesRepo = new CRUDRepository(connection, {
      ...createRepositoryOptions(UserRole),
      entityMapper: casing,
      columnMappings: casing,
    });
    usersRepo.addEnriched("UserType", "UserTypeId", userTypesRepo, "Id", "Name");
    usersRepo.addEnriched("Roles", "Key", userRolesRepo, "UserId", undefined, true);
    const admin = Object.assign(new UserType(), { Name: "admin" });
    await userTypesRepo.create(admin);
    const user = Object.assign(new UsersView(), {
      Login: "ada",
      Active: true,
      Opened: "2020-01-01T00:00:00.000Z",
      UserTypeId: admin.Id,
      Version: 1,
      Roles: [Object.assign(new UserRole(), { Name: "admin" }), Object.assign(new UserRole(), { Name: "editor" })],
    });
    await usersRepo.create(user);
    const loaded = await usersRepo.readSingle(new Where("Key", eq, user.Key));
    const version = loaded!.Version;
    loaded!.Login = "ada-v2";
    loaded!.Version = version + 1;
    const updated = await usersRepo.update(loaded!, new Where("Key", eq, loaded!.Key).and("Version", eq, version));
    const current = await usersRepo.readSingle(new Where("Key", eq, loaded!.Key));
    const stale = await usersRepo.update(loaded!, new Where("Key", eq, loaded!.Key).and("Version", eq, version));
    await usersRepo.delete(loaded!);
    const gone = await usersRepo.readSingleById(loaded!.Key);
    const roles: UserRole[] = [];
    for await (const role of userRolesRepo.read(new Where("UserId", eq, loaded!.Key)))
      roles.push(role);
    expect(admin.Id).toBe(1);
    expect(user.Key).toBe(1);
    expect(loaded?.UserType).toBe("admin");
    expect(loaded?.Roles.map((role) => role.Name).sort()).toEqual(["admin", "editor"]);
    expect(updated.rowsAffected).toBe(1);
    expect(current?.Version).toBe(2);
    expect(current?.Login).toBe("ada-v2");
    expect(stale.rowsAffected).toBe(0);
    expect(gone).toBeUndefined();
    expect(roles).toEqual([]);
  });
});
