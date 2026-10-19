import { beforeEach, describe, expect, it } from "vitest";
import {
  ChangeCasingMapper,
  Datasource,
  DatasourceFactory,
  Where,
  createCRUDRepositoryOptions,
  eq,
  type ICRUDRepository,
  type IConnection,
  type IDatasource,
  type IReadonlyRepository,
} from "@deterministic-code/datasources";
import { applySchema } from "../../../schema-spec.ts";
import { SqliteDriver } from "../src/SqliteDriver.ts";

class User {
  Id = 0;
  Username = "";
  IsEnabled = false;
}

const open = async (ds: IDatasource): Promise<IConnection> => {
  const connection = await ds.createConnection();
  await connection.open();
  return connection;
};

const options = () => ({
  ...createCRUDRepositoryOptions(User),
  propertyMappings: new ChangeCasingMapper("pascalCase", "snakeCase"),
  ids: ["Id"],
});

const register = (ds: IDatasource) => {
  const mapping = options();
  ds.repositories.register(
    "User",
    "users",
    (connection) => ds.createRepository(User, connection, mapping),
    (connection) => ds.createReadonlyRepository(User, connection, mapping),
  );
};

declare module "@deterministic-code/datasources" {
  interface IDatasource {
    readonly users: ICRUDRepository<User>;
  }
}

describe("datasource plural", () => {
  beforeEach(() => {
    DatasourceFactory.reset();
    DatasourceFactory.register("sqlite", new SqliteDriver());
    delete process.env.INSTALLED_PROVIDERS;
    delete process.env.DATASOURCE_URL;
    process.env.PROVIDER = "sqlite";
  });

  it("creates using dynamic after register", async () => {
    const ds = Datasource.default;
    const connection = await open(ds);
    await applySchema(connection, "user-pascal");
    register(ds);
    const created = await ds.users.create(Object.assign(new User(), { Username: "ada", IsEnabled: true }));
    const loaded = await ds.users.readSingle();
    const updated = await ds.users.update(created);
    const deleted = await ds.users.delete(created);
    expect(created.Id).toBeGreaterThan(0);
    expect(loaded?.Username).toBe("ada");
    expect(updated.rowsAffected).toBe(1);
    expect(deleted.rowsAffected).toBe(1);
  });

  it("writes a user through the plural name", async () => {
    const ds = Datasource.default as Datasource & { users: ICRUDRepository<User> };
    const connection = await open(ds);
    await applySchema(connection, "user-pascal");
    register(ds);
    const created = await ds.users.create(Object.assign(new User(), { Username: "ada", IsEnabled: true }));
    const byId = await ds.users.readSingleById(created.Id);
    const matches: User[] = [];
    for await (const row of ds.users.read(new Where("Username", eq, "ada").and("IsEnabled", eq, true)))
      matches.push(row);
    created.Username = "ada-v2";
    const updated = await ds.users.update(created);
    const renamed = await ds.users.readSingleById(created.Id);
    const removed = await ds.users.delete(created);
    const gone = await ds.users.readSingleById(created.Id);
    expect(created.Id).toBeGreaterThan(0);
    expect(updated.rowsAffected).toBe(1);
    expect(removed.rowsAffected).toBe(1);
    expect(byId).toEqual({ Id: created.Id, Username: "ada", IsEnabled: true });
    expect(matches.map((row) => row.Username)).toEqual(["ada"]);
    expect(renamed?.Username).toBe("ada-v2");
    expect(gone).toBeUndefined();
  });

  it("reads through access createReadonly", async () => {
    const ds = Datasource.default as Datasource & { users: ICRUDRepository<User> };
    const connection = await open(ds);
    await applySchema(connection, "user-pascal");
    register(ds);
    const created = await ds.users.create(Object.assign(new User(), { Username: "ada", IsEnabled: true }));
    const usersRepo = ds.repositories.access("users", connection).createReadonly() as IReadonlyRepository<User>;
    const loaded = await usersRepo.readSingle(new Where("Id", eq, created.Id));
    expect(loaded?.Username).toBe("ada");
    expect(loaded?.IsEnabled).toBe(true);
    expect(loaded?.Id).toBe(created.Id);
  });
});
