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

type Graph = {
  usersRepo: CRUDRepository<UserView>;
  userTypesRepo: CRUDRepository<UserType>;
  rolesRepo: CRUDRepository<Role>;
  groupsRepo: CRUDRepository<Group>;
  userRolesRepo: CRUDRepository<UserRole>;
  userGroupsRepo: CRUDRepository<UserGroup>;
  adminType: UserType;
  adminRole: Role;
  editorRole: Role;
  engineering: Group;
  ops: Group;
};

const setup = async (): Promise<IConnection> => {
  const connection = new MemoryProvider().connection();
  await connection.open();
  return connection;
};

const ids = (values: number[]) => [...values].sort((left, right) => left - right);

const graph = async (write?: LazyWriteEnrichmentStrategy<UserView>): Promise<Graph> => {
  const connection = await setup();
  const usersRepo = new CRUDRepository(connection, { type: UserView, write });
  const userTypesRepo = new CRUDRepository(UserType, connection);
  const rolesRepo = new CRUDRepository(Role, connection);
  const groupsRepo = new CRUDRepository(Group, connection);
  const userRolesRepo = new CRUDRepository(UserRole, connection);
  const userGroupsRepo = new CRUDRepository(UserGroup, connection);
  const adminType = await userTypesRepo.create(Object.assign(new UserType(), { name: "admin" }));
  const adminRole = await rolesRepo.create(Object.assign(new Role(), { name: "admin" }));
  const editorRole = await rolesRepo.create(Object.assign(new Role(), { name: "editor" }));
  const engineering = await groupsRepo.create(Object.assign(new Group(), { name: "engineering" }));
  const ops = await groupsRepo.create(Object.assign(new Group(), { name: "ops" }));
  return { usersRepo, userTypesRepo, rolesRepo, groupsRepo, userRolesRepo, userGroupsRepo, adminType, adminRole, editorRole, engineering, ops };
};

const link = (usersRepo: CRUDRepository<UserView>, seeded: Graph): void => {
  usersRepo.addEnriched({ into: "user_type", from: "user_type_id", by: "id" }, seeded.userTypesRepo);
  usersRepo.addLinked(
    { into: "roles", from: "id", by: "id", association: { parent: "user_id", related: "role_id" } },
    seeded.rolesRepo,
    seeded.userRolesRepo,
  );
  usersRepo.addLinked(
    { into: "groups", from: "id", by: "id", association: { parent: "user_id", related: "group_id" } },
    seeded.groupsRepo,
    seeded.userGroupsRepo,
  );
};

const createAda = async (seeded: Graph, roles: Role[] | null, groups: Group[] | null): Promise<UserView> => {
  link(seeded.usersRepo, seeded);
  return seeded.usersRepo.create(Object.assign(new UserView(), {
    username: "ada",
    is_enabled: true,
    user_type_id: seeded.adminType.id,
    roles,
    groups,
  }));
};

