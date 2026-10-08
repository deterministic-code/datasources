import { describe, expect, it } from "vitest";
import { MemoryDriver } from "@datasources/memory";
import { collectAsync } from "./collectAsync.ts";
import type { Cells } from "./converters/Cell.ts";
import { TypeConverter } from "./converters/TypeConverter.ts";
import { DefaultReadonlyEnrichmentStrategy } from "./DefaultReadonlyEnrichmentStrategy.ts";
import { NestedToObjectEnrichmentStrategy } from "./NestedToObjectEnrichmentStrategy.ts";
import { HashsetOptimizedReadonlyEnrichmentStrategy } from "./HashsetOptimizedReadonlyEnrichmentStrategy.ts";
import { ReadonlyRepository } from "./ReadonlyRepository.ts";
import { RepositoryTreeNode } from "./RepositoryTreeNode.ts";
import { ChangeCasingMapper } from "./mappers/ChangeCasingMapper.ts";
import { DictionaryMapper } from "./mappers/DictionaryMapper.ts";
import type { IMapper } from "./mappers/IMapper.ts";
import { TransformMapper } from "./mappers/TransformMapper.ts";
import type { IConnection } from "./IConnection.ts";
import type { Scalar } from "./builders/Scalar.ts";
import { MockCommand } from "./mocks/mock-command.ts";
import { MockConnection } from "./mocks/mock-connection.ts";
import { MockReader } from "./mocks/mock-reader.ts";
import { Join, Operator, SelectStatement, Where } from "./statements/index.ts";
import { WhereText } from "./statements/WhereText.ts";

class UserType {
  Id = 0;
  Name = "";
}

class Child {
  Id = 0;
  ParentId = 0;
  Type = new UserType();
  TypeName = "";
  Label = "";
}

class Parent {
  Id = 0;
  Type = new UserType();
  TypeName = "";
  Children: Child[] = [];
  Child?: Child | string;
}

class View {
  UserType = "";
}

type Seed = Record<string, Scalar>;

const snake = new ChangeCasingMapper("pascalCase", "snakeCase");
const references = new DictionaryMapper<string>([["Type", "type_id"]]);
const columns: IMapper<string, string> = new TransformMapper(
  (name) => {
    const mapped = references.map(name);
    return mapped !== name ? mapped : snake.map(name);
  },
  (name) => {
    const from = references.reverseMap(name);
    return from !== name ? from : snake.reverseMap(name);
  },
);

const repo = <T>(type: new () => T, connection: IConnection) =>
  new ReadonlyRepository(connection, { type, propertyMappings: columns });

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

const remove = (connection: IConnection, row: Seed) =>
  connection.createCommand().execute("delete", [{ name: "id", value: row.id }]);

