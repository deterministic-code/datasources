import { describe, expect, it } from "vitest";
import { MemoryProvider } from "@datasources/memory";
import { collectAsync } from "./collectAsync.ts";
import { DefaultReadonlyEnrichmentStrategy } from "./DefaultReadonlyEnrichmentStrategy.ts";
import { NestedToObjectEnrichmentStrategy } from "./NestedToObjectEnrichmentStrategy.ts";
import { HashsetOptimizedReadonlyEnrichmentStrategy } from "./HashsetOptimizedReadonlyEnrichmentStrategy.ts";
import { ReadonlyRepository } from "./ReadonlyRepository.ts";
import { RepositoryTreeNode } from "./RepositoryTreeNode.ts";
import { DictionaryMapper } from "./mappers/DictionaryMapper.ts";
import type { IConnection } from "./IConnection.ts";
import type { IReadonlyEnrichmentStrategy } from "./IReadonlyEnrichmentStrategy.ts";
import type { Scalar } from "./builders/Scalar.ts";
import { Operator, Where } from "./statements/index.ts";
import { WhereText } from "./statements/WhereText.ts";

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

class Sparse {
  type_id: number | null = null;
  type_name = "";
  id?: number;
  children?: Child[];
}

class OptionalType {
  id = 0;
  type_id?: number;
  type_name = "";
  children: Child[] = [];
}

const open = async (rows: Record<string, Scalar>[] = []): Promise<IConnection> => {
  const connection = new MemoryProvider().connection();
  await connection.open();
  for (const row of rows)
    await connection.createCommand().execute(
      "insert",
      Object.entries(row).map(([name, value]) => ({ name, value })),
    );
  return connection;
};

const fillLookup = async (strategy: IReadonlyEnrichmentStrategy<Parent>) => {
  const parentsRepo = new ReadonlyRepository(Parent, await open([
    { id: 1, type_id: 10 },
    { id: 2, type_id: 99 },
  ]));
  const typesRepo = new ReadonlyRepository(TypeRow, await open([{ id: 10, name: "person" }]));
  parentsRepo.addEnriched({ into: "type_name", from: "type_id", by: "id", select: "name" }, typesRepo, strategy);
  const items = await collectAsync(parentsRepo.read());
  expect(items[0]?.type_name).toBe("person");
  expect(items[1]?.type_name).toBe("");
};

const fillLists = async (strategy: IReadonlyEnrichmentStrategy<Parent>) => {
  const parentsRepo = new ReadonlyRepository(Parent, await open([{ id: 1, type_id: 10 }]));
  const typesRepo = new ReadonlyRepository(TypeRow, await open([{ id: 10, name: "org" }]));
  const childrenRepo = new ReadonlyRepository(Child, await open([
    { parent_id: 1, label: "a" },
    { parent_id: 1, label: "b" },
    { parent_id: 2, label: "x" },
  ]));
  parentsRepo.addEnriched({ into: "type_name", from: "type_id", by: "id", select: "name" }, typesRepo, strategy);
  parentsRepo.addEnriched({ into: "children", from: "id", by: "parent_id", many: true }, childrenRepo, strategy);
  const item = await parentsRepo.readSingle();
  expect(item?.type_name).toBe("org");
  expect(item?.children.map((child) => child.label)).toEqual(["a", "b"]);
};

const nestListLookup = async (strategy: IReadonlyEnrichmentStrategy<Parent>) => {
  const parentsRepo = new ReadonlyRepository(Parent, await open([{ id: 1, type_id: 10 }]));
  const typesRepo = new ReadonlyRepository(TypeRow, await open([{ id: 10, name: "org" }]));
  const childTypesRepo = new ReadonlyRepository(TypeRow, await open([{ id: 3, name: "home" }]));
  const childrenRepo = new ReadonlyRepository(Child, await open([
    { parent_id: 1, type_id: 3, label: "a" },
    { parent_id: 1, type_id: 9, label: "b" },
  ]));
  childrenRepo.addEnriched({ into: "type_name", from: "type_id", by: "id", select: "name" }, childTypesRepo);
  parentsRepo.addEnriched({ into: "type_name", from: "type_id", by: "id", select: "name" }, typesRepo, strategy);
  parentsRepo.addEnriched({ into: "children", from: "id", by: "parent_id", many: true }, childrenRepo, strategy);
  const item = await parentsRepo.readSingle();
  expect(item?.type_name).toBe("org");
  expect(item?.children[0]?.type_name).toBe("home");
  expect(item?.children[1]?.type_name).toBe("");
};

