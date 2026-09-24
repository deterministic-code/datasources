import { describe, expect, it } from "vitest";
import {
  ChangeCasingMapper,
  Datasource,
  DictionaryMapper,
  EnrichedReadonlyRepository,
  FlatToNestedMapper,
  FlatToNestedReadonlyEnrichmentStrategy,
  HashsetOptimizedReadonlyEnrichmentStrategy,
  Join,
  QueryStatement,
  TransformMapper,
  WhereClause,
  eq,
  type IConnection,
  type IDatasource,
} from "@datasources/core";
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

class JoinRow {}

const seed = () => `
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
  CREATE TABLE roles (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL
  );
  INSERT INTO roles (id, name) VALUES
    (1, 'admin'),
    (2, 'editor'),
    (3, 'viewer');
  CREATE TABLE user_roles (
    user_id TEXT NOT NULL,
    role_id INTEGER NOT NULL
  );
  INSERT INTO user_roles (user_id, role_id) VALUES
    ('${ada}', 1),
    ('${ada}', 2),
    ('${grace}', 3);
`;

const names = (roles: UserRole[]) => roles.map((role) => role.Name).sort();

describe("users view roles", () => {
  it("fills roles with a hashset", async () => {
    const ds = new Datasource("sqlite", new SqliteDriver(), { filename: ":memory:" });
    const connection = await open(ds, seed());
    const casing = new ChangeCasingMapper("pascalCase", "snakeCase");
    const users = ds.createReadonlyRepository(UsersView, connection, {
      entityMapper: new DictionaryMapper([["UsersView", "user"]]),
      columnMappings: new DictionaryMapper([
        ["Key", "id"],
        ["Login", "username"],
        ["Active", "is_enabled"],
        ["Opened", "created"],
        ["UserType", "user_type_id"],
      ]),
    });
    const usersRepo = new EnrichedReadonlyRepository(
      users,
      new HashsetOptimizedReadonlyEnrichmentStrategy<UsersView>(),
    );
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
      query: new TransformMapper(
        (query) =>
          new QueryStatement({
            table: "user_roles",
            fields: ["user_roles.user_id AS user_id", "roles.name AS name"],
            where: query.where,
            joins: [Join.inner("roles", "roles.id", "user_roles.role_id")],
          }),
        (query) => query,
      ),
    });
    usersRepo.addEnriched("UserType", "UserType", typesRepo, "Id", "Name");
    usersRepo.addEnriched("Roles", "Key", rolesRepo, "UserId", undefined, true);
    const usersView = await usersRepo.readSingle(new WhereClause("Key", eq, ada));
    expect(usersView?.UserType).toBe("admin");
    expect(names(usersView?.Roles ?? [])).toEqual(["admin", "editor"]);
    expect(usersView?.Roles.every((role) => role.UserId === ada)).toBe(true);
  });

  it("fills roles from a join", async () => {
    const ds = new Datasource("sqlite", new SqliteDriver(), { filename: ":memory:" });
    const connection = await open(ds, seed());
    const source = ds.createReadonlyRepository(JoinRow, connection);
    const usersRepo = new EnrichedReadonlyRepository(
      ds.createReadonlyRepository(UsersView, connection),
      new FlatToNestedReadonlyEnrichmentStrategy(
        source,
        new FlatToNestedMapper<UsersView>({
          key: "Key",
          fields: {
            Key: "Key",
            Login: "Login",
            Active: "Active",
            Opened: "Opened",
            UserType: "UserType",
            Roles: {
              key: "RoleId",
              fields: { RoleId: "RoleId", UserId: "UserId", Name: "RoleName" },
            },
          },
        }),
        new TransformMapper(
          (row) =>
            Object.assign(new UsersView(), row, {
              Roles: row.Roles.map((role) => Object.assign(new UserRole(), role)),
            }),
          () => {
            throw new Error("UsersView hydrate does not reverse");
          },
        ),
        new QueryStatement({
          table: "user",
          fields: [
            "user.id AS Key",
            "user.username AS Login",
            "user.is_enabled AS Active",
            "user.created AS Opened",
            "user_type.name AS UserType",
            "user.id AS UserId",
            "user_roles.role_id AS RoleId",
            "roles.name AS RoleName",
          ],
          joins: [
            Join.left("user_type", "user_type.id", "user.user_type_id"),
            Join.left("user_roles", "user_roles.user_id", "user.id"),
            Join.left("roles", "roles.id", "user_roles.role_id"),
          ],
        }),
      ),
    );
    const items: UsersView[] = [];
    for await (const item of usersRepo.read()) items.push(item);
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

const open = async (ds: IDatasource, sql: string): Promise<IConnection> => {
  const connection = await ds.createConnection();
  await connection.open();
  const command = connection.createCommand();
  for (const statement of sql.split(";").map((part) => part.trim()).filter(Boolean))
    await command.execute(statement);
  return connection;
};