describe("ReadonlyRepository", () => {
  it("reads one row through a where clause", async () => {
    const first: Seed = {};
    const second: Seed = {};
    const connection = await setup([first, second]);
    const parentsRepo = repo(Parent, connection);
    const byClause = await collectAsync(parentsRepo.read(new Where(new Operator.Equals("Id", first.id))));
    const byWhere = await collectAsync(parentsRepo.read(new WhereText(JSON.stringify({ id: first.id }))));
    expect(byClause.map((item) => item.Id)).toEqual([first.id]);
    expect(byWhere.map((item) => item.Id)).toEqual([first.id]);
  });

  it("fills lookup select and leaves missing default", async () => {
    const person: Seed = { name: "person" };
    const gone: Seed = { name: "gone" };
    const typesConnection = await setup([person, gone]);
    await remove(typesConnection, gone);
    const connection = await setup([{ type_id: person.id }, { type_id: gone.id }]);
    const parentsRepo = repo(Parent, connection);
    const typesRepo = repo(UserType, typesConnection);
    parentsRepo.addEnriched({ destinationProperty: "TypeName", sourceProperty: "Type", relatedKey: "Id", relatedProperty: "Name" }, typesRepo);
    const items = await collectAsync(parentsRepo.read());
    expect(items[0]?.TypeName).toBe("person");
    expect(items[1]?.TypeName).toBe("");
  });

  it("hashset fills lookup select and leaves missing default", async () => {
    const person: Seed = { name: "person" };
    const gone: Seed = { name: "gone" };
    const typesConnection = await setup([person, gone]);
    await remove(typesConnection, gone);
    const connection = await setup([{ type_id: person.id }, { type_id: gone.id }]);
    const parentsRepo = repo(Parent, connection);
    const typesRepo = repo(UserType, typesConnection);
    parentsRepo.addEnriched(
      { destinationProperty: "TypeName", sourceProperty: "Type", relatedKey: "Id", relatedProperty: "Name" },
      typesRepo,
      new HashsetOptimizedReadonlyEnrichmentStrategy(),
    );
    const items = await collectAsync(parentsRepo.read());
    expect(items[0]?.TypeName).toBe("person");
    expect(items[1]?.TypeName).toBe("");
  });

  it("fills lists and readSingle is enriched", async () => {
    const org: Seed = { name: "org" };
    const typesConnection = await setup([org]);
    const parent: Seed = { type_id: org.id };
    const other: Seed = { type_id: org.id };
    const connection = await setup([parent, other]);
    await remove(connection, other);
    const parentsRepo = repo(Parent, connection);
    const typesRepo = repo(UserType, typesConnection);
    const childrenConnection = await setup([
      { parent_id: parent.id, label: "a" },
      { parent_id: parent.id, label: "b" },
      { parent_id: other.id, label: "x" },
    ]);
    const childrenRepo = repo(Child, childrenConnection);
    parentsRepo.addEnriched({ destinationProperty: "TypeName", sourceProperty: "Type", relatedKey: "Id", relatedProperty: "Name" }, typesRepo);
    parentsRepo.addEnriched({ destinationProperty: "Children", sourceProperty: "Id", relatedKey: "ParentId", type: "list" }, childrenRepo);
    const item = await parentsRepo.readSingle();
    expect(item?.TypeName).toBe("org");
    expect(item?.Children.map((child) => child.Label)).toEqual(["a", "b"]);
  });

  it("hashset fills lists and readSingle is enriched", async () => {
    const org: Seed = { name: "org" };
    const typesConnection = await setup([org]);
    const parent: Seed = { type_id: org.id };
    const other: Seed = { type_id: org.id };
    const connection = await setup([parent, other]);
    await remove(connection, other);
    const parentsRepo = repo(Parent, connection);
    const typesRepo = repo(UserType, typesConnection);
    const childrenConnection = await setup([
      { parent_id: parent.id, label: "a" },
      { parent_id: parent.id, label: "b" },
      { parent_id: other.id, label: "x" },
    ]);
    const childrenRepo = repo(Child, childrenConnection);
    const strategy = new HashsetOptimizedReadonlyEnrichmentStrategy<Parent>();
    parentsRepo.addEnriched({ destinationProperty: "TypeName", sourceProperty: "Type", relatedKey: "Id", relatedProperty: "Name" }, typesRepo, strategy);
    parentsRepo.addEnriched({ destinationProperty: "Children", sourceProperty: "Id", relatedKey: "ParentId", type: "list" }, childrenRepo, strategy);
    const item = await parentsRepo.readSingle();
    expect(item?.TypeName).toBe("org");
    expect(item?.Children.map((child) => child.Label)).toEqual(["a", "b"]);
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
    const parentsRepo = repo(Parent, connection);
    const typesRepo = repo(UserType, typesConnection);
    const childTypesRepo = repo(UserType, childTypesConnection);
    const childrenConnection = await setup([
      { parent_id: parent.id, type_id: home.id, label: "a" },
      { parent_id: parent.id, type_id: gone.id, label: "b" },
    ]);
    const childrenRepo = repo(Child, childrenConnection);
    childrenRepo.addEnriched({ destinationProperty: "TypeName", sourceProperty: "Type", relatedKey: "Id", relatedProperty: "Name" }, childTypesRepo);
    parentsRepo.addEnriched({ destinationProperty: "TypeName", sourceProperty: "Type", relatedKey: "Id", relatedProperty: "Name" }, typesRepo);
    parentsRepo.addEnriched({ destinationProperty: "Children", sourceProperty: "Id", relatedKey: "ParentId", type: "list" }, childrenRepo);
    const item = await parentsRepo.readSingle();
    expect(item?.TypeName).toBe("org");
    expect(item?.Children[0]?.TypeName).toBe("home");
    expect(item?.Children[1]?.TypeName).toBe("");
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
    const parentsRepo = repo(Parent, connection);
    const typesRepo = repo(UserType, typesConnection);
    const childTypesRepo = repo(UserType, childTypesConnection);
    const childrenConnection = await setup([
      { parent_id: parent.id, type_id: home.id, label: "a" },
      { parent_id: parent.id, type_id: gone.id, label: "b" },
    ]);
    const childrenRepo = repo(Child, childrenConnection);
    const strategy = new HashsetOptimizedReadonlyEnrichmentStrategy<Parent>();
    childrenRepo.addEnriched({ destinationProperty: "TypeName", sourceProperty: "Type", relatedKey: "Id", relatedProperty: "Name" }, childTypesRepo);
    parentsRepo.addEnriched({ destinationProperty: "TypeName", sourceProperty: "Type", relatedKey: "Id", relatedProperty: "Name" }, typesRepo, strategy);
    parentsRepo.addEnriched({ destinationProperty: "Children", sourceProperty: "Id", relatedKey: "ParentId", type: "list" }, childrenRepo, strategy);
    const item = await parentsRepo.readSingle();
    expect(item?.TypeName).toBe("org");
    expect(item?.Children[0]?.TypeName).toBe("home");
    expect(item?.Children[1]?.TypeName).toBe("");
  });

  it("skips missing enrichment keys", async () => {
    const connection = await setup([{ type_id: -1 }, { type_id: -1 }]);
    const parentsRepo = repo(Parent, connection);
    const typesRepo = repo(UserType, await setup([{ name: "person" }]));
    const childrenRepo = repo(Child, await setup([{ label: "a" }]));
    parentsRepo.addEnriched({ destinationProperty: "TypeName", sourceProperty: "Type", relatedKey: "Id", relatedProperty: "Name" }, typesRepo);
    parentsRepo.addEnriched({ destinationProperty: "Children", sourceProperty: "Id", relatedKey: "ParentId", type: "list" }, childrenRepo);
    const items = await collectAsync(parentsRepo.read());
    expect(items[0]?.TypeName).toBe("");
    expect(items[1]?.TypeName).toBe("");
    expect(items[1]?.Children).toEqual([]);
  });

  it("readSingle is undefined when empty", async () => {
    const parentsRepo = repo(Parent, await setup());
    expect(await parentsRepo.readSingle()).toBeUndefined();
  });

  it("nests on a singular related object and skips strings", async () => {
    const kept: Seed = { child: "skip" };
    const nested: Seed = {};
    const connection = await setup([kept, nested]);
    const parentsRepo = repo(Parent, connection);
    const home: Seed = { name: "home" };
    const childTypesRepo = repo(UserType, await setup([home]));
    const childrenRepo = repo(Child, await setup([
      { parent_id: nested.id, type_id: home.id, label: "only" },
    ]));
    childrenRepo.addEnriched({ destinationProperty: "TypeName", sourceProperty: "Type", relatedKey: "Id", relatedProperty: "Name" }, childTypesRepo);
    parentsRepo.addEnriched({ destinationProperty: "Child", sourceProperty: "Id", relatedKey: "ParentId" }, childrenRepo);
    const items = await collectAsync(parentsRepo.read());
    expect(items[0]?.Child).toBe("skip");
    expect(items[1]?.Child && typeof items[1].Child === "object" ? items[1].Child.TypeName : undefined).toBe("home");
  });

  it("throws when an enrichment key is an object", async () => {
    const typesRepo = repo(UserType, await setup());
    await expect(
      new DefaultReadonlyEnrichmentStrategy<Parent>().apply(
        typesRepo,
        { destinationProperty: "TypeName", sourceProperty: "Type", relatedKey: "Id", relatedProperty: "Name" },
        [Object.assign(new Parent(), { Id: 1, Type: { Name: "x" } })],
      ),
    ).rejects.toThrow(/scalar/);
  });

  it("hashset assigns the related row when select is omitted", async () => {
    const person: Seed = { name: "person" };
    const typesConnection = await setup([person]);
    const connection = await setup([{ type_id: person.id }]);
    const parentsRepo = repo(Parent, connection);
    const typesRepo = repo(UserType, typesConnection);
    parentsRepo.addEnriched(
      { destinationProperty: "Type", sourceProperty: "Type", relatedKey: "Id" },
      typesRepo,
      new HashsetOptimizedReadonlyEnrichmentStrategy(),
    );
    const items = await collectAsync(parentsRepo.read());
    expect(items[0]?.Type).toMatchObject({ Name: "person" });
  });

  it("hashset skips missing keys and unmatched rows", async () => {
    const person: Seed = { name: "person" };
    const typesConnection = await setup([person]);
    const matched: Seed = { type_id: person.id };
    const missing: Seed = { type_id: Number(person.id) + 1 };
    const blank: Seed = { type_id: -1 };
    const connection = await setup([matched, missing, blank]);
    const parentsRepo = repo(Parent, connection);
    const typesRepo = repo(UserType, typesConnection);
    const childrenConnection = await setup([
      { label: "orphan" },
      { parent_id: matched.id, label: "a" },
      { parent_id: matched.id, label: "b" },
    ]);
    const childrenRepo = repo(Child, childrenConnection);
    const strategy = new HashsetOptimizedReadonlyEnrichmentStrategy<Parent>();
    parentsRepo.addEnriched({ destinationProperty: "TypeName", sourceProperty: "Type", relatedKey: "Id", relatedProperty: "Name" }, typesRepo, strategy);
    parentsRepo.addEnriched({ destinationProperty: "Children", sourceProperty: "Id", relatedKey: "ParentId", type: "list" }, childrenRepo, strategy);
    const items = await collectAsync(parentsRepo.read());
    expect(items[0]?.TypeName).toBe("person");
    expect(items[0]?.Children.map((child) => child.Label)).toEqual(["a", "b"]);
    expect(items[1]?.TypeName).toBe("");
    expect(items[1]?.Children).toEqual([]);
    expect(items[2]?.Children).toEqual([]);
    expect(new RepositoryTreeNode({}, "").value).toEqual({});
  });

  it("collapses join rows", async () => {
    const person: Seed = { TypeName: "person" };
    const org: Seed = { TypeName: "org" };
    const connection = await setup([person, org]);
    person.Id = person.id;
    org.Id = org.id;
    const parents: Parent[] = [];
    const parentsRepo = repo(Parent, connection);
    await new NestedToObjectEnrichmentStrategy<Parent>({
      key: "Id",
      fields: { Id: "Id", TypeName: "TypeName" },
    }).apply(parentsRepo, { destinationProperty: "TypeName", sourceProperty: "Id", relatedKey: "Id" }, parents, [person, org]);
    const items = parents;
    expect(items.map((item) => item.Id)).toEqual([person.id, org.id]);
    expect(items.map((item) => item.TypeName)).toEqual(["person", "org"]);
  });

    it("lookup key comes from renamed row", async () => {
    const admin: Seed = { name: "admin" };
    const typesConnection = await setup([admin]);
    const typesRepo = repo(UserType, typesConnection);
    const usersConnection = await setup([{ user_type_id: admin.id }]);
    const usersRepo = new ReadonlyRepository(usersConnection, {
      type: View,
      propertyMappings: new DictionaryMapper([["UserType", "user_type_id"]]),
    });
    usersRepo.addEnriched({ destinationProperty: "UserType", sourceProperty: "UserType", relatedKey: "Id", relatedProperty: "Name" }, typesRepo);
    const item = await usersRepo.readSingle();
    expect(item?.UserType).toBe("admin");
  });

  it("uses per-call object and row converters when provided", async () => {
    class Item {
      Label = "";
    }
    class FixedLabel extends TypeConverter<Item, Cells> {
      constructor(private readonly label: string) {
        super();
      }
      convert = (_from: Cells): Item => Object.assign(new Item(), { Label: this.label });
      reverseConvert = (to: Item): Cells => ({ Label: to.Label });
    }
    class TagLabel extends TypeConverter<Item, Cells> {
      convert = (from: Cells): Item =>
        Object.assign(new Item(), { Label: `tag:${String(from.Label)}` });
      reverseConvert = (to: Item): Cells => ({ Label: to.Label });
    }
    class UpperLabel extends TypeConverter<Cells, Cells> {
      convert = (from: Cells): Cells => ({
        ...from,
        Label: String(from.Label).toUpperCase(),
      });
      reverseConvert = (to: Cells): Cells => to;
    }

    const connection = new MockConnection();
    const command = new MockCommand(connection);
    connection.command = command;
    command.reader = new MockReader([{ Label: "ada" }]);
    const itemsRepo = new ReadonlyRepository(connection, {
      type: Item,
      entity: "items",
      readProperties: ["Label"],
      objectConverter: new FixedLabel("default"),
    });
    expect((await itemsRepo.readSingle())?.Label).toBe("default");

    command.reader = new MockReader([{ Label: "ada" }]);
    expect(
      (
        await itemsRepo.readSingle(undefined, undefined, {
          objectConverter: new TagLabel(),
          rowConverter: new UpperLabel(),
        })
      )?.Label,
    ).toBe("tag:ADA");
  });

  it("public include uses the node binder", async () => {
    const person: Seed = { name: "person" };
    const typesConnection = await setup([person]);
    const connection = await setup([{ type_id: person.id }]);
    const parentsRepo = repo(Parent, connection);
    const typesRepo = repo(UserType, typesConnection);
    parentsRepo.addEnriched({ destinationProperty: "TypeName", sourceProperty: "Type", relatedKey: "Id", relatedProperty: "Name" }, typesRepo);
    const items = await collectAsync(parentsRepo.read());
    expect(items[0]?.TypeName).toBe("person");
    await new DefaultReadonlyEnrichmentStrategy<Parent>().apply(
      typesRepo,
      { destinationProperty: "TypeName", sourceProperty: "Type", relatedKey: "Id", relatedProperty: "Name" },
      items,
      [{ Type: person.id }],
    );
  });
});
