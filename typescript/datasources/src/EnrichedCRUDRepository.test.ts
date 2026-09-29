import { describe, expect, it } from "vitest";
import { MemoryDriver } from "@datasources/memory";
import { collectAsync } from "./collectAsync.ts";
import { CRUDRepository } from "./CRUDRepository.ts";
import { CascadeDeleteUpdateEnrichmentStrategy } from "./CascadeDeleteUpdateEnrichmentStrategy.ts";
import { DefaultWriteEnrichmentStrategy } from "./DefaultWriteEnrichmentStrategy.ts";
import { ReadonlyRepository } from "./ReadonlyRepository.ts";
import type { IConnection } from "./IConnection.ts";
import type { IReadonlyRepository } from "./IReadonlyRepository.ts";
import type { Scalar } from "./builders/Scalar.ts";
import { DeleteEnrichmentStrategy } from "./DeleteEnrichmentStrategy.ts";

class Parent {
  id = 0;
  type_id = 0;
  type_name = "";
  children: Child[] = [];
}

class TypeRow {
  id = 0;
  name = "";
}

class Child {
  id = 0;
  parent_id = 0;
  type_id = 0;
  label = "";
  type_name = "";
  notes: Note[] = [];
}

class Note {
  id = 0;
  child_id = 0;
  text = "";
}

class Tagged {
  id = 0;
  label = "";
  tags: Tag[] = [];
}

class Tag {
  id = 0;
  name = "";
}

class ParentTag {
  id = 0;
  parent_id = 0;
  tag_id = 0;
}

class SkipDelete<T extends object> extends DeleteEnrichmentStrategy<T> {
  deleteOwned(): Promise<void> {
    return Promise.resolve();
  }

  deleteLinked(): Promise<void> {
    return Promise.resolve();
  }
}

type Seed = Record<string, Scalar>;

const setup = async (rows: Seed[] = []): Promise<IConnection> => {
  const connection = new MemoryDriver().connection();
  await connection.open();
  for (const row of rows) {
    const response = await connection.createCommand().execute(
      "insert",
      Object.entries(row).map(([name, value]) => ({ name, value })),
    );
    if (typeof response.result === "number") row.id = response.result;
  }
  return connection;
};

const relate = (
  parentsRepo: CRUDRepository<Parent>,
  typesRepo: IReadonlyRepository<TypeRow>,
  childrenRepo: IReadonlyRepository<Child>,
) => {
  parentsRepo.addEnriched({ into: "type_name", from: "type_id", by: "id", select: "name" }, typesRepo);
  parentsRepo.addEnriched({ into: "children", from: "id", by: "parent_id", many: true }, childrenRepo);
};

