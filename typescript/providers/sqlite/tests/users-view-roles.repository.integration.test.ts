import { describe, expect, it } from "vitest";
import {
  ChangeCasingMapper,
  Datasource,
  DictionaryMapper,
  HashsetOptimizedReadonlyEnrichmentStrategy,
  NestedToObjectEnrichmentStrategy,
  Join,
  ReadonlyRepository,
  SelectStatement,
  Where,
  eq,
  Native,
  type IConnection,
  type IDatasource,
} from "@deterministic-code/datasources";
import { applySchema } from "../../../schema-spec.ts";
import { SqliteDriver } from "../src/SqliteDriver.ts";

const ada = "11111111-1111-1111-1111-111111111111";
const grace = "22222222-2222-2222-2222-222222222222";
const lin = "33333333-3333-3333-3333-333333333333";

class UserType {
  Id = 0;
  Name = "";
}

class UserRole {
  UserId = "";
  Name = "";
}

class UsersView {
  Key = "";
  Login = "";
  Active = 0;
  Opened = "";
  UserType = "";
  Roles: UserRole[] = [];
}

const seed = () => `
  INSERT INTO user_type (id, name) VALUES
    (1, 'admin'),
    (2, 'user'),
    (3, 'service');
  INSERT INTO user (id, username, is_enabled, created, user_type_id) VALUES
    ('${ada}', 'ada', 1, '2020-01-01T00:00:00.000Z', 1),
    ('${grace}', 'grace', 1, '2024-06-01T00:00:00.000Z', 2),
    ('${lin}', 'lin', 0, '2024-06-01T00:00:00.000Z', 3);
  INSERT INTO roles (id, name) VALUES
    (1, 'admin'),
    (2, 'editor'),
    (3, 'viewer');
  INSERT INTO user_roles (user_id, role_id) VALUES
    ('${ada}', 1),
    ('${ada}', 2),
    ('${grace}', 3);
`;

const names = (roles: UserRole[]) => roles.map((role) => role.Name).sort();

describe("users view roles", () => {
  it("fills roles with a hashset", async () => {
    const ds = new Datasource("sqlite", new SqliteDriver(), { DATASOURCE_URL: "sqlite:///:memory:" });
    const connection = await open(ds);
    await applySchema(connection, "user-roles-read");
    await executeSql(connection, seed());
    const casing = new ChangeCasingMapper("pascalCase", "snakeCase");
    const strategy = new HashsetOptimizedReadonlyEnrichmentStrategy<UsersView>();
    const usersRepo = new ReadonlyRepository(connection, {
      type: UsersView,
      entityMapper: new DictionaryMapper([["UsersView", "user"]]),
      propertyMappings: new DictionaryMapper([
        ["Key", "id"],
        ["Login", "username"],
        ["Active", "is_enabled"],
        ["Opened", "created"],
        ["UserType", "user_type_id"],
      ]),
    });
    const typesRepo = ds.createReadonlyRepository(UserType, connection, {
      entityMapper: casing,
      propertyMappings: casing,
    });
    const rolesRepo = ds.createReadonlyRepository(UserRole, connection, {
      entityMapper: new DictionaryMapper([["UserRole", "user_roles"]]),
      propertyMappings: new DictionaryMapper([
        ["UserId", "user_id"],
        ["Name", "name"],
      ]),
    });
    usersRepo.addEnriched({ destinationProperty: "UserType", sourceProperty: "UserType", relatedKey: "Id", relatedProperty: "Name" }, typesRepo, strategy);
    usersRepo.addEnriched({ destinationProperty: "Roles", sourceProperty: "Key", relatedKey: "UserId", type: "list" }, rolesRepo, strategy);
    const usersView = await usersRepo.readSingle(new Where("Key", eq, ada));
    expect(usersView?.UserType).toBe("admin");
    expect(names(usersView?.Roles ?? [])).toEqual(["admin", "editor"]);
    expect(usersView?.Roles.every((role) => role.UserId === ada)).toBe(true);
  });

  it("fills roles from a join", async () => {
    const ds = new Datasource("sqlite", new SqliteDriver(), { DATASOURCE_URL: "sqlite:///:memory:" });
    const connection = await open(ds);
    await applySchema(connection, "user-roles-read");
    await executeSql(connection, seed());
    const usersRepo = new ReadonlyRepository(connection, {
      type: UsersView,
      entity: "User",
      entityMapper: new DictionaryMapper([
        ["User", "user"],
        ["UserType", "user_type"],
        ["UserRole", "user_roles"],
        ["Role", "roles"],
      ]),
      propertyMappings: new DictionaryMapper([
        ["Key", "id"],
        ["Login", "username"],
        ["Active", "is_enabled"],
        ["Opened", "created"],
        ["UserType", "user_type.name"],
        ["UserTypeId", "user_type_id"],
        ["UserId", "user_id"],
        ["RoleId", "role_id"],
        ["Name", "roles.name"],
        ["Id", "id"],
      ]),
    });
    const joined = new SelectStatement({
      table: "User",
      fields: [
        "User.Key",
        "User.Login",
        "User.Active",
        "User.Opened",
        "User.UserType",
        "UserRole.UserId",
        "UserRole.RoleId",
        "Role.Name",
      ],
      joins: [
        Join.left("UserType", "UserType.Id", "User.UserTypeId"),
        Join.left("UserRole", "UserRole.UserId", "User.Key"),
        Join.left("Role", "Role.Id", "UserRole.RoleId"),
      ],
    });
    usersRepo.addEnriched(
      { destinationProperty: "Roles", sourceProperty: "Key", relatedKey: "UserId", type: "list" },
      usersRepo,
      new NestedToObjectEnrichmentStrategy<UsersView>({
        key: "Key",
        fields: {
          Key: "Key",
          Login: "Login",
          Active: "Active",
          Opened: "Opened",
          UserType: "UserType",
          Roles: {
            key: "RoleId",
            fields: { RoleId: "RoleId", UserId: "UserId", Name: "Name" },
          },
        },
      }),
    );
    const items: UsersView[] = [];
    for await (const item of usersRepo.read(joined)) items.push(item);
    const found = (login: string) => items.find((item) => item.Login === login);
    const adaView = found("ada");
    const graceView = found("grace");
    const linView = found("lin");
    expect(items).toHaveLength(3);
    expect(adaView).toMatchObject({
      Key: ada,
      Login: "ada",
      Active: true,
      Opened: "2020-01-01T00:00:00.000Z",
      UserType: "admin",
    });
    expect(names(adaView?.Roles ?? [])).toEqual(["admin", "editor"]);
    expect(adaView?.Roles.every((role) => role.UserId === ada)).toBe(true);
    expect(graceView).toMatchObject({
      Key: grace,
      Login: "grace",
      Active: true,
      Opened: "2024-06-01T00:00:00.000Z",
      UserType: "user",
    });
    expect(names(graceView?.Roles ?? [])).toEqual(["viewer"]);
    expect(graceView?.Roles.every((role) => role.UserId === grace)).toBe(true);
    expect(linView).toMatchObject({
      Key: lin,
      Login: "lin",
      Active: false,
      Opened: "2024-06-01T00:00:00.000Z",
      UserType: "service",
      Roles: [],
    });
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
