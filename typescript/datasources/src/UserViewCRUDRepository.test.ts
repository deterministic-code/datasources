import { describe, expect, it } from "vitest";
import { CRUDRepository } from "./CRUDRepository.ts";
import { LazyWriteEnrichmentStrategy } from "./LazyWriteEnrichmentStrategy.ts";
import type { IWriteEnrichmentStrategy } from "./IWriteEnrichmentStrategy.ts";
import { ListCRUDRepository } from "./tests/listCRUDRepository.ts";

class UserView {
  id = 0;
  username = "";
  is_enabled = false;
  user_type_id = 0;
  user_type?: UserType;
  roles: Role[] | null = [];
  groups: Group[] | null = [];
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
  user_id = 0;
  role_id = 0;
}

class UserGroup {
  user_id = 0;
  group_id = 0;
}

const assign = <T extends object>(type: new () => T, values: Partial<T>): T =>
  Object.assign(new type(), values);

const insert = async <T extends object>(repo: ListCRUDRepository<T>, item: T): Promise<T> => {
  await repo.create(item);
  return repo.items.at(-1)!;
};

const graph = async () => {
  const users = new ListCRUDRepository<UserView>();
  const userTypes = new ListCRUDRepository<UserType>();
  const roles = new ListCRUDRepository<Role>();
  const groups = new ListCRUDRepository<Group>();
  const userRoles = new ListCRUDRepository<UserRole>();
  const userGroups = new ListCRUDRepository<UserGroup>();
  return {
    users,
    userTypes,
    roles,
    groups,
    userRoles,
    userGroups,
    adminType: await insert(userTypes, assign(UserType, { name: "admin" })),
    adminRole: await insert(roles, assign(Role, { name: "admin" })),
    editorRole: await insert(roles, assign(Role, { name: "editor" })),
    engineering: await insert(groups, assign(Group, { name: "engineering" })),
    ops: await insert(groups, assign(Group, { name: "ops" })),
  };
};

const repoOf = (
  seeded: Awaited<ReturnType<typeof graph>>,
  write?: IWriteEnrichmentStrategy<UserView>,
) => {
  const repo = new CRUDRepository(seeded.users, undefined, write);
  repo.addEnriched("user_type", "user_type_id", seeded.userTypes, "id");
  repo.addLinked("roles", "id", seeded.roles, "id", seeded.userRoles, "user_id", "role_id");
  repo.addLinked("groups", "id", seeded.groups, "id", seeded.userGroups, "user_id", "group_id");
  return repo;
};

const ids = (values: number[]) => [...values].sort((left, right) => left - right);