describe("CRUDRepository user view", () => {
  it("creates, updates, and deletes through link rows", async () => {
    const seeded = await graph();
    const usersRepo = seeded.usersRepo;
    usersRepo.addEnriched({ into: "user_type", from: "user_type_id", by: "id" }, seeded.userTypesRepo);
    usersRepo.addLinked(
      { into: "roles", from: "id", by: "id", association: { parent: "user_id", related: "role_id" } },
      seeded.rolesRepo,
      seeded.userRolesRepo,
    );
    usersRepo.addLinked(
      { into: "groups", from: "id", by: "id", association: { parent: "user_id", related: "group_id" } },
      seeded.groupsRepo,
      seeded.userGroupsRepo,
    );
    const created = await usersRepo.create(Object.assign(new UserView(), {
      username: "ada",
      is_enabled: true,
      user_type_id: seeded.adminType.id,
      roles: [seeded.adminRole, seeded.editorRole],
      groups: [seeded.engineering],
    }));
    const createdRoleLinks = await collectAsync(seeded.userRolesRepo.read());
    const createdGroupLinks = await collectAsync(seeded.userGroupsRepo.read());
    const saved = await usersRepo.update(Object.assign(new UserView(), {
      id: created.id,
      username: "ada",
      is_enabled: true,
      user_type_id: seeded.adminType.id,
      roles: [seeded.editorRole],
      groups: [seeded.engineering, seeded.ops],
    }));
    const updated = await usersRepo.readSingleById(created.id);
    const updatedRoleLinks = await collectAsync(seeded.userRolesRepo.read());
    const updatedGroupLinks = await collectAsync(seeded.userGroupsRepo.read());
    const removed = await usersRepo.delete(Object.assign(new UserView(), { id: created.id }));

    expect(created.id > 0).toEqual(true);
    expect(created.username).toEqual("ada");
    expect(created.is_enabled).toEqual(true);
    expect(created.user_type_id).toEqual(seeded.adminType.id);
    expect(created.user_type?.name).toEqual("admin");
    expect(ids(created.roles?.map((role) => role.id) ?? [])).toEqual(ids([seeded.adminRole.id, seeded.editorRole.id]));
    expect(created.groups?.map((group) => group.name)).toEqual(["engineering"]);
    expect(createdRoleLinks.map((linkRow) => linkRow.user_id)).toEqual([created.id, created.id]);
    expect(ids(createdRoleLinks.map((linkRow) => linkRow.role_id))).toEqual(ids([seeded.adminRole.id, seeded.editorRole.id]));
    expect(createdGroupLinks.map((linkRow) => linkRow.user_id)).toEqual([created.id]);
    expect(createdGroupLinks.map((linkRow) => linkRow.group_id)).toEqual([seeded.engineering.id]);
    expect(saved.rowsAffected).toEqual(1);
    expect(updated?.id).toEqual(created.id);
    expect(updated?.username).toEqual("ada");
    expect(updated?.is_enabled).toEqual(true);
    expect(updated?.user_type_id).toEqual(seeded.adminType.id);
    expect(updated?.roles?.map((role) => role.id)).toEqual([seeded.editorRole.id]);
    expect(ids(updated?.groups?.map((group) => group.id) ?? [])).toEqual(ids([seeded.engineering.id, seeded.ops.id]));
    expect(updatedRoleLinks.map((linkRow) => linkRow.user_id)).toEqual([created.id]);
    expect(updatedRoleLinks.map((linkRow) => linkRow.role_id)).toEqual([seeded.editorRole.id]);
    expect(updatedGroupLinks.map((linkRow) => linkRow.user_id)).toEqual([created.id, created.id]);
    expect(ids(updatedGroupLinks.map((linkRow) => linkRow.group_id))).toEqual(ids([seeded.engineering.id, seeded.ops.id]));
    expect(removed.rowsAffected).toEqual(1);
    expect(await collectAsync(usersRepo.read())).toEqual([]);
    expect(await collectAsync(seeded.userRolesRepo.read())).toEqual([]);
    expect(await collectAsync(seeded.userGroupsRepo.read())).toEqual([]);
    expect((await collectAsync(seeded.rolesRepo.read())).map((role) => role.name).sort()).toEqual(["admin", "editor"]);
    expect((await collectAsync(seeded.groupsRepo.read())).map((group) => group.name).sort()).toEqual(["engineering", "ops"]);
    expect((await collectAsync(seeded.userTypesRepo.read())).map((type) => type.name)).toEqual(["admin"]);
  });

  it("clears one association when the list is null", async () => {
    const seeded = await graph();
    const created = await createAda(seeded, [seeded.adminRole, seeded.editorRole], [seeded.engineering]);
    const saved = await seeded.usersRepo.update(Object.assign(new UserView(), {
      id: created.id,
      username: "ada",
      is_enabled: true,
      user_type_id: seeded.adminType.id,
      roles: null,
      groups: [seeded.engineering],
    }));
    const loaded = await seeded.usersRepo.readSingleById(created.id);
    const userRoles = await collectAsync(seeded.userRolesRepo.read());
    const userGroups = await collectAsync(seeded.userGroupsRepo.read());

    expect(created.id > 0).toEqual(true);
    expect(saved.rowsAffected).toEqual(1);
    expect(loaded?.id).toEqual(created.id);
    expect(loaded?.username).toEqual("ada");
    expect(loaded?.user_type_id).toEqual(seeded.adminType.id);
    expect(loaded?.user_type?.name).toEqual("admin");
    expect(loaded?.roles ?? []).toEqual([]);
    expect(loaded?.groups?.map((group) => group.id)).toEqual([seeded.engineering.id]);
    expect(userRoles).toEqual([]);
    expect(userGroups.map((linkRow) => linkRow.user_id)).toEqual([created.id]);
    expect(userGroups.map((linkRow) => linkRow.group_id)).toEqual([seeded.engineering.id]);
    expect((await collectAsync(seeded.rolesRepo.read())).map((role) => role.name).sort()).toEqual(["admin", "editor"]);
    expect((await collectAsync(seeded.groupsRepo.read())).map((group) => group.name).sort()).toEqual(["engineering", "ops"]);
  });

  it("clears one association when the list is empty", async () => {
    const seeded = await graph();
    const created = await createAda(seeded, [seeded.adminRole, seeded.editorRole], [seeded.engineering]);
    const saved = await seeded.usersRepo.update(Object.assign(new UserView(), {
      id: created.id,
      username: "ada",
      is_enabled: true,
      user_type_id: seeded.adminType.id,
      roles: [],
      groups: [seeded.engineering],
    }));
    const loaded = await seeded.usersRepo.readSingleById(created.id);
    const userRoles = await collectAsync(seeded.userRolesRepo.read());
    const userGroups = await collectAsync(seeded.userGroupsRepo.read());

    expect(created.id > 0).toEqual(true);
    expect(saved.rowsAffected).toEqual(1);
    expect(loaded?.id).toEqual(created.id);
    expect(loaded?.username).toEqual("ada");
    expect(loaded?.user_type_id).toEqual(seeded.adminType.id);
    expect(loaded?.user_type?.name).toEqual("admin");
    expect(loaded?.roles ?? []).toEqual([]);
    expect(loaded?.groups?.map((group) => group.id)).toEqual([seeded.engineering.id]);
    expect(userRoles).toEqual([]);
    expect(userGroups.map((linkRow) => linkRow.user_id)).toEqual([created.id]);
    expect(userGroups.map((linkRow) => linkRow.group_id)).toEqual([seeded.engineering.id]);
    expect((await collectAsync(seeded.rolesRepo.read())).map((role) => role.name).sort()).toEqual(["admin", "editor"]);
    expect((await collectAsync(seeded.groupsRepo.read())).map((group) => group.name).sort()).toEqual(["engineering", "ops"]);
  });

  it("inserts one junction row for a duplicate related id", async () => {
    const seeded = await graph();
    const created = await createAda(seeded, [seeded.adminRole, seeded.adminRole], [seeded.engineering]);
    const roleLinks = await collectAsync(seeded.userRolesRepo.read());
    const groupLinks = await collectAsync(seeded.userGroupsRepo.read());

    expect(created.id > 0).toEqual(true);
    expect(roleLinks.map((linkRow) => linkRow.user_id)).toEqual([created.id]);
    expect(roleLinks.map((linkRow) => linkRow.role_id)).toEqual([seeded.adminRole.id]);
    expect(groupLinks.map((linkRow) => linkRow.user_id)).toEqual([created.id]);
    expect(groupLinks.map((linkRow) => linkRow.group_id)).toEqual([seeded.engineering.id]);
    expect((await collectAsync(seeded.rolesRepo.read())).map((role) => role.name).sort()).toEqual(["admin", "editor"]);
  });

  it("lazy write skips unloaded links", async () => {
    const seeded = await graph(new LazyWriteEnrichmentStrategy());
    link(seeded.usersRepo, seeded);
    const created = await seeded.usersRepo.create(Object.assign(new UserView(), {
      username: "ada",
      is_enabled: true,
      user_type_id: seeded.adminType.id,
      roles: null,
      groups: [seeded.engineering],
    }));
    const createdRoleLinks = await collectAsync(seeded.userRolesRepo.read());
    const createdGroupLinks = await collectAsync(seeded.userGroupsRepo.read());
    const skipped = await seeded.usersRepo.update(Object.assign(new UserView(), {
      id: created.id,
      username: "ada",
      is_enabled: false,
      user_type_id: seeded.adminType.id,
    }));
    const afterSkip = await seeded.usersRepo.readSingleById(created.id);
    const skippedRoleLinks = await collectAsync(seeded.userRolesRepo.read());
    const skippedGroupLinks = await collectAsync(seeded.userGroupsRepo.read());
    const linked = await seeded.usersRepo.update(Object.assign(new UserView(), {
      id: created.id,
      username: "ada",
      is_enabled: false,
      user_type_id: seeded.adminType.id,
      roles: [seeded.editorRole],
    }));
    const linkedRoleLinks = await collectAsync(seeded.userRolesRepo.read());
    const linkedGroupLinks = await collectAsync(seeded.userGroupsRepo.read());
    const removed = await seeded.usersRepo.delete(Object.assign(new UserView(), { id: created.id, roles: null, groups: null }));

    expect(created.id > 0).toEqual(true);
    expect(createdRoleLinks).toEqual([]);
    expect(createdGroupLinks.map((linkRow) => linkRow.user_id)).toEqual([created.id]);
    expect(createdGroupLinks.map((linkRow) => linkRow.group_id)).toEqual([seeded.engineering.id]);
    expect(skipped.rowsAffected).toEqual(1);
    expect(afterSkip?.id).toEqual(created.id);
    expect(afterSkip?.username).toEqual("ada");
    expect(afterSkip?.is_enabled).toEqual(false);
    expect(skippedRoleLinks).toEqual([]);
    expect(skippedGroupLinks.map((linkRow) => linkRow.user_id)).toEqual([created.id]);
    expect(skippedGroupLinks.map((linkRow) => linkRow.group_id)).toEqual([seeded.engineering.id]);
    expect(linked.rowsAffected).toEqual(1);
    expect(linkedRoleLinks.map((linkRow) => linkRow.user_id)).toEqual([created.id]);
    expect(linkedRoleLinks.map((linkRow) => linkRow.role_id)).toEqual([seeded.editorRole.id]);
    expect(linkedGroupLinks.map((linkRow) => linkRow.user_id)).toEqual([created.id]);
    expect(linkedGroupLinks.map((linkRow) => linkRow.group_id)).toEqual([seeded.engineering.id]);
    expect((await collectAsync(seeded.rolesRepo.read())).map((role) => role.name).sort()).toEqual(["admin", "editor"]);
    expect((await collectAsync(seeded.groupsRepo.read())).map((group) => group.name).sort()).toEqual(["engineering", "ops"]);
    expect(removed.rowsAffected).toEqual(1);
    expect(await collectAsync(seeded.usersRepo.read())).toEqual([]);
    expect(await collectAsync(seeded.userRolesRepo.read())).toEqual([]);
    expect(await collectAsync(seeded.userGroupsRepo.read())).toEqual([]);
  });

  it("rejects a link without an id", async () => {
    const seeded = await graph();
    seeded.usersRepo.addLinked(
      { into: "roles", from: "id", by: "id", association: { parent: "user_id", related: "role_id" } },
      seeded.rolesRepo,
      seeded.userRolesRepo,
    );
    await expect(seeded.usersRepo.create(Object.assign(new UserView(), {
      username: "ada",
      user_type_id: seeded.adminType.id,
      is_enabled: true,
      roles: [new Role()],
    }))).rejects.toThrow(/requires "id"/);
    const users = await collectAsync(seeded.usersRepo.read());
    expect(users.length).toEqual(1);
    expect(users[0]!.id > 0).toEqual(true);
    expect(users[0]!.username).toEqual("ada");
    expect(await collectAsync(seeded.userRolesRepo.read())).toEqual([]);
  });
});
