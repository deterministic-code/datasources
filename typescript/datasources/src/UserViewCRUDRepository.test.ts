import { describe, expect, it } from "vitest";
import { LazyWriteEnrichmentStrategy } from "./LazyWriteEnrichmentStrategy.ts";
import type { IWriteEnrichmentStrategy } from "./IWriteEnrichmentStrategy.ts";
import { ListCRUDRepository } from "./tests/listCRUDRepository.ts";

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

const graph = async (write?: IWriteEnrichmentStrategy<UserView>) => {
  const users = new ListCRUDRepository(write, [], UserView);
  const userTypes = new ListCRUDRepository<UserType>([], UserType);
  const roles = new ListCRUDRepository<Role>([], Role);
  const groups = new ListCRUDRepository<Group>([], Group);
  const userRoles = new ListCRUDRepository<UserRole>([], UserRole);
  const userGroups = new ListCRUDRepository<UserGroup>([], UserGroup);
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

const repoOf = (seeded: Awaited<ReturnType<typeof graph>>) => {
  seeded.users.addEnriched("user_type", "user_type_id", seeded.userTypes, "id");
  seeded.users.addLinked(
    "roles",
    "id",
    seeded.roles,
    "id",
    seeded.userRoles,
    "user_id",
    "role_id",
  );
  seeded.users.addLinked(
    "groups",
    "id",
    seeded.groups,
    "id",
    seeded.userGroups,
    "user_id",
    "group_id",
  );
  return seeded.users;
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
    expect(created?.id).toBe(1);
    expect(seeded.userRoles.items.map((link) => link.user_id)).toEqual([created?.id, created?.id]);
    expect(ids(seeded.userRoles.items.map((link) => link.role_id))).toEqual(
      ids([seeded.adminRole.id, seeded.editorRole.id]),
    );
    expect(seeded.userGroups.items.map((link) => link.group_id)).toEqual([
      seeded.engineering.id,
    ]);

    const loaded = await repo.readSingleById(created!.id);
    expect(loaded?.user_type?.name).toBe("admin");
    expect(ids(loaded?.roles?.map((role) => role.id) ?? [])).toEqual(
      ids([seeded.adminRole.id, seeded.editorRole.id]),
    );
    expect(loaded?.groups?.map((group) => group.name)).toEqual(["engineering"]);

    await repo.update(
      assign(UserView, {
        id: created!.id,
        username: "ada",
        is_enabled: true,
        user_type_id: seeded.adminType.id,
        roles: [seeded.editorRole],
        groups: [seeded.engineering, seeded.ops],
      }),
    );
    expect(seeded.userRoles.items.map((link) => link.role_id)).toEqual([seeded.editorRole.id]);
    expect(ids(seeded.userGroups.items.map((link) => link.group_id))).toEqual(
      ids([seeded.engineering.id, seeded.ops.id]),
    );

    await repo.delete(assign(UserView, { id: created!.id }));
    expect(seeded.users.items).toHaveLength(0);
    expect(seeded.userRoles.items).toHaveLength(0);
    expect(seeded.userGroups.items).toHaveLength(0);
  });

  it("lazy write skips unloaded links", async () => {
    const seeded = await graph(new LazyWriteEnrichmentStrategy());
    const repo = repoOf(seeded);
    const created = await repo.create(
      assign(UserView, {
        username: "ada",
        is_enabled: true,
        user_type_id: seeded.adminType.id,
        roles: [seeded.adminRole],
        groups: [seeded.engineering],
      }),
    );
    await repo.update(
      assign(UserView, {
        id: created!.id,
        username: "ada",
        is_enabled: false,
        user_type_id: seeded.adminType.id,
      }),
    );
    expect(seeded.userRoles.items.map((link) => link.role_id)).toEqual([seeded.adminRole.id]);
    expect(seeded.userGroups.items.map((link) => link.group_id)).toEqual([
      seeded.engineering.id,
    ]);
  });

  it("rejects a link without an id and skips a missing key", async () => {
    const seeded = await graph();
    const repo = repoOf(seeded);
    await expect(
      repo.create(
        assign(UserView, {
          username: "ada",
          roles: [assign(Role, {})],
        }),
      ),
    ).rejects.toThrow();
    await repo.create(
      assign(UserView, {
        username: "ada",
        roles: [],
        groups: [],
      }),
    );
  });
});
