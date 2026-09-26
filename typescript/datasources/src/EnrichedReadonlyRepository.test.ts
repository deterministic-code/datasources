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

type Seed = Record<string, Scalar>;

const setup = async (rows: Seed[] = []): Promise<IConnection> => {
  const connection = new MemoryProvider().connection();
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

const remove = (connection: IConnection, row: Seed) =>
  connection.createCommand().execute("delete", [{ name: "id", value: row.id }]);

describe("ReadonlyRepository", () => {
  it("reads one row through a where clause", async () => {
    const first: Seed = {};
    const second: Seed = {};
    const connection = await setup([first, second]);
    const parentsRepo = new ReadonlyRepository(Parent, connection);
    const byClause = await collectAsync(parentsRepo.read(new Where(new Operator.Equals("id", first.id))));
    const byWhere = await collectAsync(parentsRepo.read(new WhereText(JSON.stringify({ id: first.id }))));
    expect(byClause.map((item) => item.id)).toEqual([first.id]);
    expect(byWhere.map((item) => item.id)).toEqual([first.id]);
  });

  it("fills lookup select and leaves missing default", async () => {
    const person: Seed = { name: "person" };
    const gone: Seed = { name: "gone" };
    const typesConnection = await setup([person, gone]);
    await remove(typesConnection, gone);
    const connection = await setup([{ type_id: person.id }, { type_id: gone.id }]);
    const parentsRepo = new ReadonlyRepository(Parent, connection);
    const typesRepo = new ReadonlyRepository(TypeRow, typesConnection);
    parentsRepo.addEnriched({ into: "type_name", from: "type_id", by: "id", select: "name" }, typesRepo);
    const items = await collectAsync(parentsRepo.read());
    expect(items[0]?.type_name).toBe("person");
    expect(items[1]?.type_name).toBe("");
  });

  it("hashset fills lookup select and leaves missing default", async () => {
    const person: Seed = { name: "person" };
    const gone: Seed = { name: "gone" };
    const typesConnection = await setup([person, gone]);
    await remove(typesConnection, gone);
    const connection = await setup([{ type_id: person.id }, { type_id: gone.id }]);
    const parentsRepo = new ReadonlyRepository(Parent, connection);
    const typesRepo = new ReadonlyRepository(TypeRow, typesConnection);
    parentsRepo.addEnriched(
      { into: "type_name", from: "type_id", by: "id", select: "name" },
      typesRepo,
      new HashsetOptimizedReadonlyEnrichmentStrategy(),
    );
    const items = await collectAsync(parentsRepo.read());
    expect(items[0]?.type_name).toBe("person");
    expect(items[1]?.type_name).toBe("");
  });

  it("fills lists and readSingle is enriched", async () => {
    const org: Seed = { name: "org" };
    const typesConnection = await setup([org]);
    const parent: Seed = { type_id: org.id };
    const other: Seed = { type_id: org.id };
    const connection = await setup([parent, other]);
    await remove(connection, other);
    const parentsRepo = new ReadonlyRepository(Parent, connection);
    const typesRepo = new ReadonlyRepository(TypeRow, typesConnection);
    const childrenConnection = await setup([
      { parent_id: parent.id, label: "a" },
      { parent_id: parent.id, label: "b" },
      { parent_id: other.id, label: "x" },
    ]);
    const childrenRepo = new ReadonlyRepository(Child, childrenConnection);
    parentsRepo.addEnriched({ into: "type_name", from: "type_id", by: "id", select: "name" }, typesRepo);
    parentsRepo.addEnriched({ into: "children", from: "id", by: "parent_id", many: true }, childrenRepo);
    const item = await parentsRepo.readSingle();
    expect(item?.type_name).toBe("org");
    expect(item?.children.map((child) => child.label)).toEqual(["a", "b"]);
  });

  it("hashset fills lists and readSingle is enriched", async () => {
    const org: Seed = { name: "org" };
    const typesConnection = await setup([org]);
    const parent: Seed = { type_id: org.id };
    const other: Seed = { type_id: org.id };
    const connection = await setup([parent, other]);
    await remove(connection, other);
    const parentsRepo = new ReadonlyRepository(Parent, connection);
    const typesRepo = new ReadonlyRepository(TypeRow, typesConnection);
    const childrenConnection = await setup([
      { parent_id: parent.id, label: "a" },
      { parent_id: parent.id, label: "b" },
      { parent_id: other.id, label: "x" },
    ]);
    const childrenRepo = new ReadonlyRepository(Child, childrenConnection);
    const strategy = new HashsetOptimizedReadonlyEnrichmentStrategy<Parent>();
    parentsRepo.addEnriched({ into: "type_name", from: "type_id", by: "id", select: "name" }, typesRepo, strategy);
    parentsRepo.addEnriched({ into: "children", from: "id", by: "parent_id", many: true }, childrenRepo, strategy);
    const item = await parentsRepo.readSingle();
    expect(item?.type_name).toBe("org");
    expect(item?.children.map((child) => child.label)).toEqual(["a", "b"]);
  });

  it("nests list lookup and keeps root sibling", async () => {
    const org: Seed = { name: "org" };
    const typesConnection = await setup([org]);
    const home: Seed = { name: "home" };
    const gone: Seed = { name: "gone" };
    const childTypesConnection = await setup([home, gone]);
    await remove(childTypesConnection, gone);
    const parent: Seed = { type_id: org.id };
    const connection = await setup([parent]);
    const parentsRepo = new ReadonlyRepository(Parent, connection);
    const typesRepo = new ReadonlyRepository(TypeRow, typesConnection);
    const childTypesRepo = new ReadonlyRepository(TypeRow, childTypesConnection);
    const childrenConnection = await setup([
      { parent_id: parent.id, type_id: home.id, label: "a" },
      { parent_id: parent.id, type_id: gone.id, label: "b" },
    ]);
    const childrenRepo = new ReadonlyRepository(Child, childrenConnection);
    childrenRepo.addEnriched({ into: "type_name", from: "type_id", by: "id", select: "name" }, childTypesRepo);
    parentsRepo.addEnriched({ into: "type_name", from: "type_id", by: "id", select: "name" }, typesRepo);
    parentsRepo.addEnriched({ into: "children", from: "id", by: "parent_id", many: true }, childrenRepo);
    const item = await parentsRepo.readSingle();
    expect(item?.type_name).toBe("org");
    expect(item?.children[0]?.type_name).toBe("home");
    expect(item?.children[1]?.type_name).toBe("");
  });

  it("hashset nests list lookup and keeps root sibling", async () => {
    const org: Seed = { name: "org" };
    const typesConnection = await setup([org]);
    const home: Seed = { name: "home" };
    const gone: Seed = { name: "gone" };
    const childTypesConnection = await setup([home, gone]);
    await remove(childTypesConnection, gone);
    const parent: Seed = { type_id: org.id };
    const connection = await setup([parent]);
    const parentsRepo = new ReadonlyRepository(Parent, connection);
    const typesRepo = new ReadonlyRepository(TypeRow, typesConnection);
    const childTypesRepo = new ReadonlyRepository(TypeRow, childTypesConnection);
    const childrenConnection = await setup([
      { parent_id: parent.id, type_id: home.id, label: "a" },
      { parent_id: parent.id, type_id: gone.id, label: "b" },
    ]);
    const childrenRepo = new ReadonlyRepository(Child, childrenConnection);
    const strategy = new HashsetOptimizedReadonlyEnrichmentStrategy<Parent>();
    childrenRepo.addEnriched({ into: "type_name", from: "type_id", by: "id", select: "name" }, childTypesRepo);
    parentsRepo.addEnriched({ into: "type_name", from: "type_id", by: "id", select: "name" }, typesRepo, strategy);
    parentsRepo.addEnriched({ into: "children", from: "id", by: "parent_id", many: true }, childrenRepo, strategy);
    const item = await parentsRepo.readSingle();
    expect(item?.type_name).toBe("org");
    expect(item?.children[0]?.type_name).toBe("home");
    expect(item?.children[1]?.type_name).toBe("");
  });

  it("skips missing enrichment keys", async () => {
    const connection = await setup([{ type_id: null, type_name: "" }, { type_name: "" }]);
    const parentsRepo = new ReadonlyRepository(Sparse, connection);
    const typesRepo = new ReadonlyRepository(TypeRow, await setup([{ name: "person" }]));
    const childrenRepo = new ReadonlyRepository(Child, await setup([{ label: "a" }]));
    parentsRepo.addEnriched({ into: "type_name", from: "type_id", by: "id", select: "name" }, typesRepo);
    parentsRepo.addEnriched({ into: "children", from: "id", by: "parent_id", many: true }, childrenRepo);
    const items = await collectAsync(parentsRepo.read());
    expect(items[0]?.type_name).toBe("");
    expect(items[1]?.children).toBeUndefined();
  });

  it("readSingle is undefined when empty", async () => {
    const parentsRepo = new ReadonlyRepository(Parent, await setup());
    expect(await parentsRepo.readSingle()).toBeUndefined();
  });

  it("nests on a singular related object and skips strings", async () => {
    const kept: Seed = { child: "skip" };
    const nested: Seed = {};
    const connection = await setup([kept, nested]);
    const parentsRepo = new ReadonlyRepository(Parent, connection);
    const home: Seed = { name: "home" };
    const childTypesRepo = new ReadonlyRepository(TypeRow, await setup([home]));
    const childrenRepo = new ReadonlyRepository(Child, await setup([
      { parent_id: nested.id, type_id: home.id, label: "only" },
    ]));
    childrenRepo.addEnriched({ into: "type_name", from: "type_id", by: "id", select: "name" }, childTypesRepo);
    parentsRepo.addEnriched({ into: "child", from: "id", by: "parent_id" }, childrenRepo);
    const items = await collectAsync(parentsRepo.read());
    expect(items[0]?.child).toBe("skip");
    expect(items[1]?.child && typeof items[1].child === "object" ? items[1].child.type_name : undefined).toBe("home");
  });

  it("throws when an enrichment key is an object", async () => {
    const typesRepo = new ReadonlyRepository(TypeRow, await setup());
    await expect(
      new DefaultReadonlyEnrichmentStrategy<Parent>().apply(
        typesRepo,
        { into: "type_name", from: "type_id", by: "id", select: "name" },
        [Object.assign(new Parent(), { id: 1, type_id: { x: 1 } })],
      ),
    ).rejects.toThrow(/scalar/);
  });

  it("hashset assigns the related row when select is omitted", async () => {
    const person: Seed = { name: "person" };
    const typesConnection = await setup([person]);
    const connection = await setup([{ type_id: person.id }]);
    const parentsRepo = new ReadonlyRepository(Parent, connection);
    const typesRepo = new ReadonlyRepository(TypeRow, typesConnection);
    parentsRepo.addEnriched(
      { into: "type", from: "type_id", by: "id" },
      typesRepo,
      new HashsetOptimizedReadonlyEnrichmentStrategy(),
    );
    const items = await collectAsync(parentsRepo.read());
    expect(items[0]?.type).toMatchObject({ name: "person" });
  });

  it("hashset skips missing keys and unmatched rows", async () => {
    const person: Seed = { name: "person" };
    const typesConnection = await setup([person]);
    const gone: Seed = { name: "gone" };
    await setup([gone]);
    const matched: Seed = { type_id: person.id };
    const missing: Seed = { type_id: gone.id };
    const blank: Seed = {};
    const connection = await setup([matched, missing, blank]);
    const parentsRepo = new ReadonlyRepository(OptionalType, connection);
    const typesRepo = new ReadonlyRepository(TypeRow, typesConnection);
    const childrenConnection = await setup([
      { label: "orphan" },
      { parent_id: matched.id, label: "a" },
      { parent_id: matched.id, label: "b" },
    ]);
    const childrenRepo = new ReadonlyRepository(Child, childrenConnection);
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

  it("collapses join rows", async () => {
    const person: Seed = { type_name: "person" };
    const org: Seed = { type_name: "org" };
    const connection = await setup([person, org]);
    const parents: object[] = [];
    const parentsRepo = new ReadonlyRepository(Parent, connection);
    await new NestedToObjectEnrichmentStrategy<Parent>({
      key: "id",
      fields: { id: "id", type_name: "type_name" },
    }).apply(parentsRepo, { into: "type_name", from: "id", by: "id" }, parents, [person, org]);
    const items = parents as Parent[];
    expect(items.map((item) => item.id)).toEqual([person.id, org.id]);
    expect(items.map((item) => item.type_name)).toEqual(["person", "org"]);
  });

  it("lookup key comes from renamed row", async () => {
    const admin: Seed = { name: "admin" };
    const typesConnection = await setup([admin]);
    const typesRepo = new ReadonlyRepository(TypeRow, typesConnection);
    const usersConnection = await setup([{ user_type_id: admin.id }]);
    const usersRepo = new ReadonlyRepository(usersConnection, {
      type: View,
      columnMappings: new DictionaryMapper([["UserType", "user_type_id"]]),
    });
    usersRepo.addEnriched({ into: "UserType", from: "UserType", by: "id", select: "name" }, typesRepo);
    const item = await usersRepo.readSingle();
    expect(item?.UserType).toBe("admin");
  });

  it("public include uses the node binder", async () => {
    const person: Seed = { name: "person" };
    const typesConnection = await setup([person]);
    const connection = await setup([{ type_id: person.id }]);
    const parentsRepo = new ReadonlyRepository(Parent, connection);
    const typesRepo = new ReadonlyRepository(TypeRow, typesConnection);
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
