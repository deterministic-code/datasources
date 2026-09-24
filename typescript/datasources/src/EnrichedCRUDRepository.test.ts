import { describe, expect, it } from "vitest";
import { CRUDRepository } from "./CRUDRepository.ts";
import { MockCommand } from "./mocks/mock-command.ts";
import { MockConnection } from "./mocks/mock-connection.ts";
import { MockReader } from "./mocks/mock-reader.ts";
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

const assign = <T extends object>(type: new () => T, values: Partial<T>): T =>
  Object.assign(new type(), values);

describe("CRUDRepository", () => {
  it("creates children, stamps FK, and skips select", async () => {
    const parents = new ListCRUDRepository<Parent>();
    const types = new ListCRUDRepository([assign(TypeRow, { id: 10, name: "person" })]);
    const children = new ListCRUDRepository<Child>();
    const repo = repoOf(parents, types, children);
    await repo.create(
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
    const repo = new CRUDRepository(parents);
    repo.addEnriched("children", "id", children, "parent_id", undefined, true);
    await repo.create(assign(Parent, { children: [assign(Child, { label: "n" })] }));
    expect(children.items[0]?.parent_id).toBe(5);
  });

  it("update replace-syncs children", async () => {
    const parents = new ListCRUDRepository([assign(Parent, { id: 1, type_id: 10 })]);
    const children = new ListCRUDRepository([
      assign(Child, { id: 1, parent_id: 1, label: "a" }),
      assign(Child, { id: 2, parent_id: 1, label: "b" }),
    ]);
    const repo = new CRUDRepository(parents);
    repo.addEnriched("children", "id", children, "parent_id", undefined, true);
    await repo.update(
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
    const repo = new CRUDRepository(parents);
    repo.addEnriched("children", "id", children, "parent_id", undefined, true, (node) =>
      node.include(notes, "notes", "id", "child_id", undefined, true),
    );
    await repo.delete(assign(Parent, { id: 1 }));
    expect(parents.items).toHaveLength(0);
    expect(children.items).toHaveLength(0);
    expect(notes.items.map((note) => note.text)).toEqual(["other"]);
  });

  it("writes nested many and skips nested select", async () => {
    const parents = new ListCRUDRepository<Parent>();
    const children = new ListCRUDRepository<Child>();
    const notes = new ListCRUDRepository<Note>();
    const childTypes = new ListCRUDRepository([
      assign(TypeRow, { id: 3, name: "home" }),
    ]);
    const repo = new CRUDRepository(parents);
    repo.addEnriched("children", "id", children, "parent_id", undefined, true, (node) => {
      node.include(childTypes, "type_name", "type_id", "id", "name");
      node.include(notes, "notes", "id", "child_id", undefined, true);
    });
    await repo.create(
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
    const repo = new CRUDRepository(new ListCRUDRepository<Parent>());
    repo.addEnriched(
      "children",
      "id",
      new ListReadonlyRepository<Child>([]),
      "parent_id",
      undefined,
      true,
    );
    await expect(
      repo.create(assign(Parent, { children: [assign(Child, { label: "a" })] })),
    ).rejects.toThrow(/children/);
  });

  it("skips non-list child payloads", async () => {
    const parents = new ListCRUDRepository<Parent>();
    const children = new ListCRUDRepository<Child>();
    const repo = new CRUDRepository(parents);
    repo.addEnriched("children", "id", children, "parent_id", undefined, true);
    await repo.create(Object.assign(assign(Parent, {}), { children: "x" }));
    await repo.create(Object.assign(assign(Parent, {}), { children: { label: "o" } }));
    expect(children.items).toHaveLength(0);
  });

  it("applies Id identity and skips empty results", async () => {
    class Capital {
      Id = 0;
    }
    const ids = new ListCRUDRepository<Capital>();
    await new CRUDRepository(ids).create(new Capital());
    expect(ids.items[0]?.Id).toBe(1);
    const bare = new ListCRUDRepository<object>();
    bare.create = async (item) => {
      bare.items.push(item);
      return { result: 1, rowsAffected: 1 };
    };
    await new CRUDRepository(bare).create({});
    expect(bare.items[0]).toEqual({});
    const empty = new ListCRUDRepository<object>();
    empty.create = async (item) => {
      empty.items.push(item);
      return { result: null, rowsAffected: 1 };
    };
    await new CRUDRepository(empty).create({ id: 1 });
    expect(empty.items[0]).toEqual({ id: 1 });
  });

  it("skips object ids when loading children", async () => {
    const parents = new ListCRUDRepository<Parent>();
    const children = new ListCRUDRepository<Child>();
    const repo = new CRUDRepository(parents);
    repo.addEnriched("children", "id", children, "parent_id", undefined, true);
    children.items.push(Object.assign({ parent_id: 1, label: "ghost" }));
    await repo.delete(Object.assign(assign(Parent, {}), { id: { x: 1 } }));
    await repo.update(
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

  it("stamps bigint identity", async () => {
    const parents = new ListCRUDRepository<Parent>();
    parents.create = async (item) => {
      Object.assign(item, { id: 9n });
      parents.items.push(item);
      return { result: 9n, rowsAffected: 1 };
    };
    await new CRUDRepository(parents).create(assign(Parent, {}));
    expect(parents.items[0]?.id).toBe(9n);
  });

  it("rejects unknown persist mode", async () => {
    const repo = new CRUDRepository(new ListCRUDRepository<Parent>());
    repo.addEnriched("children", "id", new ListCRUDRepository<Child>(), "parent_id", undefined, true);
    const persistNode = Reflect.get(repo, "persistNode") as (
      child: object,
      parents: object[],
      mode: string,
    ) => Promise<void>;
    await expect(persistNode.call(repo, repo.tree.parent.items[0], [{}], "nope")).rejects.toThrow(
      /Unknown persist mode/,
    );
  });

  it("applies boolean and bigint identity and reads one enriched row by id", async () => {
    const parents = new ListCRUDRepository<Parent>();
    const children = new ListCRUDRepository<Child>();
    const repo = new CRUDRepository(parents);
    repo.addEnriched("children", "id", children, "parent_id", undefined, true);
    parents.create = async (item) => {
      parents.items.push(item);
      return { result: true, rowsAffected: 1 };
    };
    const flagged = assign(Parent, { children: [] });
    await repo.create(flagged);
    expect(flagged.id).toBe(true);
    parents.create = async (item) => {
      parents.items.push(item);
      return { result: 4n, rowsAffected: 1 };
    };
    const numbered = assign(Parent, { children: [] });
    await repo.create(numbered);
    expect(numbered.id).toBe(4n);
    parents.items.push(assign(Parent, { id: 3 }));
    await repo.enrich(parents.items);
    const loaded = await repo.readSingleById(3);
    expect(loaded?.children).toEqual([]);
    expect(await repo.readSingleById(99)).toBeUndefined();
    parents.create = async (item) => {
      parents.items.push(item);
      return { result: null, rowsAffected: 1 };
    };
    await repo.create(assign(Parent, { children: [] }));
    parents.create = async (item) => {
      parents.items.push(item);
      return { result: { raw: 1 }, rowsAffected: 1 };
    };
    const boxed = assign(Parent, { id: 0, children: [] });
    await repo.create(boxed);
    expect(boxed.id).toBe(0);
    children.create = async (item) => {
      children.items.push(item);
      return { result: null, rowsAffected: 1 };
    };
    await repo.create(assign(Parent, { children: [assign(Child, { label: "z" })] }));
    await repo.update(assign(Parent, { id: 1, children: [] }));
  });

  it("writes enriched sql rows and reads them by id", async () => {
    class RowItem {
      id = 0;
      name = "";
      children: Child[] = [];
    }
    const connection = new MockConnection(undefined, "memory");
    const command = new MockCommand(connection);
    connection.command = command;
    command.response = { result: 8, rowsAffected: 1 };
    command.reader = new MockReader([{ id: 8, name: "ada" }]);
    const children = new ListCRUDRepository<Child>();
    const repo = new CRUDRepository(RowItem, connection);
    repo.addEnriched("children", "id", children, "parent_id", undefined, true);
    const item = Object.assign(new RowItem(), { name: "ada", children: [assign(Child, { label: "a" })] });
    await repo.create(item);
    expect(item.id).toBe(8);
    expect(children.items[0]?.parent_id).toBe(8);
    await repo.update(item);
    await repo.readSingleById(8);
    await repo.delete(item);
    expect(command.executeCalls.map((call) => call.text)).toEqual([
      "INSERT INTO RowItem (name) VALUES ($name)",
      "UPDATE RowItem SET name = $name WHERE id = $id",
      "DELETE FROM RowItem WHERE id = $id",
    ]);
  });

  it("reuses a nested repository's id names", async () => {
    const connection = new MockConnection(undefined, "memory");
    const command = new MockCommand(connection);
    connection.command = command;
    command.response = { result: 3, rowsAffected: 1 };
    const children = new CRUDRepository(Child, connection);
    const wrapped = new CRUDRepository(children);
    const child = assign(Child, { label: "a" });
    const parents = new ListCRUDRepository<Parent>();
    const repo = new CRUDRepository(parents);
    repo.addEnriched("children", "id", wrapped, "parent_id", undefined, true);
    await repo.create(assign(Parent, { children: [child] }));
    expect(wrapped.ids).toEqual(["id"]);
    expect(child.id).toBe(3);
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
});

const repoOf = (
  parents: ListCRUDRepository<Parent>,
  types: ListCRUDRepository<TypeRow>,
  children: ListCRUDRepository<Child>,
): CRUDRepository<Parent> => {
  const repo = new CRUDRepository(parents);
  repo.addEnriched("type_name", "type_id", types, "id", "name");
  repo.addEnriched("children", "id", children, "parent_id", undefined, true);
  return repo;
};