describe("ReadonlyRepository", () => {
  it("reads one row through a where clause", async () => {
    const parentsRepo = new ReadonlyRepository(Parent, await open([{ id: 1 }]));
    const byClause = await collectAsync(parentsRepo.read(new Where(new Operator.Equals("id", 1))));
    const byWhere = await collectAsync(parentsRepo.read(new WhereText("id = 1")));
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
    const parentsRepo = new ReadonlyRepository(Sparse, await open([
      { type_id: null, type_name: "" },
      { type_name: "" },
    ]));
    const typesRepo = new ReadonlyRepository(TypeRow, await open([{ id: 10, name: "person" }]));
    const childrenRepo = new ReadonlyRepository(Child, await open([{ parent_id: 1, label: "a" }]));
    parentsRepo.addEnriched({ into: "type_name", from: "type_id", by: "id", select: "name" }, typesRepo);
    parentsRepo.addEnriched({ into: "children", from: "id", by: "parent_id", many: true }, childrenRepo);
    const items = await collectAsync(parentsRepo.read());
    expect(items[0]?.type_name).toBe("");
    expect(items[1]?.children).toBeUndefined();
  });

  it("readSingle is undefined when empty", async () => {
    const parentsRepo = new ReadonlyRepository(Parent, await open());
    expect(await parentsRepo.readSingle()).toBeUndefined();
  });

  it("nests on a singular related object and skips strings", async () => {
    const parentsRepo = new ReadonlyRepository(Parent, await open([
      { id: 1, type_id: 10, child: "skip" },
      { id: 2, type_id: 10 },
    ]));
    const childTypesRepo = new ReadonlyRepository(TypeRow, await open([{ id: 3, name: "home" }]));
    const childrenRepo = new ReadonlyRepository(Child, await open([
      { parent_id: 2, type_id: 3, label: "only" },
    ]));
    childrenRepo.addEnriched({ into: "type_name", from: "type_id", by: "id", select: "name" }, childTypesRepo);
    parentsRepo.addEnriched({ into: "child", from: "id", by: "parent_id" }, childrenRepo);
    const items = await collectAsync(parentsRepo.read());
    expect(items[0]?.child).toBe("skip");
    expect(
      items[1]?.child && typeof items[1].child === "object"
        ? items[1].child.type_name
        : undefined,
    ).toBe("home");
  });

  it("throws when an enrichment key is an object", async () => {
    const typesRepo = new ReadonlyRepository(TypeRow, await open());
    await expect(
      new DefaultReadonlyEnrichmentStrategy<Parent>().apply(
        typesRepo,
        { into: "type_name", from: "type_id", by: "id", select: "name" },
        [Object.assign(new Parent(), { id: 1, type_id: { x: 1 } })],
      ),
    ).rejects.toThrow(/scalar/);
  });

  it("hashset assigns the related row when select is omitted", async () => {
    const parentsRepo = new ReadonlyRepository(Parent, await open([{ id: 1, type_id: 10 }]));
    const typesRepo = new ReadonlyRepository(TypeRow, await open([{ id: 10, name: "person" }]));
    const strategy = new HashsetOptimizedReadonlyEnrichmentStrategy<Parent>();
    parentsRepo.addEnriched({ into: "type", from: "type_id", by: "id" }, typesRepo, strategy);
    const items = await collectAsync(parentsRepo.read());
    expect(items[0]).toMatchObject({
      type: { id: 10, name: "person" },
    });
  });

  it("hashset skips missing keys and unmatched rows", async () => {
    const parentsRepo = new ReadonlyRepository(OptionalType, await open([
      { id: 1, type_id: 10 },
      { id: 3, type_id: 99 },
      { id: 2 },
    ]));
    const typesRepo = new ReadonlyRepository(TypeRow, await open([{ id: 10, name: "person" }]));
    const childrenRepo = new ReadonlyRepository(Child, await open([
      { label: "orphan" },
      { parent_id: 1, label: "a" },
      { parent_id: 1, label: "b" },
    ]));
    const strategy = new HashsetOptimizedReadonlyEnrichmentStrategy<OptionalType>();
    parentsRepo.addEnriched({ into: "type_name", from: "type_id", by: "id", select: "name" }, typesRepo, strategy);
    parentsRepo.addEnriched({ into: "children", from: "id", by: "parent_id", many: true }, childrenRepo, strategy);
    const items = await collectAsync(parentsRepo.read());
    expect(items[0]?.type_name).toBe("person");
    expect(items[0]?.children.map((child) => child.label)).toEqual(["a", "b"]);
    expect(items[1]?.type_name).toBe("");
    expect(items[1]?.children).toEqual([]);
    expect(items[2]?.children).toEqual([]);
    expect(new RepositoryTreeNode({}, "").value).toEqual({});
  });

  it("collapses join rows with Result", async () => {
    const parents: object[] = [];
    const parentsRepo = new ReadonlyRepository(Parent, new MemoryProvider().connection());
    await new NestedToObjectEnrichmentStrategy<Parent>({
      key: "id",
      fields: { id: "id", type_name: "type_name" },
    }).apply(
      parentsRepo,
      { into: "type_name", from: "id", by: "id" },
      parents,
      [
        { id: 1, type_name: "person" },
        { id: 2, type_name: "org" },
      ],
    );
    const items = parents as Parent[];
    expect(items.map((item) => item.id)).toEqual([1, 2]);
    expect(items.map((item) => item.type_name)).toEqual(["person", "org"]);
  });

  it("lookup key comes from renamed row", async () => {
    const typesRepo = new ReadonlyRepository(TypeRow, await open([{ id: 1, name: "admin" }]));
    const usersRepo = new ReadonlyRepository(await open([{ user_type_id: 1 }]), {
      type: View,
      columnMappings: new DictionaryMapper([["UserType", "user_type_id"]]),
    });
    usersRepo.addEnriched({ into: "UserType", from: "UserType", by: "id", select: "name" }, typesRepo);
    const item = await usersRepo.readSingle();
    expect(item?.UserType).toBe("admin");
  });

  it("public include uses the node binder", async () => {
    const parentsRepo = new ReadonlyRepository(Parent, await open([{ id: 1, type_id: 10 }]));
    const typesRepo = new ReadonlyRepository(TypeRow, await open([{ id: 10, name: "person" }]));
    parentsRepo.addEnriched({ into: "type_name", from: "type_id", by: "id", select: "name" }, typesRepo);
    const items = await collectAsync(parentsRepo.read());
    expect(items[0]?.type_name).toBe("person");
    await new DefaultReadonlyEnrichmentStrategy<Parent>().apply(
      typesRepo,
      { into: "type_name", from: "type_id", by: "id", select: "name" },
      items,
    );
  });
});
