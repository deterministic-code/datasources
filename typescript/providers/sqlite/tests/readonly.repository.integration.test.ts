import { beforeEach, describe, expect, it } from "vitest";
import {
  CRUDRepository,
  ChangeCasingMapper, Datasource,
  DatasourceFactory,
  DictionaryMapper,
  PluralizerMapper,
  ReadonlyRepository,
  TypeConverter,
  Where,
  type Cells,
  createCRUDRepositoryOptions,
  eq,
  gt,
  Native,
  type IConnection,
  type IDatasource,
} from "@datasources/core";
import { applySchema } from "../../../schema-spec.ts";
import { SqliteDriver } from "../src/SqliteDriver.ts";

const ada = "11111111-1111-1111-1111-111111111111";
const grace = "22222222-2222-2222-2222-222222222222";
const lin = "33333333-3333-3333-3333-333333333333";

class User {
  id = 0;
  username = "";
  is_enabled = false;
}

class MappedUser {
  id = 0;
  username = "";
  is_enabled = false;
}

class UserConverter extends TypeConverter<MappedUser, Cells> {
  convert = (from: Cells): MappedUser => {
    const user = new MappedUser();
    user.id = from.id as number;
    user.username = from.username as string;
    user.is_enabled = from.is_enabled === true;
    return user;
  };

  reverseConvert = (to: MappedUser): Cells => ({
    id: to.id,
    username: to.username,
    is_enabled: to.is_enabled ? 1 : 0,
  });
}

class UserAccount {
  Key = "";
  Login = "";
  Active = false;
  Opened = "";
}

