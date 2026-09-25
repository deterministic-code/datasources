import { describe, expect, it } from "vitest";
import { BulkCRUDRepository } from "./BulkCRUDRepository.ts";
import { ListCRUDRepository } from "./tests/listCRUDRepository.ts";

class Parent {
  id = 0;
  children: Child[] = [];
}

class Child {
  id = 0;
  parent_id = 0;
  label = "";
}

const assign = <T extends object>(type: new () => T, values: Partial<T>): T =>
  Object.assign(new type(), values);

const repoOf = (
  parents: ListCRUDRepository<Parent>,
  children: ListCRUDRepository<Child>,
): BulkCRUDRepository<Parent> => {
  const repo = new BulkCRUDRepository(parents);
  repo.addEnriched("children", "id", children, "parent_id", undefined, true);
  return repo;
};

describe("BulkCRUDRepository", () => {
  it("creates parents and children", async () => {
    const parents = new ListCRUDRepository<Parent>();
    const children = new ListCRUDRepository<Child>();
    const response = await repoOf(parents, children).createBulk([
      assign(Parent, { children: [assign(Child, { label: "a" })] }),
      assign(Parent, {
        children: [assign(Child, { label: "b" }), assign(Child, { label: "c" })],
      }),
    ]);
    expect(parents.items.map((parent) => parent.id)).toEqual([1, 2]);
    expect(children.items.map((child) => child.parent_id)).toEqual([1, 2, 2]);
    expect(response.rowsAffected).toBe(2);
  });

  it("update replace-syncs each parent", async () => {
    const parents = new ListCRUDRepository([
      assign(Parent, { id: 1 }),
      assign(Parent, { id: 2 }),
    ]);
    const children = new ListCRUDRepository([
      assign(Child, { id: 1, parent_id: 1, label: "a" }),
      assign(Child, { id: 2, parent_id: 2, label: "b" }),
    ]);
    await repoOf(parents, children).updateBulk([
      assign(Parent, {
        id: 1,
        children: [assign(Child, { id: 1, label: "aa" })],
      }),
      assign(Parent, { id: 2, children: [assign(Child, { label: "c" })] }),
    ]);
    expect(children.items).toHaveLength(2);
    expect(children.items.find((child) => child.parent_id === 1)?.label).toBe(
      "aa",
    );
    expect(children.items.find((child) => child.parent_id === 2)?.label).toBe(
      "c",
    );
    expect(children.items.some((child) => child.id === 2)).toBe(false);
  });

  it("delete cascades each parent", async () => {
    const parents = new ListCRUDRepository([
      assign(Parent, { id: 1 }),
      assign(Parent, { id: 2 }),
    ]);
    const children = new ListCRUDRepository([
      assign(Child, { id: 1, parent_id: 1, label: "a" }),
      assign(Child, { id: 2, parent_id: 2, label: "b" }),
      assign(Child, { id: 3, parent_id: 9, label: "keep" }),
    ]);
    await repoOf(parents, children).deleteBulk([
      assign(Parent, { id: 1 }),
      assign(Parent, { id: 2 }),
    ]);
    expect(parents.items).toHaveLength(0);
    expect(children.items.map((child) => child.label)).toEqual(["keep"]);
  });

  it("reads an Id column and ignores an object id", async () => {
    class Named {
      Id = 0;
    }
    const named = await new BulkCRUDRepository(new ListCRUDRepository<Named>()).createBulk([new Named()]);
    expect(named.result).toBe(1);

    const inner = new ListCRUDRepository<{ id: object }>();
    inner.create = async () => ({ id: { n: 1 } });
    const boxed = await new BulkCRUDRepository(inner).createBulk([{} as { id: object }]);
    expect(boxed.result).toBeNull();
    expect(boxed.rowsAffected).toBe(1);
  });
});
