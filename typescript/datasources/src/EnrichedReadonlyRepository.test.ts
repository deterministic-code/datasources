import { describe, expect, it } from "vitest";
import { collectAsync } from "./collectAsync.ts";
import { EnrichedReadonlyRepository } from "./EnrichedReadonlyRepository.ts";
import { FlatToTypeMapper } from "./mappers/FlatToTypeMapper.ts";
import { FlatToTypeReadonlyEnrichmentStrategy } from "./FlatToTypeReadonlyEnrichmentStrategy.ts";
import { HashsetOptimizedReadonlyEnrichmentStrategy } from "./HashsetOptimizedReadonlyEnrichmentStrategy.ts";
import { ListReadonlyRepository } from "./tests/listReadonlyRepository.ts";
import { RepositoryTreeNode } from "./RepositoryTreeNode.ts";
import { TransformMapper } from "./mappers/TransformMapper.ts";
import type { IQuery } from "./IQuery.ts";
import type { IReadonlyRepository } from "./IReadonlyRepository.ts";
import type { IWhere } from "./IWhere.ts";

class Parent {
  id = 0;
  type_id = 0;
  type_name = "";
  type?: TypeRow;
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

  it("enrich returns the same items", async () => {
    const items = [parent(1, 10)];
    const repo = new EnrichedReadonlyRepository(new ListReadonlyRepository(items));
    expect(await repo.enrich(items)).toBe(items);
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

  it("hashset uses one get per edge", async () => {
    const types = counting(
      new ListReadonlyRepository([
        Object.assign(new TypeRow(), { id: 10, name: "person" }),
        Object.assign(new TypeRow(), { id: 11, name: "org" }),
      ]),
    );
    const children = counting(
      new ListReadonlyRepository([
        Object.assign(new Child(), { parent_id: 1, label: "a" }),
        Object.assign(new Child(), { parent_id: 2, label: "b" }),
      ]),
    );
    const repo = new EnrichedReadonlyRepository(
      new ListReadonlyRepository([parent(1, 10), parent(2, 11)]),
      new HashsetOptimizedReadonlyEnrichmentStrategy(),
    );
    repo.addEnriched("type_name", "type_id", types, "id", "name");
    repo.addEnriched("children", "id", children, "parent_id", undefined, true);
    await collectAsync(repo.get());
    expect(types.gets).toBe(1);
    expect(types.getSingles).toBe(0);
    expect(children.gets).toBe(1);
    expect(children.getSingles).toBe(0);
  });

  it("hashset assigns the related row when select is omitted", async () => {
    const repo = new EnrichedReadonlyRepository(
      new ListReadonlyRepository([parent(1, 10)]),
      new HashsetOptimizedReadonlyEnrichmentStrategy(),
    );
    repo.addEnriched(
      "type",
      "type_id",
      new ListReadonlyRepository([
        Object.assign(new TypeRow(), { id: 10, name: "person" }),
      ]),
      "id",
    );
    const items = await collectAsync(repo.get());
    expect(items[0]).toMatchObject({
      type: { id: 10, name: "person" },
    });
  });

  it("hashset skips missing keys and unmatched rows", async () => {
    const blank = Object.assign(parent(2, 0), { type_id: undefined });
    const repo = new EnrichedReadonlyRepository(
      new ListReadonlyRepository([parent(1, 10), parent(3, 99), blank]),
      new HashsetOptimizedReadonlyEnrichmentStrategy(),
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
    repo.addEnriched(
      "children",
      "id",
      new ListReadonlyRepository([
        Object.assign(new Child(), { label: "orphan" }),
        Object.assign(new Child(), { parent_id: 1, label: "a" }),
        Object.assign(new Child(), { parent_id: 1, label: "b" }),
      ]),
      "parent_id",
      undefined,
      true,
    );
    const items = await collectAsync(repo.get());
    expect(items[0]?.type_name).toBe("person");
    expect(items[0]?.children.map((child) => child.label)).toEqual(["a", "b"]);
    expect(items[1]?.type_name).toBe("");
    expect(items[1]?.children).toEqual([]);
    expect(items[2]?.children).toEqual([]);
    expect(new RepositoryTreeNode({}, "").value).toEqual({});
    const strategy = new HashsetOptimizedReadonlyEnrichmentStrategy();
    await strategy.apply(
      new ListReadonlyRepository<TypeRow>([]),
      { into: "type_name", from: "type_id", by: "id" },
      [{ id: 1 }],
    );
    await strategy.apply(
      {
        type: Object,
        getSingle: async () => undefined,
        async *get() {},
      },
      { into: "children", from: "id", by: "parent_id", many: true },
      [{}],
    );
    await strategy.apply(
      {
        type: Object,
        getSingle: async () => undefined,
        async *get() {
          yield { label: "missing" };
          yield { parent_id: null, label: "n" };
          yield { parent_id: undefined, label: "u" };
          yield { parent_id: 1, label: "a" };
          yield { parent_id: 1, label: "b" };
        },
      },
      { into: "children", from: "id", by: "parent_id", many: true },
      [{ id: 1 }, {}],
    );
  });

  it("collapses join rows with FlatToType", async () => {
    const strategy = new FlatToTypeReadonlyEnrichmentStrategy(
      new ListReadonlyRepository([
        { id: 1, type_name: "person" },
        { id: 2, type_name: "org" },
      ]),
      new FlatToTypeMapper({
        key: "id",
        fields: { id: "id", type_name: "type_name" },
      }),
      new TransformMapper(
        (item) => item,
        (item) => item,
      ),
    );
    const repo = new EnrichedReadonlyRepository(
      new ListReadonlyRepository<Parent>([]),
      strategy,
    );
    const items = await collectAsync(repo.get());
    expect(items.map((item) => item.id)).toEqual([1, 2]);
    expect(items.map((item) => item.type_name)).toEqual(["person", "org"]);
    await strategy.enrich(repo.tree, items);
    await strategy.apply(
      new ListReadonlyRepository<TypeRow>([]),
      { into: "type_name", from: "type_id", by: "id" },
      items,
    );
  });
});

const counting = <T extends object>(
  inner: IReadonlyRepository<T>,
): IReadonlyRepository<T> & { gets: number; getSingles: number } => {
  const state = { gets: 0, getSingles: 0 };
  return {
    type: inner.type,
    get gets() {
      return state.gets;
    },
    get getSingles() {
      return state.getSingles;
    },
    getSingle: async (query?: IQuery | IWhere) => {
      state.getSingles += 1;
      return inner.getSingle(query);
    },
    async *get(query?: IQuery | IWhere) {
      state.gets += 1;
      yield* inner.get(query);
    },
  };
};