describe("readonly repository", () => {
  it("reads users by id and by username and enabled", async () => {
    const ds = new Datasource("sqlite", new SqliteDriver(), { DATASOURCE_URL: "sqlite:///:memory:" });
    const connection = await open(ds);
    await applySchema(connection, "minimal-users");
    await executeSql(connection, `
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
    expect(byId?.is_enabled).toBe(true);
    expect(disabled.length).toBe(1);
    expect(disabled[0]?.id).toBe(2);
    expect(disabled[0]?.is_enabled).toBe(false);
  });

  it("maps an object explicitly", async () => {
    const ds = new Datasource("sqlite", new SqliteDriver(), { DATASOURCE_URL: "sqlite:///:memory:" });
    const connection = await open(ds);
    await applySchema(connection, "minimal-users");
    await executeSql(connection, `
      INSERT INTO users (id, username, is_enabled) VALUES
        (1, 'ada', 1),
        (2, 'grace', 0);
    `);
    const usersRepo = ds.createReadonlyRepository(MappedUser, connection, {
      entity: "User",
      entityMapper: new PluralizerMapper(),
      readProperties: ["id", "username", "is_enabled"],
      objectConverter: new UserConverter(),
    });
    const user = await usersRepo.readSingle(new Where("id", eq, 1));
    expect(user?.id).toBe(1);
    expect(user?.username).toBe("ada");
    expect(user?.is_enabled).toBe(true);
  });

  it("reads user by guid username and created", async () => {
    const ds = new Datasource("sqlite", new SqliteDriver(), { DATASOURCE_URL: "sqlite:///:memory:" });
    const connection = await open(ds);
    await applySchema(connection, "user-account");
    await executeSql(connection, `
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
    expect(byId?.Active).toBe(true);
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
    Active = false;
    Opened = "";
    UserType = "";
  }

  it("fills user type on a user view", async () => {
    const ds = new Datasource("sqlite", new SqliteDriver(), { DATASOURCE_URL: "sqlite:///:memory:" });
    const connection = await open(ds);
    await applySchema(connection, "user-type-lookup");
    await executeSql(connection, `
      INSERT INTO user_type (id, name) VALUES
        (1, 'admin'),
        (2, 'user'),
        (3, 'service');
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
    usersRepo.addEnriched({ into: "UserType", from: "UserType", by: "Id", select: "Name" }, typesRepo);
    const usersView = await usersRepo.readSingle(new Where("Key", eq, ada));
    expect(usersView?.UserType).toBe("admin");
    expect(usersView?.Login).toBe("ada");
    expect(usersView?.Key).toBe(ada);
    expect(usersView?.Active).toBe(true);
    expect(usersView?.Opened).toBe("2020-01-01T00:00:00.000Z");
    expect(usersView === undefined || !("UserTypeId" in usersView)).toBe(true);
  });
});

const open = async (ds: IDatasource): Promise<IConnection> => {
  const connection = await ds.createConnection();
  await connection.open();
  return connection;
};

const executeSql = async (connection: IConnection, sql: string): Promise<void> => {
  const command = connection.createCommand();
  for (const statement of sql.split(";").map((part) => part.trim()).filter(Boolean))
    await command.executeNative(new Native(statement));
};

describe("user crud", () => {
  beforeEach(() => {
    DatasourceFactory.reset();
    DatasourceFactory.register("sqlite", new SqliteDriver());
    delete process.env.INSTALLED_PROVIDERS;
    delete process.env.DATASOURCE_URL;
    process.env.PROVIDER = "sqlite";
  });

  it("writes a user from the type", async () => {
    const ds = Datasource.default;
    const connection = await open(ds);
    await applySchema(connection, "user-pascal");
    const usersRepo = new CRUDRepository(User, connection);
    const created = await usersRepo.create(Object.assign(new User(), { username: "ada", is_enabled: true }));
    const byId = await usersRepo.readSingleById(created.id);
    const matches: User[] = [];
    for await (const row of usersRepo.read(new Where("username", eq, "ada").and("is_enabled", eq, true)))
      matches.push(row);
    created.username = "ada-v2";
    const updated = await usersRepo.update(created);
    const renamed = await usersRepo.readSingleById(created.id);
    const removed = await usersRepo.delete(created);
    const gone = await usersRepo.readSingleById(created.id);
    expect(created.id).toBeGreaterThan(0);
    expect(updated.rowsAffected).toBe(1);
    expect(removed.rowsAffected).toBe(1);
    expect(byId).toEqual({ id: created.id, username: "ada", is_enabled: true });
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
    const ds = new Datasource("sqlite", new SqliteDriver(), { DATASOURCE_URL: "sqlite:///:memory:" });
    const connection = await open(ds);
    await applySchema(connection, "user-view-version");
    const casing = new ChangeCasingMapper("pascalCase", "snakeCase");
    const usersRepo = new CRUDRepository(connection, {
      ...createCRUDRepositoryOptions(UsersView),
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
      ...createCRUDRepositoryOptions(UserType),
      entityMapper: casing,
      columnMappings: casing,
    });
    const userRolesRepo = new CRUDRepository(connection, {
      ...createCRUDRepositoryOptions(UserRole),
      entityMapper: casing,
      columnMappings: casing,
    });
    usersRepo.addEnriched({ into: "UserType", from: "UserTypeId", by: "Id", select: "Name" }, userTypesRepo);
    usersRepo.addEnriched({ into: "Roles", from: "Key", by: "UserId", many: true }, userRolesRepo);
    const admin = await userTypesRepo.create(Object.assign(new UserType(), { Name: "admin" }));
    expect(admin.Id).toBe(1);
    const user = Object.assign(new UsersView(), {
      Login: "ada",
      Active: true,
      Opened: "2020-01-01T00:00:00.000Z",
      UserTypeId: admin.Id,
      Version: 1,
      Roles: [Object.assign(new UserRole(), { Name: "admin" }), Object.assign(new UserRole(), { Name: "editor" })],
    });
    const created = await usersRepo.create(user);
    expect(created.Key).toBe(1);
    const loaded = await usersRepo.readSingle(new Where("Key", eq, user.Key));
    const version = loaded!.Version;
    loaded!.Login = "ada-v2";
    loaded!.Version = version + 1;
    const updated = await usersRepo.update(loaded!, new Where("Key", eq, loaded!.Key).and("Version", eq, version));
    const current = await usersRepo.readSingle(new Where("Key", eq, loaded!.Key));
    const stale = await usersRepo.update(loaded!, new Where("Key", eq, loaded!.Key).and("Version", eq, version));
    const removed = await usersRepo.delete(loaded!);
    expect(removed.rowsAffected).toBe(1);
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

describe("multi-tenant account", () => {
  class Account {
    tenant_id = 0;
    id = 0;
    name = "";
  }

  it("reads updates and deletes by tenant and id", async () => {
    const ds = new Datasource("sqlite", new SqliteDriver(), { DATASOURCE_URL: "sqlite:///:memory:" });
    const connection = await open(ds);
    await applySchema(connection, "multi-tenant");
    await executeSql(connection, `
      INSERT INTO account (tenant_id, id, name) VALUES
        (1, 1, 'ada'),
        (2, 1, 'grace');
    `);
    const accountsRepo = new CRUDRepository(connection, {
      ...createCRUDRepositoryOptions(Account),
      entity: "account",
      ids: ["tenant_id", "id"],
    });
    const ada = await accountsRepo.readSingleById({ tenant_id: 1, id: 1 });
    ada!.name = "ada-v2";
    const updated = await accountsRepo.update(ada!);
    const renamed = await accountsRepo.readSingleById({ tenant_id: 1, id: 1 });
    const removed = await accountsRepo.delete(ada!);
    const gone = await accountsRepo.readSingleById({ tenant_id: 1, id: 1 });
    const graceRow = await accountsRepo.readSingleById({ tenant_id: 2, id: 1 });
    expect(updated.rowsAffected).toBe(1);
    expect(removed.rowsAffected).toBe(1);
    expect(renamed?.name).toBe("ada-v2");
    expect(renamed?.tenant_id).toBe(1);
    expect(gone).toBeUndefined();
    expect(graceRow?.name).toBe("grace");
    expect(graceRow?.tenant_id).toBe(2);
  });
});
