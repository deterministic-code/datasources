import { describe, expect, it } from "vitest";
import { collectAsync } from "./collectAsync.ts";
import { CascadeDeleteEnrichmentStrategy } from "./CascadeDeleteEnrichmentStrategy.ts";
import { DefaultWriteEnrichmentStrategy } from "./DefaultWriteEnrichmentStrategy.ts";
import { DeleteEnrichmentStrategy } from "./DeleteEnrichmentStrategy.ts";
import { LazyWriteEnrichmentStrategy } from "./LazyWriteEnrichmentStrategy.ts";
import { enrichmentsOf } from "./ReadonlyRepository.ts";
import { ListCRUDRepository } from "./tests/listCRUDRepository.ts";
import { ListReadonlyRepository } from "./tests/listReadonlyRepository.ts";

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

class SkipDelete<T> extends DeleteEnrichmentStrategy<T> {
  deleteOwned(): Promise<void> {
    return Promise.resolve();
  }

  deleteLinked(): Promise<void> {
    return Promise.resolve();
  }
}

const assign = <T extends object>(type: new () => T, values: Partial<T>): T =>
  Object.assign(new type(), values);

describe("CRUDRepository", () => {
  it("creates children, stamps FK, and skips select", async () => {
    const parents = new ListCRUDRepository<Parent>();
    const types = new ListCRUDRepository([assign(TypeRow, { id: 10, name: "person" })]);
    const children = new ListCRUDRepository<Child>();
    repoOf(parents, types, children);
    await parents.create(
      assign(Parent, {
        type_id: 10,
        children: [assign(Child, { label: "a" }), assign(Child, { label: "b" })],
      }),
    );
    expect(parents.items[0]?.id).toBe(1);
    expect(children.items.map((child) => child.parent_id)).toEqual([1, 1]);
    expect(children.items.map((child) => child.label)).toEqual(["a", "b"]);
    expect(types.items).toHaveLength(1);
    expect(types.items[0]?.name).toBe("person");
  });

  it("applies generated parent id before child FKs", async () => {
    const parents = new ListCRUDRepository([assign(Parent, { id: 4 })]);
    const children = new ListCRUDRepository<Child>();
    parents.addEnriched({ into: "children", from: "id", by: "parent_id", many: true }, children);
    await parents.create(assign(Parent, { children: [assign(Child, { label: "n" })] }));
    expect(children.items[0]?.parent_id).toBe(5);
  });

  it("update replace-syncs children", async () => {
    const parents = new ListCRUDRepository([assign(Parent, { id: 1, type_id: 10 })]);
    const children = new ListCRUDRepository([
      assign(Child, { id: 1, parent_id: 1, label: "a" }),
      assign(Child, { id: 2, parent_id: 1, label: "b" }),
    ]);
    parents.addEnriched({ into: "children", from: "id", by: "parent_id", many: true }, children);
    await parents.update(
      assign(Parent, {
        id: 1,
        children: [
          assign(Child, { id: 1, label: "aa" }),
          assign(Child, { label: "c" }),
        ],
      }),
    );
    expect(children.items).toHaveLength(2);
    expect(children.items.find((child) => child.id === 1)?.label).toBe("aa");
    expect(children.items.find((child) => child.id !== 1)?.label).toBe("c");
    expect(children.items.every((child) => child.parent_id === 1)).toBe(true);
  });

  it("delete cascades children then parent", async () => {
    const parents = new ListCRUDRepository([assign(Parent, { id: 1 })]);
    const children = new ListCRUDRepository([
      assign(Child, { id: 3, parent_id: 1, label: "a" }),
    ]);
    const notes = new ListCRUDRepository([
      assign(Note, { id: 8, child_id: 3, text: "n" }),
      assign(Note, { id: 9, child_id: 9, text: "other" }),
    ]);
    children.addEnriched({ into: "notes", from: "id", by: "child_id", many: true }, notes);
    parents.addEnriched({ into: "children", from: "id", by: "parent_id", many: true }, children);
    await parents.delete(assign(Parent, { id: 1 }));
    expect(parents.items).toHaveLength(0);
    expect(children.items).toHaveLength(0);
    expect(notes.items.map((note) => note.text)).toEqual(["other"]);
  });

  it("delete strategy can skip children", async () => {
    const children = new ListCRUDRepository([
      assign(Child, { id: 3, parent_id: 1, label: "a" }),
    ]);
    const parents = new ListCRUDRepository(
      new DefaultWriteEnrichmentStrategy(new SkipDelete()),
      [assign(Parent, { id: 1 })],
    );
    parents.addEnriched({ into: "children", from: "id", by: "parent_id", many: true }, children);
    await parents.delete(assign(Parent, { id: 1 }));
    expect(parents.items).toHaveLength(0);
    expect(children.items.map((child) => child.label)).toEqual(["a"]);
  });

  it("writes nested many and skips nested select", async () => {
    const parents = new ListCRUDRepository<Parent>();
    const children = new ListCRUDRepository<Child>();
    const notes = new ListCRUDRepository<Note>();
    const childTypes = new ListCRUDRepository([
      assign(TypeRow, { id: 3, name: "home" }),
    ]);
    children.addEnriched({ into: "type_name", from: "type_id", by: "id", select: "name" }, childTypes);
    children.addEnriched({ into: "notes", from: "id", by: "child_id", many: true }, notes);
    parents.addEnriched({ into: "children", from: "id", by: "parent_id", many: true }, children);
    await parents.create(
      assign(Parent, {
        children: [
          assign(Child, {
            type_id: 3,
            label: "a",
            notes: [assign(Note, { text: "hi" })],
          }),
        ],
      }),
    );
    expect(notes.items[0]?.text).toBe("hi");
    expect(notes.items[0]?.child_id).toBe(children.items[0]?.id);
    expect(childTypes.items[0]?.name).toBe("home");
  });

  it("throws when many via is readonly", async () => {
    const parents = new ListCRUDRepository<Parent>();
    parents.addEnriched({ into: "children", from: "id", by: "parent_id", many: true }, new ListReadonlyRepository<Child>([]));
    await expect(
      parents.create(assign(Parent, { children: [assign(Child, { label: "a" })] })),
    ).rejects.toThrow(/children/);
    await expect(parents.delete(assign(Parent, { id: 1 }))).rejects.toThrow(/children/);
  });

  it("skips non-list child payloads", async () => {
    const parents = new ListCRUDRepository<Parent>();
    const children = new ListCRUDRepository<Child>();
    parents.addEnriched({ into: "children", from: "id", by: "parent_id", many: true }, children);
    await parents.create(Object.assign(assign(Parent, {}), { children: "x" }));
    await parents.create(Object.assign(assign(Parent, {}), { children: { label: "o" } }));
    expect(children.items).toHaveLength(0);
  });

  it("get still enriches", async () => {
    const parents = new ListCRUDRepository([assign(Parent, { id: 1, type_id: 10 })]);
    const types = new ListCRUDRepository([assign(TypeRow, { id: 10, name: "org" })]);
    const children = new ListCRUDRepository([
      assign(Child, { parent_id: 1, label: "a" }),
    ]);
    const item = await repoOf(parents, types, children).readSingle();
    expect(item?.type_name).toBe("org");
    expect(item?.children.map((child) => child.label)).toEqual(["a"]);
  });

  it("skips children that were not loaded", async () => {
    const parents = new ListCRUDRepository(
      new LazyWriteEnrichmentStrategy(),
      [assign(Parent, { id: 1 })],
    );
    const children = new ListCRUDRepository<Child>();
    parents.addEnriched({ into: "children", from: "id", by: "parent_id", many: true }, children);
    await parents.update({ id: 1 } as Parent);
    expect(children.items).toEqual([]);
  });

  it("requires a link repository for an association", async () => {
    const parents = new ListCRUDRepository<Parent>();
    parents.addEnriched(
      {
        into: "children",
        from: "id",
        by: "parent_id",
        many: true,
        association: { parent: "parent_id", related: "child_id" },
      },
      new ListCRUDRepository<Child>(),
    );
    await expect(
      new DefaultWriteEnrichmentStrategy<Parent>().persist(
        parents.enrichmentTree(),
        [assign(Parent, { id: 1, children: [] })],
        "create",
      ),
    ).rejects.toThrow('Deep write requires a CRUD repository for "children"');
  });

  it("rejects unknown persist mode", async () => {
    const parents = new ListCRUDRepository<Parent>();
    parents.addEnriched({ into: "children", from: "id", by: "parent_id", many: true }, new ListCRUDRepository<Child>());
    const write = Reflect.get(parents, "write") as {
      persist(tree: ReturnType<ListCRUDRepository<Parent>["enrichmentTree"]>, items: Parent[], mode: string): Promise<void>;
    };
    await expect(write.persist(parents.enrichmentTree(), [assign(Parent, {})], "nope")).rejects.toThrow(
      /Unknown persist mode/,
    );
  });

  it("builds from parts and options without ids", async () => {
    class Item {
      id = 0;
      name = "";
    }
    const { CRUDRepository } = await import("./CRUDRepository.ts");
    const { MockConnection } = await import("./mocks/mock-connection.ts");
    const { parts } = await import("./RepositoryBuilder.ts");
    const connection = new MockConnection();
    expect(new CRUDRepository(parts(connection, { type: Item, entity: "items", ids: ["id"] })).ids).toEqual([
      "id",
    ]);
    expect(new CRUDRepository(connection, { entity: "items", type: Item }).ids).toEqual(["id"]);
    expect(new CRUDRepository(connection, { entity: "items" }).ids).toEqual(["id"]);
  });

  it("public includeLinked stores the link writer", async () => {
    const parents = new ListCRUDRepository<Parent>();
    const children = new ListCRUDRepository([assign(Child, { id: 2, label: "a" })]);
    const links = new ListCRUDRepository([
      { parent_id: 1, child_id: 2 },
      { parent_id: 1, child_id: null as never },
      { parent_id: 9, child_id: 2 },
    ]);
    parents.addLinked(
      { into: "children", from: "id", by: "id", association: { parent: "parent_id", related: "child_id" } },
      children,
      links,
    );
    expect(enrichmentsOf(parents)?.[0]?.link).toBe(links);
    parents.items.push(assign(Parent, { id: 1 }), Object.assign(assign(Parent, {}), { id: null }));
    const loaded = await collectAsync(parents.read());
    expect(loaded[0]?.children.map((child) => child.label)).toEqual(["a"]);
    expect(loaded[1]?.children).toEqual([]);
  });

  it("skips object ids when loading children", async () => {
    const parents = new ListCRUDRepository<Parent>();
    const children = new ListCRUDRepository<Child>();
    parents.addEnriched({ into: "children", from: "id", by: "parent_id", many: true }, children);
    children.items.push(Object.assign({ parent_id: 1, label: "ghost" }));
    await parents.delete(Object.assign(assign(Parent, {}), { id: { x: 1 } }));
    await parents.update(
      assign(Parent, {
        id: 1,
        children: [
          Object.assign({ Id: 2, label: "z", parent_id: 1 }),
          Object.assign(assign(Child, { label: "n" }), { id: { x: 1 } }),
        ],
      }),
    );
    expect(children.items.some((child) => child.label === "z")).toBe(true);
  });

  it("write identity and empty-id writers", async () => {
    class LabelOnly {
      label = "";
      kids: { label: string }[] = [];
    }
    const parents = new ListCRUDRepository<LabelOnly>([], LabelOnly);
    const kids = new ListCRUDRepository<{ label: string }>();
    parents.addEnriched({ into: "kids", from: "label", by: "label", many: true }, kids);
    const kid = { label: "a" };
    kids.create = async () => kid;
    await parents.create(Object.assign(new LabelOnly(), { label: "a", kids: [kid] }));
    expect(kid.label).toBe("a");
    await parents.update(
      Object.assign(new LabelOnly(), {
        label: "a",
        kids: [],
      }),
    );
    expect(kids.items).toEqual([]);
  });

  it("syncs link rows with missing related ids", async () => {
    const parents = new ListCRUDRepository([assign(Parent, { id: 1 })]);
    const children = new ListCRUDRepository([
      assign(Child, { id: 2, label: "a" }),
      assign(Child, { id: 3, label: "b" }),
    ]);
    const links = new ListCRUDRepository([
      { parent_id: 1, child_id: null as never },
      { parent_id: 1, child_id: 2 },
    ]);
    parents.addLinked({ into: "children", from: "id", by: "id", association: { parent: "parent_id", related: "child_id" } }, children, links);
    await parents.update(
      assign(Parent, {
        id: 1,
        children: [assign(Child, { id: 2, label: "a" }), assign(Child, { id: 3, label: "b" })],
      }),
    );
    expect(links.items.some((link) => link.child_id === 3)).toBe(true);
  });

  it("create returns item when ids are not a single column", async () => {
    class LabelOnly {
      label = "";
      kids: { label: string }[] = [];
    }
    const parents = new ListCRUDRepository<LabelOnly>([], LabelOnly);
    const kids = new ListCRUDRepository<{ label: string }>();
    parents.addEnriched({ into: "kids", from: "label", by: "label", many: true }, kids);
    const item = Object.assign(new LabelOnly(), { label: "a", kids: [] });
    await expect(parents.create(item)).resolves.toBe(item);
  });

  it("create keeps the same object when insert returns it", async () => {
    const parents = new ListCRUDRepository<Parent>();
    const children = new ListCRUDRepository<Child>();
    const item = assign(Parent, { children: [] });
    parents.inserted = async () => item;
    parents.addEnriched({ into: "children", from: "id", by: "parent_id", many: true }, children);
    await expect(parents.create(item)).resolves.toBe(item);
  });

  it("create returns item when id stays unset", async () => {
    class Soft {
      id: number | undefined = undefined;
      name = "";
    }
    const parents = new ListCRUDRepository(
      {
        persist: async () => {},
      },
      [],
      Soft,
    );
    Object.defineProperty(parents, "inserted", {
      value: async (item: Soft) => {
        parents.items.push(item);
        return item;
      },
    });
    parents.addEnriched({ into: "name", from: "id", by: "id", select: "name" }, new ListReadonlyRepository([{ id: 1, name: "x" } as never]));
    const item = Object.assign(new Soft(), { name: "ada" });
    await expect(parents.create(item)).resolves.toBe(item);
  });

  it("link create rejects a missing related id and skips duplicates", async () => {
    const parents = new ListCRUDRepository<Parent>();
    const children = new ListCRUDRepository([
      assign(Child, { id: 2, label: "a" }),
    ]);
    const links = new ListCRUDRepository<{ parent_id: number; child_id: number }>();
    parents.addLinked({ into: "children", from: "id", by: "id", association: { parent: "parent_id", related: "child_id" } }, children, links);
    await expect(
      parents.create(
        assign(Parent, {
          children: [Object.assign(assign(Child, {}), { id: undefined })],
        }),
      ),
    ).rejects.toThrow(/requires "id"/);
    await parents.create(
      assign(Parent, {
        children: [assign(Child, { id: 2, label: "a" }), assign(Child, { id: 2, label: "dup" })],
      }),
    );
    expect(links.items).toHaveLength(1);
  });

  it("uses default id names when the writer has none", async () => {
    const parents = new ListCRUDRepository<Parent>();
    const children = new ListCRUDRepository<Child>();
    (children as { ids: readonly string[] }).ids = [];
    parents.addEnriched({ into: "children", from: "id", by: "parent_id", many: true }, children);
    await parents.create(
      assign(Parent, { children: [assign(Child, { label: "a" })] }),
    );
    expect(children.items[0]?.label).toBe("a");
  });

  it("load skips object parent keys", async () => {
    const children = new ListCRUDRepository<Child>();
    expect(
      await DeleteEnrichmentStrategy.load(
        children,
        { into: "children", from: "id", by: "parent_id", many: true },
        { id: { x: 1 } },
      ),
    ).toEqual([]);
  });

  it("rejects write through a readonly many edge", async () => {
    const parents = new ListCRUDRepository<Parent>();
    parents.addEnriched(
      { into: "children", from: "id", by: "parent_id", many: true },
      new ListReadonlyRepository<Child>([]),
    );
    await expect(
      new DefaultWriteEnrichmentStrategy<Parent>().persist(
        parents.enrichmentTree(),
        [assign(Parent, { id: 1, children: [assign(Child, { label: "a" })] })],
        "create",
      ),
    ).rejects.toThrow(/children/);
  });

  it("link create requires the parent key", async () => {
    const parents = new ListCRUDRepository<Parent>();
    const children = new ListCRUDRepository([assign(Child, { id: 2, label: "a" })]);
    const links = new ListCRUDRepository<{ parent_id: number; child_id: number }>();
    parents.addLinked({ into: "children", from: "id", by: "id", association: { parent: "parent_id", related: "child_id" } }, children, links);
    await expect(
      new DefaultWriteEnrichmentStrategy<Parent>().persist(
        parents.enrichmentTree(),
        [Object.assign(assign(Parent, {}), { id: undefined, children: [assign(Child, { id: 2 })] })],
        "create",
      ),
    ).rejects.toThrow(/requires "id"/);
  });

  it("strips known enrichment fields", async () => {
    const parents = new ListCRUDRepository<Parent>();
    const children = new ListCRUDRepository<Child>();
    const notes = new ListCRUDRepository<Note>();
    children.addEnriched({ into: "notes", from: "id", by: "child_id", many: true }, notes);
    parents.addEnriched({ into: "children", from: "id", by: "parent_id", many: true }, children);
    await parents.create(
      assign(Parent, {
        children: [assign(Child, { label: "a", notes: [assign(Note, { text: "n" })] })],
      }),
    );
    expect(notes.items[0]?.text).toBe("n");
  });

  it("deleteLinked skips missing keys and link ids", async () => {
    const parents = new ListCRUDRepository<Parent>([assign(Parent, { id: 1 })]);
    const children = new ListCRUDRepository([assign(Child, { id: 2, label: "a" })]);
    const links = new ListCRUDRepository([
      { parent_id: 1, child_id: null as never },
      { parent_id: 1, child_id: 2 },
    ]);
    parents.addLinked({ into: "children", from: "id", by: "id", association: { parent: "parent_id", related: "child_id" } }, children, links);
    const entry = enrichmentsOf(parents)![0]!;
    await parents.delete(assign(Parent, { id: 1 }));
    await new CascadeDeleteEnrichmentStrategy().deleteLinked(entry, entry.enrichment, {});
    await expect(
      new CascadeDeleteEnrichmentStrategy().deleteLinked(
        entry,
        { into: "children", from: "id", by: "id", many: true },
        { id: 1 },
      ),
    ).rejects.toThrow(/Missing association/);
  });

  it("cascade delete requires CRUD writers", async () => {
    const ownedParents = new ListCRUDRepository<Parent>();
    ownedParents.addEnriched(
      { into: "children", from: "id", by: "parent_id", many: true },
      new ListReadonlyRepository<Child>([]),
    );
    const linkedParents = new ListCRUDRepository<Parent>();
    linkedParents.addEnriched(
      {
        into: "children",
        from: "id",
        by: "id",
        many: true,
        association: { parent: "parent_id", related: "child_id" },
      },
      new ListReadonlyRepository<Child>([]),
    );
    const owned = enrichmentsOf(ownedParents)![0]!;
    const linked = enrichmentsOf(linkedParents)![0]!;
    await expect(
      new CascadeDeleteEnrichmentStrategy().deleteOwned(owned, owned.enrichment, { id: 1 }, async () => {}),
    ).rejects.toThrow(/children/);
    await expect(
      new CascadeDeleteEnrichmentStrategy().deleteLinked(linked, linked.enrichment, { id: 1 }),
    ).rejects.toThrow(/children/);
  });
});

const repoOf = (
  parents: ListCRUDRepository<Parent>,
  types: ListCRUDRepository<TypeRow>,
  children: ListCRUDRepository<Child>,
): ListCRUDRepository<Parent> => {
  parents.addEnriched({ into: "type_name", from: "type_id", by: "id", select: "name" }, types);
  parents.addEnriched({ into: "children", from: "id", by: "parent_id", many: true }, children);
  return parents;
};
