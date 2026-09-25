import { describe, expect, it } from "vitest";
import { collectAsync } from "./collectAsync.ts";
import { DefaultReadonlyEnrichmentStrategy } from "./DefaultReadonlyEnrichmentStrategy.ts";
import { NestedToObjectEnrichmentStrategy } from "./NestedToObjectEnrichmentStrategy.ts";
import { FlatToNestedMapper } from "./mappers/FlatToNestedMapper.ts";
import { HashsetOptimizedReadonlyEnrichmentStrategy } from "./HashsetOptimizedReadonlyEnrichmentStrategy.ts";
import { ListReadonlyRepository } from "./tests/listReadonlyRepository.ts";
import { ReadonlyRepository } from "./ReadonlyRepository.ts";
import { RepositoryTreeNode } from "./RepositoryTreeNode.ts";
import { TransformMapper } from "./mappers/TransformMapper.ts";
import type { Parameter } from "./Parameter.ts";
import type { IExecuteStoredProcedureStatement } from "./statements/IExecuteStoredProcedureStatement.ts";
import type { ISelectStatement } from "./statements/ISelectStatement.ts";
import type { IReadonlyRepository } from "./IReadonlyRepository.ts";
import type { IReadonlyEnrichmentStrategy } from "./IReadonlyEnrichmentStrategy.ts";
import type { Cells } from "./converters/Cell.ts";
import type { IMapper } from "./mappers/IMapper.ts";
import { Operator, Where } from "./statements/index.ts";
import { WhereText } from "./statements/WhereText.ts";
import type { IWhere } from "./statements/IWhere.ts";

class Parent {
  id = 0;
  type_id = 0;
  type_name = "";
  type?: TypeRow;
  children: Child[] = [];
  child?: Child | string;
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

class View {
  UserType = "";
}

class SourcedView extends ReadonlyRepository<View> {
  protected async *readItems() {
    yield { item: new View(), source: { UserType: 1 } };
  }
}

class JoinRows extends ReadonlyRepository<Parent> {
  constructor(
    private readonly rows: Cells[],
    result: IMapper<Iterable<Cells>, Parent[]>,
  ) {
    super();
    this.addEnriched(
      { into: "type_name", from: "id", by: "id" },
      new ListReadonlyRepository<Parent>(),
      new NestedToObjectEnrichmentStrategy(result),
    );
  }

