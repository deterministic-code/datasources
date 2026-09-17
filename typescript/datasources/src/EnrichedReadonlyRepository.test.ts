import { describe, expect, it } from "vitest";
import { collectAsync } from "./collectAsync.ts";
import { EnrichedReadonlyRepository } from "./EnrichedReadonlyRepository.ts";
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
  parent_id = 0;
  type_id = 0;
  label = "";
  type_name = "";
}

const parent = (id: number, type_id: number): Parent =>
  Object.assign(new Parent(), { id, type_id });

describe("EnrichedReadonlyRepository", () => {
  it("fills lookup select and leaves missing default", async () => {
    const repo = new EnrichedReadonlyRepository(
      new ListReadonlyRepository([parent(1, 10), parent(2, 99)]),
    );
    repo.addEnriched(
      "type_name",
      "type_id",
      new ListReadonlyRepository([
        Object.assign(new TypeRow(), { id: 10, name: "person" }),
      ]),
      "id",
      "name",
    );
    const items = await collectAsync(repo.get());
    expect(items[0]?.type_name).toBe("person");
    expect(items[1]?.type_name).toBe("");
  });

  it("fills lists and getSingle is enriched", async () => {
    const repo = new EnrichedReadonlyRepository(
      new ListReadonlyRepository([parent(1, 10)]),
    );
    repo.addEnriched(
      "type_name",
      "type_id",
      new ListReadonlyRepository([
        Object.assign(new TypeRow(), { id: 10, name: "org" }),
      ]),
      "id",
      "name",
    );
    repo.addEnriched(
      "children",
      "id",
      new ListReadonlyRepository([
        Object.assign(new Child(), { parent_id: 1, label: "a" }),
        Object.assign(new Child(), { parent_id: 1, label: "b" }),
        Object.assign(new Child(), { parent_id: 2, label: "x" }),
      ]),
      "parent_id",
      undefined,
      true,
    );
    const item = await repo.getSingle();
    expect(item?.type_name).toBe("org");
    expect(item?.children.map((child) => child.label)).toEqual(["a", "b"]);
  });

  it("nests list lookup and keeps root sibling", async () => {
    const repo = new EnrichedReadonlyRepository(
      new ListReadonlyRepository([parent(1, 10)]),
    );
    repo.addEnriched(
      "type_name",
      "type_id",
      new ListReadonlyRepository([
        Object.assign(new TypeRow(), { id: 10, name: "org" }),
      ]),
      "id",
      "name",
    );
    repo.addEnriched(
      "children",
      "id",
      new ListReadonlyRepository([
        Object.assign(new Child(), { parent_id: 1, type_id: 3, label: "a" }),
        Object.assign(new Child(), { parent_id: 1, type_id: 9, label: "b" }),
      ]),
      "parent_id",
      undefined,
      true,
      (node) =>
        node.include(
          new ListReadonlyRepository([
            Object.assign(new TypeRow(), { id: 3, name: "home" }),
          ]),
          "type_name",
          "type_id",
          "id",
          "name",
        ),
    );
    const item = await repo.getSingle();
    expect(item?.type_name).toBe("org");
    expect(item?.children[0]?.type_name).toBe("home");
    expect(item?.children[1]?.type_name).toBe("");
  });

  it("skips missing enrichment keys", async () => {
    const repo = new EnrichedReadonlyRepository(
      new ListReadonlyRepository<object>([
        { type_id: null, type_name: "", children: [] },
        { type_name: "" },
      ]),
    );
    repo.addEnriched(
      "type_name",
      "type_id",
      new ListReadonlyRepository([Object.assign(new TypeRow(), { id: 10, name: "person" })]),
      "id",
      "name",
    );
    repo.addEnriched(
      "children",
      "id",
      new ListReadonlyRepository([Object.assign(new Child(), { parent_id: 1, label: "a" })]),
      "parent_id",
      undefined,
      true,
    );
    const items = await collectAsync(repo.get());
    expect(items[0]?.type_name).toBe("");
    expect(items[1]?.children).toBeUndefined();
  });

  it("getSingle is undefined when empty", async () => {
    const repo = new EnrichedReadonlyRepository(new ListReadonlyRepository<Parent>([]));
    expect(await repo.getSingle()).toBeUndefined();
  });

  it("nests on a singular related object and skips strings", async () => {
    const repo = new EnrichedReadonlyRepository(
      new ListReadonlyRepository([
        Object.assign(parent(1, 10), { child: "skip" }),
        parent(2, 10),
      ]),
    );
    repo.addEnriched(
      "child",
      "id",
      new ListReadonlyRepository([
        Object.assign(new Child(), { parent_id: 2, type_id: 3, label: "only" }),
      ]),
      "parent_id",
      undefined,
      false,
      (node) =>
        node.include(
          new ListReadonlyRepository([
            Object.assign(new TypeRow(), { id: 3, name: "home" }),
          ]),
          "type_name",
          "type_id",
          "id",
          "name",
        ),
    );
    const items = await collectAsync(repo.get());
    expect(items[0]?.child).toBe("skip");
    expect((items[1] as Parent & { child?: Child }).child?.type_name).toBe("home");
  });

  it("throws when an enrichment key is an object", async () => {
    const repo = new EnrichedReadonlyRepository(
      new ListReadonlyRepository([Object.assign(parent(1, 10), { type_id: { x: 1 } })]),
    );
    repo.addEnriched(
      "type_name",
      "type_id",
      new ListReadonlyRepository<TypeRow>([]),
      "id",
      "name",
    );
    await expect(collectAsync(repo.get())).rejects.toThrow(/scalar/);
  });
});
