import { describe, expect, it } from "vitest";
import { collectAsync } from "./collectAsync.ts";
import { EnrichedReadonlyRepository } from "./EnrichedReadonlyRepository.ts";
import { FlatToNestedMapper } from "./mappers/FlatToNestedMapper.ts";
import { FlatToNestedReadonlyEnrichmentStrategy } from "./FlatToNestedReadonlyEnrichmentStrategy.ts";
import { HashsetOptimizedReadonlyEnrichmentStrategy } from "./HashsetOptimizedReadonlyEnrichmentStrategy.ts";
import { ListReadonlyRepository } from "./tests/listReadonlyRepository.ts";
import { RepositoryTreeNode } from "./RepositoryTreeNode.ts";
import { TransformMapper } from "./mappers/TransformMapper.ts";
import type { IExecuteStoredProcedureStatement } from "./statements/IExecuteStoredProcedureStatement.ts";
import type { IQuery } from "./statements/IQuery.ts";
import type { IReadonlyRepository } from "./IReadonlyRepository.ts";
import { Operator, Where } from "./statements/index.ts";
import { WhereText } from "./statements/WhereText.ts";
import type { IWhere } from "./statements/IWhere.ts";

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
  it("reads one row through a where clause", async () => {
    const repo = new EnrichedReadonlyRepository(new ListReadonlyRepository([parent(1, 0)]));
    const byClause = await collectAsync(repo.read(new Where(new Operator.Equals("id", 1))));
    const byWhere = await collectAsync(repo.read(new WhereText("id = 1")));
    expect(byClause.map((item) => item.id)).toEqual([1]);
    expect(byWhere.map((item) => item.id)).toEqual([1]);
  });

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
    const items = await collectAsync(repo.read());
    expect(items[0]?.type_name).toBe("person");
    expect(items[1]?.type_name).toBe("");
  });

  it("fills lists and readSingle is enriched", async () => {
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
    const item = await repo.readSingle();
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
    const item = await repo.readSingle();
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
    const items = await collectAsync(repo.read());
    expect(items[0]?.type_name).toBe("");
    expect(items[1]?.children).toBeUndefined();
  });

  it("readSingle is undefined when empty", async () => {
    const repo = new EnrichedReadonlyRepository(new ListReadonlyRepository<Parent>([]));
    expect(await repo.readSingle()).toBeUndefined();
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
    const items = await collectAsync(repo.read());
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
    await expect(collectAsync(repo.read())).rejects.toThrow(/scalar/);
  });

  it("hashset uses one read per edge", async () => {
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
    await collectAsync(repo.read());
    expect(types.reads).toBe(1);
    expect(types.readSingles).toBe(0);
    expect(children.reads).toBe(1);
    expect(children.readSingles).toBe(0);
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
    const items = await collectAsync(repo.read());
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
    const items = await collectAsync(repo.read());
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
        readSingle: async () => undefined,
        async *read() {},
      },
      { into: "children", from: "id", by: "parent_id", many: true },
      [{}],
    );
    await strategy.apply(
      {
        type: Object,
        readSingle: async () => undefined,
        async *read() {
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

  it("collapses join rows with FlatToNested", async () => {
    const strategy = new FlatToNestedReadonlyEnrichmentStrategy(
      new ListReadonlyRepository([
        { id: 1, type_name: "person" },
        { id: 2, type_name: "org" },
      ]),
      new FlatToNestedMapper({
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
    const items = await collectAsync(repo.read());
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
): IReadonlyRepository<T> & { reads: number; readSingles: number } => {
  const state = { reads: 0, readSingles: 0 };
  return {
    type: inner.type,
    get reads() {
      return state.reads;
    },
    get readSingles() {
      return state.readSingles;
    },
    readSingle: async (query?: IQuery | IWhere | IExecuteStoredProcedureStatement) => {
      state.readSingles += 1;
      return inner.readSingle(query);
    },
    async *read(query?: IQuery | IWhere | IExecuteStoredProcedureStatement) {
      state.reads += 1;
      yield* inner.read(query);
    },
  };
};
