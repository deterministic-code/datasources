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
  TransformMapper,
  Where,
  eq,
  execute,
  Native,
  type IConnection,
  type IDatasource,
} from "@datasources/core";
import { tables } from "../../../schema-spec.ts";
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
    const connection = await open(ds, "user-roles-read", seed());
    const casing = new ChangeCasingMapper("pascalCase", "snakeCase");
    const strategy = new HashsetOptimizedReadonlyEnrichmentStrategy<UsersView>();
    const usersRepo = new ReadonlyRepository(connection, {
      type: UsersView,
      entityMapper: new DictionaryMapper([["UsersView", "user"]]),
      columnMappings: new DictionaryMapper([
        ["Key", "id"],
        ["Login", "username"],
        ["Active", "is_enabled"],
        ["Opened", "created"],
        ["UserType", "user_type_id"],
      ]),
    });
    const typesRepo = ds.createReadonlyRepository(UserType, connection, {
      entityMapper: casing,
      columnMappings: casing,
    });
    const rolesRepo = ds.createReadonlyRepository(UserRole, connection, {
      entity: "user_roles",
      columnMappings: new DictionaryMapper([
        ["UserId", "user_id"],
        ["Name", "name"],
      ]),
      select: new TransformMapper(
        (query) =>
          new SelectStatement({
            table: "user_roles",
            fields: ["user_roles.user_id AS user_id", "roles.name AS name"],
            where: query.where,
            joins: [Join.inner("roles", "roles.id", "user_roles.role_id")],
          }),
        (query) => query,
      ),
    });
    usersRepo.addEnriched({ into: "UserType", from: "UserType", by: "Id", select: "Name" }, typesRepo, strategy);
    usersRepo.addEnriched({ into: "Roles", from: "Key", by: "UserId", many: true }, rolesRepo, strategy);
    const usersView = await usersRepo.readSingle(new Where("Key", eq, ada));
    expect(usersView?.UserType).toBe("admin");
    expect(names(usersView?.Roles ?? [])).toEqual(["admin", "editor"]);
    expect(usersView?.Roles.every((role) => role.UserId === ada)).toBe(true);
  });

  it("fills roles from a join", async () => {
    const ds = new Datasource("sqlite", new SqliteDriver(), { DATASOURCE_URL: "sqlite:///:memory:" });
    const connection = await open(ds, "user-roles-read", seed());
    const usersRepo = new ReadonlyRepository(connection, {
      type: UsersView,
      columnMappings: new DictionaryMapper([
        ["Key", "id"],
        ["Login", "username"],
        ["Active", "is_enabled"],
        ["Opened", "created"],
        ["UserType", "user_type.name"],
        ["UserId", "user_id"],
        ["RoleId", "role_id"],
        ["Name", "roles.name"],
      ]),
    });
    usersRepo.addEnriched(
      { into: "Roles", from: "Key", by: "UserId", many: true },
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
    for await (const item of usersRepo.read(
      new SelectStatement({
        table: "user",
        fields: [
          "user.id",
          "user.username",
          "user.is_enabled",
          "user.created",
          "user_type.name",
          "user_roles.user_id",
          "user_roles.role_id",
          "roles.name",
        ],
        joins: [
          Join.left("user_type", "user_type.id", "user.user_type_id"),
          Join.left("user_roles", "user_roles.user_id", "user.id"),
          Join.left("roles", "roles.id", "user_roles.role_id"),
        ],
      }),
    ))
      items.push(item);
    const found = (login: string) => items.find((item) => item.Login === login);
    const adaView = found("ada");
    const graceView = found("grace");
    const linView = found("lin");
    expect(items).toHaveLength(3);
    expect(adaView?.UserType).toBe("admin");
    expect(names(adaView?.Roles ?? [])).toEqual(["admin", "editor"]);
    expect(adaView?.Roles.every((role) => role.UserId === ada)).toBe(true);
    expect(names(graceView?.Roles ?? [])).toEqual(["viewer"]);
    expect(linView?.Roles ?? []).toEqual([]);
  });
});

const quote = (id: string): string => `"${id.replaceAll('"', '""')}"`;

const open = async (ds: IDatasource, schema: string, sql = ""): Promise<IConnection> => {
  const connection = await ds.createConnection();
  await connection.open();
  const command = connection.createCommand();
  for (const statement of tables(schema, quote))
    await execute(command, statement);
  for (const statement of sql.split(";").map((part) => part.trim()).filter(Boolean))
    await command.executeNative(new Native(statement));
  return connection;
};