describe("CRUDRepository", () => {
  it("creates children, stamps FK, and skips select", async () => {
    const connection = await setup();
    const parentsRepo = new CRUDRepository(Parent, connection);
    const typesRepo = new CRUDRepository(TypeRow, connection);
    const childrenRepo = new CRUDRepository(Child, connection);
    const type = await typesRepo.create(Object.assign(new TypeRow(), { name: "person" }));
    expect(type.id).toBeGreaterThan(0);
    relate(parentsRepo, typesRepo, childrenRepo);
    const created = await parentsRepo.create(Object.assign(new Parent(), {
      type_id: type.id,
      children: [
        Object.assign(new Child(), { label: "a" }),
        Object.assign(new Child(), { label: "b" }),
      ],
    }));
    expect(created.id).toBeGreaterThan(0);
    const parent = (await collectAsync(parentsRepo.read()))[0];
    const children = await collectAsync(childrenRepo.read());
    expect(parent?.id).toBe(created.id);
    expect(children).toHaveLength(2);
    expect(children.every((child) => child.parent_id === parent?.id)).toBe(true);
    expect(children.map((child) => child.label)).toEqual(["a", "b"]);
    expect((await typesRepo.readSingle())?.name).toBe("person");
  });

  it("applies generated parent id before child FKs", async () => {
    const connection = await setup();
    const parentsRepo = new CRUDRepository(Parent, connection);
    const childrenRepo = new CRUDRepository(Child, connection);
    parentsRepo.addEnriched({ into: "children", from: "id", by: "parent_id", many: true }, childrenRepo);
    const created = await parentsRepo.create(Object.assign(new Parent(), {
      children: [Object.assign(new Child(), { label: "n" })],
    }));
    expect(created.id).toBeGreaterThan(0);
    const parent = await parentsRepo.readSingle();
    const child = await childrenRepo.readSingle();
    expect(parent?.id).toBe(created.id);
    expect(child?.parent_id).toBe(parent?.id);
  });

  it("update replace-syncs children", async () => {
    const connection = await setup();
    const parentsRepo = new CRUDRepository(Parent, connection);
    const childrenRepo = new CRUDRepository(Child, connection);
    const parent = await parentsRepo.create(new Parent());
    expect(parent.id).toBeGreaterThan(0);
    const kept = await childrenRepo.create(Object.assign(new Child(), { parent_id: parent.id, label: "a" }));
    expect(kept.id).toBeGreaterThan(0);
    const dropped = await childrenRepo.create(Object.assign(new Child(), { parent_id: parent.id, label: "b" }));
    expect(dropped.id).toBeGreaterThan(0);
    parentsRepo.addEnriched({ into: "children", from: "id", by: "parent_id", many: true }, childrenRepo);
    const updated = await parentsRepo.update(Object.assign(new Parent(), {
      id: parent.id,
      children: [
        Object.assign(new Child(), { id: kept.id, label: "aa" }),
        Object.assign(new Child(), { label: "c" }),
      ],
    }));
    expect(updated.rowsAffected).toBe(1);
    const children = await collectAsync(childrenRepo.read());
    expect(children).toHaveLength(2);
    expect(children.find((child) => child.id === kept?.id)?.label).toBe("aa");
    expect(children.find((child) => child.id !== kept?.id)?.label).toBe("c");
    expect(children.every((child) => child.parent_id === parent?.id)).toBe(true);
  });

  it("cascade update deletes children and recreates", async () => {
    const connection = await setup();
    const parentsRepo = new CRUDRepository<Parent>(connection, {
      type: Parent,
      write: new DefaultWriteEnrichmentStrategy(undefined, new CascadeDeleteUpdateEnrichmentStrategy()),
    });
    const childrenRepo = new CRUDRepository(Child, connection);
    const notesRepo = new CRUDRepository(Note, connection);
    const parent = await parentsRepo.create(new Parent());
    expect(parent.id).toBeGreaterThan(0);
    const child = await childrenRepo.create(Object.assign(new Child(), { parent_id: parent.id, label: "a" }));
    expect(child.id).toBeGreaterThan(0);
    const other = await childrenRepo.create(Object.assign(new Child(), { label: "other" }));
    expect(other.id).toBeGreaterThan(0);
    const note = await notesRepo.create(Object.assign(new Note(), { child_id: child.id, text: "n" }));
    expect(note.id).toBeGreaterThan(0);
    childrenRepo.addEnriched({ into: "notes", from: "id", by: "child_id", many: true }, notesRepo);
    parentsRepo.addEnriched({ into: "children", from: "id", by: "parent_id", many: true }, childrenRepo);
    const updated = await parentsRepo.update(Object.assign(new Parent(), {
      id: parent.id,
      children: [Object.assign(new Child(), { id: child.id, label: "fresh" })],
    }));
    expect(updated.rowsAffected).toBe(1);
    const created = (await collectAsync(childrenRepo.read())).find((row) => row.label === "fresh");
    expect(created?.id).not.toBe(child?.id);
    expect(created?.label).toBe("fresh");
    expect(created?.parent_id).toBe(parent?.id);
    expect(await collectAsync(notesRepo.read())).toEqual([]);
  });

  it("cascade update replaces links and keeps related rows", async () => {
    const connection = await setup();
    const parentsRepo = new CRUDRepository<Tagged>(connection, {
      type: Tagged,
      write: new DefaultWriteEnrichmentStrategy(undefined, new CascadeDeleteUpdateEnrichmentStrategy()),
    });
    const tagsRepo = new CRUDRepository(Tag, connection);
    const linksRepo = new CRUDRepository(ParentTag, connection);
    const parent = await parentsRepo.create(new Tagged());
    expect(parent.id).toBeGreaterThan(0);
    const keep = await tagsRepo.create(Object.assign(new Tag(), { name: "keep" }));
    expect(keep.id).toBeGreaterThan(0);
    const drop = await tagsRepo.create(Object.assign(new Tag(), { name: "drop" }));
    expect(drop.id).toBeGreaterThan(0);
    const oldLink = await linksRepo.create(Object.assign(new ParentTag(), { parent_id: parent.id, tag_id: keep.id }));
    expect(oldLink.id).toBeGreaterThan(0);
    const droppedLink = await linksRepo.create(Object.assign(new ParentTag(), { parent_id: parent.id, tag_id: drop.id }));
    expect(droppedLink.id).toBeGreaterThan(0);
    const leftover = await linksRepo.create(Object.assign(new ParentTag(), { tag_id: keep.id }));
    expect(leftover.id).toBeGreaterThan(0);
    parentsRepo.addLinked(
      { into: "tags", from: "id", by: "id", association: { parent: "parent_id", related: "tag_id" } },
      tagsRepo,
      linksRepo,
    );
    const updated = await parentsRepo.update(Object.assign(new Tagged(), {
      id: parent.id,
      label: "p",
      tags: [Object.assign(new Tag(), { id: keep.id })],
    }));
    expect(updated.rowsAffected).toBe(1);
    const links = (await collectAsync(linksRepo.read())).filter((link) => link.parent_id === parent.id);
    expect(links).toHaveLength(1);
    expect(links[0]?.tag_id).toBe(keep.id);
    expect(links[0]?.parent_id).toBe(parent.id);
    expect(links[0]?.id).not.toBe(oldLink?.id);
    expect(await collectAsync(tagsRepo.read())).toHaveLength(2);
  });

  it("delete cascades children then parent", async () => {
    const connection = await setup();
    const parentsRepo = new CRUDRepository(Parent, connection);
    const childrenRepo = new CRUDRepository(Child, connection);
    const notesRepo = new CRUDRepository(Note, connection);
    const parent = await parentsRepo.create(new Parent());
    expect(parent.id).toBeGreaterThan(0);
    const child = await childrenRepo.create(Object.assign(new Child(), { parent_id: parent.id, label: "a" }));
    expect(child.id).toBeGreaterThan(0);
    const note = await notesRepo.create(Object.assign(new Note(), { child_id: child.id, text: "n" }));
    expect(note.id).toBeGreaterThan(0);
    const other = await notesRepo.create(Object.assign(new Note(), { text: "other" }));
    expect(other.id).toBeGreaterThan(0);
    childrenRepo.addEnriched({ into: "notes", from: "id", by: "child_id", many: true }, notesRepo);
    parentsRepo.addEnriched({ into: "children", from: "id", by: "parent_id", many: true }, childrenRepo);
    const removed = await parentsRepo.delete(Object.assign(new Parent(), { id: parent.id }));
    expect(removed.rowsAffected).toBe(1);
    expect(await collectAsync(parentsRepo.read())).toEqual([]);
    expect(await collectAsync(childrenRepo.read())).toEqual([]);
    expect((await notesRepo.readSingle())?.text).toBe("other");
  });

  it("delete strategy can skip children", async () => {
    const connection = await setup();
    const childrenRepo = new CRUDRepository(Child, connection);
    const parentsRepo = new CRUDRepository(connection, {
      type: Parent,
      write: new DefaultWriteEnrichmentStrategy(new SkipDelete()),
    });
    const parent = await parentsRepo.create(new Parent());
    expect(parent.id).toBeGreaterThan(0);
    const child = await childrenRepo.create(Object.assign(new Child(), { parent_id: parent.id, label: "a" }));
    expect(child.id).toBeGreaterThan(0);
    parentsRepo.addEnriched({ into: "children", from: "id", by: "parent_id", many: true }, childrenRepo);
    const removed = await parentsRepo.delete(Object.assign(new Parent(), { id: parent.id }));
    expect(removed.rowsAffected).toBe(1);
    expect(await collectAsync(parentsRepo.read())).toEqual([]);
    expect((await childrenRepo.readSingle())?.label).toBe("a");
  });

  it("writes nested many and skips nested select", async () => {
    const connection = await setup();
    const parentsRepo = new CRUDRepository(Parent, connection);
    const childrenRepo = new CRUDRepository(Child, connection);
    const notesRepo = new CRUDRepository(Note, connection);
    const childTypesRepo = new CRUDRepository(TypeRow, connection);
    const home = await childTypesRepo.create(Object.assign(new TypeRow(), { name: "home" }));
    expect(home.id).toBeGreaterThan(0);
    childrenRepo.addEnriched({ into: "type_name", from: "type_id", by: "id", select: "name" }, childTypesRepo);
    childrenRepo.addEnriched({ into: "notes", from: "id", by: "child_id", many: true }, notesRepo);
    parentsRepo.addEnriched({ into: "children", from: "id", by: "parent_id", many: true }, childrenRepo);
    const created = await parentsRepo.create(Object.assign(new Parent(), {
      children: [Object.assign(new Child(), {
        type_id: home.id,
        label: "a",
        notes: [Object.assign(new Note(), { text: "hi" })],
      })],
    }));
    expect(created.id).toBeGreaterThan(0);
    const note = await notesRepo.readSingle();
    const child = await childrenRepo.readSingle();
    expect(note?.text).toBe("hi");
    expect(note?.child_id).toBe(child?.id);
    expect((await childTypesRepo.readSingle())?.name).toBe("home");
  });

  it("throws when many via is readonly", async () => {
    const connection = await setup();
    const parentsRepo = new CRUDRepository(Parent, connection);
    parentsRepo.addEnriched(
      { into: "children", from: "id", by: "parent_id", many: true },
      new ReadonlyRepository(Child, connection),
    );
    await expect(parentsRepo.create(Object.assign(new Parent(), {
      children: [Object.assign(new Child(), { label: "a" })],
    }))).rejects.toThrow(/children/);
  });

  it("get still enriches", async () => {
    const org: Seed = { name: "org" };
    const typesConnection = await setup([org]);
    const parent: Seed = { type_id: org.id };
    const connection = await setup([parent]);
    const parentsRepo = new CRUDRepository(Parent, connection);
    const childrenConnection = await setup([{ parent_id: parent.id, label: "a" }]);
    relate(
      parentsRepo,
      new ReadonlyRepository(TypeRow, typesConnection),
      new ReadonlyRepository(Child, childrenConnection),
    );
    const item = await parentsRepo.readSingle();
    expect(item?.type_name).toBe("org");
    expect(item?.children.map((child) => child.label)).toEqual(["a"]);
  });
});