  protected async *readItems() {
    for (const row of this.rows) yield { item: new Parent(), source: row };
  }
}

const parent = (id: number, type_id: number): Parent =>
  Object.assign(new Parent(), { id, type_id });

const fillLookup = async (strategy: IReadonlyEnrichmentStrategy<Parent>) => {
  const parents = new ListReadonlyRepository([parent(1, 10), parent(2, 99)]);
  const types = new ListReadonlyRepository([
    Object.assign(new TypeRow(), { id: 10, name: "person" }),
  ]);
  parents.addEnriched({ into: "type_name", from: "type_id", by: "id", select: "name" }, types, strategy);
  const items = await collectAsync(parents.read());
  expect(items[0]?.type_name).toBe("person");
  expect(items[1]?.type_name).toBe("");
};

const fillLists = async (strategy: IReadonlyEnrichmentStrategy<Parent>) => {
  const parents = new ListReadonlyRepository([parent(1, 10)]);
  const types = new ListReadonlyRepository([
    Object.assign(new TypeRow(), { id: 10, name: "org" }),
  ]);
  const children = new ListReadonlyRepository([
    Object.assign(new Child(), { parent_id: 1, label: "a" }),
    Object.assign(new Child(), { parent_id: 1, label: "b" }),
    Object.assign(new Child(), { parent_id: 2, label: "x" }),
  ]);
  parents.addEnriched({ into: "type_name", from: "type_id", by: "id", select: "name" }, types, strategy);
  parents.addEnriched({ into: "children", from: "id", by: "parent_id", many: true }, children, strategy);
  const item = await parents.readSingle();
  expect(item?.type_name).toBe("org");
  expect(item?.children.map((child) => child.label)).toEqual(["a", "b"]);
};

const nestListLookup = async (strategy: IReadonlyEnrichmentStrategy<Parent>) => {
  const parents = new ListReadonlyRepository([parent(1, 10)]);
  const types = new ListReadonlyRepository([
    Object.assign(new TypeRow(), { id: 10, name: "org" }),
  ]);
  const childTypes = new ListReadonlyRepository([
    Object.assign(new TypeRow(), { id: 3, name: "home" }),
  ]);
  const children = new ListReadonlyRepository([
    Object.assign(new Child(), { parent_id: 1, type_id: 3, label: "a" }),
    Object.assign(new Child(), { parent_id: 1, type_id: 9, label: "b" }),
  ]);
  children.addEnriched({ into: "type_name", from: "type_id", by: "id", select: "name" }, childTypes);
  parents.addEnriched({ into: "type_name", from: "type_id", by: "id", select: "name" }, types, strategy);
  parents.addEnriched({ into: "children", from: "id", by: "parent_id", many: true }, children, strategy);
  const item = await parents.readSingle();
  expect(item?.type_name).toBe("org");
  expect(item?.children[0]?.type_name).toBe("home");
  expect(item?.children[1]?.type_name).toBe("");
};

describe("ReadonlyRepository", () => {
  it("reads one row through a where clause", async () => {
    const repo = new ListReadonlyRepository([parent(1, 0)]);
    const byClause = await collectAsync(repo.read(new Where(new Operator.Equals("id", 1))));
    const byWhere = await collectAsync(repo.read(new WhereText("id = 1")));
    expect(byClause.map((item) => item.id)).toEqual([1]);
    expect(byWhere.map((item) => item.id)).toEqual([1]);
  });

  it("fills lookup select and leaves missing default", () =>
    fillLookup(new DefaultReadonlyEnrichmentStrategy()));

  it("hashset fills lookup select and leaves missing default", () =>
    fillLookup(new HashsetOptimizedReadonlyEnrichmentStrategy()));

  it("fills lists and readSingle is enriched", () =>
    fillLists(new DefaultReadonlyEnrichmentStrategy()));

  it("hashset fills lists and readSingle is enriched", () =>
    fillLists(new HashsetOptimizedReadonlyEnrichmentStrategy()));

  it("nests list lookup and keeps root sibling", () =>
    nestListLookup(new DefaultReadonlyEnrichmentStrategy()));

  it("hashset nests list lookup and keeps root sibling", () =>
    nestListLookup(new HashsetOptimizedReadonlyEnrichmentStrategy()));

  it("skips missing enrichment keys", async () => {
    const repo = new ListReadonlyRepository<object>([
      { type_id: null, type_name: "", children: [] },
      { type_name: "" },
    ]);
    repo.addEnriched({ into: "type_name", from: "type_id", by: "id", select: "name" }, new ListReadonlyRepository([Object.assign(new TypeRow(), { id: 10, name: "person" })]));
    repo.addEnriched({ into: "children", from: "id", by: "parent_id", many: true }, new ListReadonlyRepository([Object.assign(new Child(), { parent_id: 1, label: "a" })]));
    const items = await collectAsync(repo.read());
    expect(items[0]?.type_name).toBe("");
    expect(items[1]?.children).toBeUndefined();
  });

  it("readSingle is undefined when empty", async () => {
    expect(await new ListReadonlyRepository<Parent>([]).readSingle()).toBeUndefined();
  });

  it("enrich returns void", async () => {
    const items = [parent(1, 10)];
    const repo = new ListReadonlyRepository(items);
    await expect(repo.enrich(items)).resolves.toBeUndefined();
  });

  it("nests on a singular related object and skips strings", async () => {
    const repo = new ListReadonlyRepository([
      Object.assign(parent(1, 10), { child: "skip" }),
      parent(2, 10),
    ]);
    const children = new ListReadonlyRepository([
      Object.assign(new Child(), { parent_id: 2, type_id: 3, label: "only" }),
    ]);
    children.addEnriched({ into: "type_name", from: "type_id", by: "id", select: "name" }, new ListReadonlyRepository([
        Object.assign(new TypeRow(), { id: 3, name: "home" }),
      ]));
    repo.addEnriched({ into: "child", from: "id", by: "parent_id" }, children);
    const items = await collectAsync(repo.read());
    expect(items[0]?.child).toBe("skip");
    expect(
      items[1]?.child && typeof items[1].child === "object"
        ? items[1].child.type_name
        : undefined,
    ).toBe("home");
  });

  it("throws when an enrichment key is an object", async () => {
    const repo = new ListReadonlyRepository([
      Object.assign(parent(1, 10), { type_id: { x: 1 } }),
    ]);
    repo.addEnriched({ into: "type_name", from: "type_id", by: "id", select: "name" }, new ListReadonlyRepository<TypeRow>([]));
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
    const parents = new ListReadonlyRepository([parent(1, 10), parent(2, 11)]);
    const strategy = new HashsetOptimizedReadonlyEnrichmentStrategy<Parent>();
    parents.addEnriched({ into: "type_name", from: "type_id", by: "id", select: "name" }, types, strategy);
    parents.addEnriched({ into: "children", from: "id", by: "parent_id", many: true }, children, strategy);
    await collectAsync(parents.read());
    expect(types.reads).toBe(1);
    expect(types.readSingles).toBe(0);
    expect(children.reads).toBe(1);
    expect(children.readSingles).toBe(0);
  });

  it("default uses readSingle per lookup key", async () => {
    const types = counting(
      new ListReadonlyRepository([
        Object.assign(new TypeRow(), { id: 10, name: "person" }),
        Object.assign(new TypeRow(), { id: 11, name: "org" }),
      ]),
    );
    const parents = new ListReadonlyRepository([parent(1, 10), parent(2, 11)]);
    parents.addEnriched({ into: "type_name", from: "type_id", by: "id", select: "name" }, types);
    await collectAsync(parents.read());
    expect(types.readSingles).toBe(2);
    expect(types.reads).toBe(0);
  });

  it("hashset assigns the related row when select is omitted", async () => {
    const parents = new ListReadonlyRepository([parent(1, 10)]);
    const strategy = new HashsetOptimizedReadonlyEnrichmentStrategy<Parent>();
    parents.addEnriched({ into: "type", from: "type_id", by: "id" }, new ListReadonlyRepository([
        Object.assign(new TypeRow(), { id: 10, name: "person" }),
      ]), strategy);
    const items = await collectAsync(parents.read());
    expect(items[0]).toMatchObject({
      type: { id: 10, name: "person" },
    });
  });

  it("hashset skips missing keys and unmatched rows", async () => {
    const blank = Object.assign(parent(2, 0), { type_id: undefined });
    const parents = new ListReadonlyRepository([parent(1, 10), parent(3, 99), blank]);
    const strategy = new HashsetOptimizedReadonlyEnrichmentStrategy<Parent>();
    parents.addEnriched({ into: "type_name", from: "type_id", by: "id", select: "name" }, new ListReadonlyRepository([
        Object.assign(new TypeRow(), { id: 10, name: "person" }),
      ]), strategy);
    parents.addEnriched({ into: "children", from: "id", by: "parent_id", many: true }, new ListReadonlyRepository([
        Object.assign(new Child(), { label: "orphan" }),
        Object.assign(new Child(), { parent_id: 1, label: "a" }),
        Object.assign(new Child(), { parent_id: 1, label: "b" }),
      ]), strategy);
    const items = await collectAsync(parents.read());
    expect(items[0]?.type_name).toBe("person");
    expect(items[0]?.children.map((child) => child.label)).toEqual(["a", "b"]);
    expect(items[1]?.type_name).toBe("");
    expect(items[1]?.children).toEqual([]);
    expect(items[2]?.children).toEqual([]);
    expect(new RepositoryTreeNode({}, "").value).toEqual({});
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

  it("collapses join rows with Result", async () => {
    const hydrate = new TransformMapper(
      (row: Parent) => row,
      () => {
        throw new Error("hydrate does not reverse");
      },
    );
    const flat = new FlatToNestedMapper<Parent>({
      key: "id",
      fields: { id: "id", type_name: "type_name" },
    });
    const result = new TransformMapper(
      (rows: Iterable<Cells>) => flat.map(rows).map((row) => hydrate.map(row)),
      () => {
        throw new Error("result does not reverse");
      },
    );
    const repo = new JoinRows(
      [
        { id: 1, type_name: "person" },
        { id: 2, type_name: "org" },
      ],
      result,
    );
    const items = await collectAsync(repo.read());
    expect(items.map((item) => item.id)).toEqual([1, 2]);
    expect(items.map((item) => item.type_name)).toEqual(["person", "org"]);
  });

  it("lookup key comes from renamed row", async () => {
    const types = new ListReadonlyRepository([
      Object.assign(new TypeRow(), { id: 1, name: "admin" }),
    ]);
    const repo = new SourcedView();
    repo.addEnriched({ into: "UserType", from: "UserType", by: "id", select: "name" }, types);
    const item = await repo.readSingle();
    expect(item?.UserType).toBe("admin");
  });

  it("public include uses the node binder", async () => {
    const parents = new ListReadonlyRepository([parent(1, 10)]);
    const types = new ListReadonlyRepository([
      Object.assign(new TypeRow(), { id: 10, name: "person" }),
    ]);
    parents.addEnriched({ into: "type_name", from: "type_id", by: "id", select: "name" }, types);
    const items = await collectAsync(parents.read());
    expect(items[0]?.type_name).toBe("person");
    await new DefaultReadonlyEnrichmentStrategy<Parent>().apply(
      types,
      { into: "type_name", from: "type_id", by: "id", select: "name" },
      items,
    );
  });

  it("readTracked uses tracked when present", async () => {
    const { readTracked } = await import("./ReadonlyRepository.ts");
    const repo = new ListReadonlyRepository([parent(1, 0)]);
    const items = await collectAsync(readTracked(repo));
    expect(items.map((row) => row.item.id)).toEqual([1]);
  });

  it("readTracked falls back to read", async () => {
    const { readTracked } = await import("./ReadonlyRepository.ts");
    const items = await collectAsync(
      readTracked({
        type: Parent,
        readSingle: async () => undefined,
        async *read() {
          yield parent(1, 0);
        },
      }),
    );
    expect(items.map((row) => row.item.id)).toEqual([1]);
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
    readSingle: async (query?: ISelectStatement | IWhere | IExecuteStoredProcedureStatement, parameters?: Parameter[]) => {
      state.readSingles += 1;
      return inner.readSingle(query, parameters);
    },
    async *read(query?: ISelectStatement | IWhere | IExecuteStoredProcedureStatement, parameters?: Parameter[]) {
      state.reads += 1;
      yield* inner.read(query, parameters);
    },
  };
};