describe("CRUDRepository user view", () => {
  it("creates, updates, and deletes through link rows", async () => {
    const seeded = await graph();
    const repo = repoOf(seeded);

    const created = await repo.create(
      assign(UserView, {
        username: "ada",
        is_enabled: true,
        user_type_id: seeded.adminType.id,
        roles: [seeded.adminRole, seeded.editorRole],
        groups: [seeded.engineering],
      }),
    );
    expect(created.id).toBeGreaterThan(0);
    expect(created.username).toBe("ada");
    expect(created?.user_type?.name).toBe("admin");
    expect(created.roles?.map((role) => role.name).sort()).toEqual(["admin", "editor"]);
    expect(ids(created.roles!.map((role) => role.id))).toEqual(ids([seeded.adminRole.id, seeded.editorRole.id]));
    expect(created.roles.every((role) => role.id > 0)).toBe(true);
    expect(created.groups?.map((group) => group.name)).toEqual(["engineering"]);
    expect(created.groups!.map((group) => group.id)).toEqual([seeded.engineering.id]);
    expect(created.groups.every((group) => group.id > 0)).toBe(true);
    expect(seeded.userRoles.items.map((link) => link.user_id)).toEqual([created?.id, created?.id]);
    expect(ids(seeded.userRoles.items.map((link) => link.role_id))).toEqual(
      ids([seeded.adminRole.id, seeded.editorRole.id]),
    );
    expect(seeded.userGroups.items[0]).toMatchObject({
      user_id: created?.id,
      group_id: seeded.engineering.id,
    });
    expect(seeded.roles.items.map((role) => role.name)).toEqual(["admin", "editor"]);
    expect(seeded.userTypes.items.map((type) => type.name)).toEqual(["admin"]);

    await repo.update(
      assign(UserView, {
        id: created?.id,
        username: "ada",
        is_enabled: true,
        user_type_id: seeded.adminType.id,
        roles: [seeded.editorRole],
        groups: [seeded.engineering, seeded.ops],
      }),
    );
    const updated = await repo.readSingle();
    expect(updated?.roles?.map((role) => role.name)).toEqual(["editor"]);
    expect(updated?.roles?.map((role) => role.id)).toEqual([seeded.editorRole.id]);
    expect(updated?.roles?.every((role) => role.id > 0)).toBe(true);
    expect(updated?.groups?.map((group) => group.name).sort()).toEqual(["engineering", "ops"]);
    expect(ids(updated?.groups?.map((group) => group.id) ?? [])).toEqual(ids([seeded.engineering.id, seeded.ops.id]));
    expect(updated?.groups?.every((group) => group.id > 0)).toBe(true);
    expect(seeded.userRoles.items.map((link) => link.role_id)).toEqual([seeded.editorRole.id]);
    expect(ids(seeded.userGroups.items.map((link) => link.group_id))).toEqual(
      ids([seeded.engineering.id, seeded.ops.id]),
    );
    expect(seeded.roles.items.map((role) => role.name).sort()).toEqual(["admin", "editor"]);
    expect(seeded.groups.items.map((group) => group.name).sort()).toEqual(["engineering", "ops"]);

    await repo.delete(assign(UserView, { id: created?.id }));
    expect(seeded.users.items).toEqual([]);
    expect(seeded.userRoles.items).toEqual([]);
    expect(seeded.userGroups.items).toEqual([]);
    expect(seeded.roles.items.map((role) => role.name).sort()).toEqual(["admin", "editor"]);
    expect(seeded.groups.items.map((group) => group.name).sort()).toEqual(["engineering", "ops"]);
    expect(seeded.userTypes.items.map((type) => type.name)).toEqual(["admin"]);
  });

  it("skips unloaded collections on lazy create and update and cascades delete", async () => {
    const seeded = await graph();
    const repo = repoOf(seeded, new LazyWriteEnrichmentStrategy());

    const created = await repo.create(
      assign(UserView, {
        username: "ada",
        is_enabled: true,
        user_type_id: seeded.adminType.id,
        roles: null,
        groups: [seeded.engineering],
      }),
    );
    expect(seeded.userRoles.items).toEqual([]);
    expect(created.groups?.map((group) => group.id)).toEqual([seeded.engineering.id]);
    expect(created.groups?.every((group) => group.id > 0)).toBe(true);
    expect(seeded.userGroups.items[0]).toMatchObject({
      user_id: created?.id,
      group_id: seeded.engineering.id,
    });

    await repo.update(
      assign(UserView, {
        id: created?.id,
        username: "ada",
        is_enabled: true,
        user_type_id: seeded.adminType.id,
        roles: null,
        groups: null,
      }),
    );
    expect(seeded.userRoles.items).toEqual([]);
    expect(seeded.userGroups.items[0]).toMatchObject({
      user_id: created?.id,
      group_id: seeded.engineering.id,
    });

    await repo.update(
      assign(UserView, {
        id: created?.id,
        username: "ada",
        is_enabled: true,
        user_type_id: seeded.adminType.id,
        roles: [seeded.editorRole],
        groups: null,
      }),
    );
    expect(seeded.userRoles.items.map((link) => link.role_id)).toEqual([seeded.editorRole.id]);
    expect(seeded.userGroups.items[0]).toMatchObject({
      user_id: created?.id,
      group_id: seeded.engineering.id,
    });
    expect(seeded.roles.items.map((role) => role.name).sort()).toEqual(["admin", "editor"]);
    expect(seeded.groups.items.map((group) => group.name).sort()).toEqual(["engineering", "ops"]);

    await repo.delete(assign(UserView, { id: created?.id, roles: null, groups: null }));
    expect(seeded.users.items).toEqual([]);
    expect(seeded.userRoles.items).toEqual([]);
    expect(seeded.userGroups.items).toEqual([]);
    expect(seeded.roles.items.map((role) => role.name).sort()).toEqual(["admin", "editor"]);
    expect(seeded.groups.items.map((group) => group.name).sort()).toEqual(["engineering", "ops"]);
    expect(seeded.userTypes.items.map((type) => type.name)).toEqual(["admin"]);
  });
});
