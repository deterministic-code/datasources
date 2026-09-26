import { describe, expect, it } from "vitest";
import { MemoryProvider } from "@datasources/memory";
import { collectAsync } from "./collectAsync.ts";
import { CRUDRepository } from "./CRUDRepository.ts";
import { LazyWriteEnrichmentStrategy } from "./LazyWriteEnrichmentStrategy.ts";
import type { IConnection } from "./IConnection.ts";

class UserView {
  id = 0;
  username = "";
  is_enabled = false;
  user_type_id = 0;
  user_type?: UserType;
  roles: Role[] | null = null;
  groups: Group[] | null = null;
}

class UserType {
  id = 0;
  name = "";
}

class Role {
  id = 0;
  name = "";
}

class Group {
  id = 0;
  name = "";
}

class UserRole {
  id = 0;
  user_id = 0;
  role_id = 0;
}

class UserGroup {
  id = 0;
  user_id = 0;
  group_id = 0;
}

const setup = async (): Promise<IConnection> => {
  const connection = new MemoryProvider().connection();
  await connection.open();
  return connection;
};

const ids = (values: number[]) => [...values].sort((left, right) => left - right);

describe("CRUDRepository user view", () => {
  it("creates, updates, and deletes through link rows", async () => {
    const connection = await setup();
    const usersRepo = new CRUDRepository(UserView, connection);
    const userTypesRepo = new CRUDRepository(UserType, connection);
    const rolesRepo = new CRUDRepository(Role, connection);
    const groupsRepo = new CRUDRepository(Group, connection);
    const userRolesRepo = new CRUDRepository(UserRole, connection);
    const userGroupsRepo = new CRUDRepository(UserGroup, connection);
    await userTypesRepo.create(Object.assign(new UserType(), { name: "admin" }));
    await rolesRepo.create(Object.assign(new Role(), { name: "admin" }));
    await rolesRepo.create(Object.assign(new Role(), { name: "editor" }));
    await groupsRepo.create(Object.assign(new Group(), { name: "engineering" }));
    await groupsRepo.create(Object.assign(new Group(), { name: "ops" }));
    const adminType = (await collectAsync(userTypesRepo.read()))[0];
    const roles = await collectAsync(rolesRepo.read());
    const groups = await collectAsync(groupsRepo.read());
    const adminRole = roles.find((role) => role.name === "admin");
    const editorRole = roles.find((role) => role.name === "editor");
    const engineering = groups.find((group) => group.name === "engineering");
    const ops = groups.find((group) => group.name === "ops");
    usersRepo.addEnriched({ into: "user_type", from: "user_type_id", by: "id" }, userTypesRepo);
    usersRepo.addLinked(
      { into: "roles", from: "id", by: "id", association: { parent: "user_id", related: "role_id" } },
      rolesRepo,
      userRolesRepo,
    );
    usersRepo.addLinked(
      { into: "groups", from: "id", by: "id", association: { parent: "user_id", related: "group_id" } },
      groupsRepo,
      userGroupsRepo,
    );
    await usersRepo.create(Object.assign(new UserView(), {
      username: "ada",
      is_enabled: true,
      user_type_id: adminType?.id,
      roles: [adminRole, editorRole],
      groups: [engineering],
    }));
    const created = (await collectAsync(usersRepo.read())).find((user) => user.username === "ada");
    expect(created?.id).toBeGreaterThan(0);
    const links = await collectAsync(userRolesRepo.read());
    expect(links.map((link) => link.user_id)).toEqual([created?.id, created?.id]);
    expect(ids(links.map((link) => link.role_id))).toEqual(ids([adminRole!.id, editorRole!.id]));
    expect((await collectAsync(userGroupsRepo.read())).map((link) => link.group_id)).toEqual([engineering?.id]);
    const loaded = await usersRepo.readSingleById(created!.id);
    expect(loaded?.user_type?.name).toBe("admin");
    expect(ids(loaded?.roles?.map((role) => role.id) ?? [])).toEqual(ids([adminRole!.id, editorRole!.id]));
    expect(loaded?.groups?.map((group) => group.name)).toEqual(["engineering"]);
    await usersRepo.update(Object.assign(new UserView(), {
      id: created?.id,
      username: "ada",
      is_enabled: true,
      user_type_id: adminType?.id,
      roles: [editorRole],
      groups: [engineering, ops],
    }));
    expect((await collectAsync(userRolesRepo.read())).map((link) => link.role_id)).toEqual([editorRole?.id]);
    expect(ids((await collectAsync(userGroupsRepo.read())).map((link) => link.group_id))).toEqual(
      ids([engineering!.id, ops!.id]),
    );
    await usersRepo.delete(Object.assign(new UserView(), { id: created?.id }));
    expect(await collectAsync(usersRepo.read())).toEqual([]);
    expect(await collectAsync(userRolesRepo.read())).toEqual([]);
    expect(await collectAsync(userGroupsRepo.read())).toEqual([]);
  });

  it("lazy write skips unloaded links", async () => {
    const connection = await setup();
    const usersRepo = new CRUDRepository(connection, {
      type: UserView,
      write: new LazyWriteEnrichmentStrategy(),
    });
    const userTypesRepo = new CRUDRepository(UserType, connection);
    const rolesRepo = new CRUDRepository(Role, connection);
    const groupsRepo = new CRUDRepository(Group, connection);
    const userRolesRepo = new CRUDRepository(UserRole, connection);
    const userGroupsRepo = new CRUDRepository(UserGroup, connection);
    await userTypesRepo.create(Object.assign(new UserType(), { name: "admin" }));
    await rolesRepo.create(Object.assign(new Role(), { name: "admin" }));
    await groupsRepo.create(Object.assign(new Group(), { name: "engineering" }));
    const adminType = await userTypesRepo.readSingle();
    const adminRole = await rolesRepo.readSingle();
    const engineering = await groupsRepo.readSingle();
    usersRepo.addEnriched({ into: "user_type", from: "user_type_id", by: "id" }, userTypesRepo);
    usersRepo.addLinked(
      { into: "roles", from: "id", by: "id", association: { parent: "user_id", related: "role_id" } },
      rolesRepo,
      userRolesRepo,
    );
    usersRepo.addLinked(
      { into: "groups", from: "id", by: "id", association: { parent: "user_id", related: "group_id" } },
      groupsRepo,
      userGroupsRepo,
    );
    await usersRepo.create(Object.assign(new UserView(), {
      username: "ada",
      is_enabled: true,
      user_type_id: adminType?.id,
      roles: [adminRole],
      groups: [engineering],
    }));
    const created = await usersRepo.readSingle();
    await usersRepo.update(Object.assign(new UserView(), {
      id: created?.id,
      username: "ada",
      is_enabled: false,
      user_type_id: adminType?.id,
    }));
    expect((await collectAsync(userRolesRepo.read())).map((link) => link.role_id)).toEqual([adminRole?.id]);
    expect((await collectAsync(userGroupsRepo.read())).map((link) => link.group_id)).toEqual([engineering?.id]);
  });

  it("rejects a link without an id", async () => {
    const connection = await setup();
    const usersRepo = new CRUDRepository(UserView, connection);
    const rolesRepo = new CRUDRepository(Role, connection);
    const userRolesRepo = new CRUDRepository(UserRole, connection);
    usersRepo.addLinked(
      { into: "roles", from: "id", by: "id", association: { parent: "user_id", related: "role_id" } },
      rolesRepo,
      userRolesRepo,
    );
    await expect(usersRepo.create(Object.assign(new UserView(), {
      username: "ada",
      roles: [new Role()],
    }))).rejects.toThrow();
  });
});
