import { describe, expect, it } from "vitest";
import { MemoryDriver } from "@datasources/memory";
import { collectAsync } from "./collectAsync.ts";
import { BulkCRUDRepository } from "./BulkCRUDRepository.ts";
import { CascadeDeleteEnrichmentStrategy } from "./CascadeDeleteEnrichmentStrategy.ts";
import { CascadeDeleteUpdateEnrichmentStrategy } from "./CascadeDeleteUpdateEnrichmentStrategy.ts";
import { CRUDRepository } from "./CRUDRepository.ts";
import { DefaultReadonlyEnrichmentStrategy } from "./DefaultReadonlyEnrichmentStrategy.ts";
import { DefaultCreateEnrichmentStrategy } from "./DefaultCreateEnrichmentStrategy.ts";
import { DefaultUpdateEnrichmentStrategy } from "./DefaultUpdateEnrichmentStrategy.ts";
import { DefaultWriteEnrichmentStrategy } from "./DefaultWriteEnrichmentStrategy.ts";
import { LazyWriteEnrichmentStrategy } from "./LazyWriteEnrichmentStrategy.ts";
import { DeleteEnrichmentStrategy } from "./DeleteEnrichmentStrategy.ts";
import { HashsetOptimizedReadonlyEnrichmentStrategy } from "./HashsetOptimizedReadonlyEnrichmentStrategy.ts";
import { NestedToObjectEnrichmentStrategy } from "./NestedToObjectEnrichmentStrategy.ts";
import { ReadonlyRepository } from "./ReadonlyRepository.ts";
import { RepositoryTree } from "./RepositoryTree.ts";
import { RepositoryTreeNode } from "./RepositoryTreeNode.ts";
import { SingleCommandIBulkWriteStrategy } from "./SingleCommandIBulkWriteStrategy.ts";
import type { Cells } from "./converters/Cell.ts";
import type { Scalar } from "./builders/Scalar.ts";
import { TypeConverter } from "./converters/TypeConverter.ts";
import { MockCommand } from "./mocks/mock-command.ts";
import { MockConnection } from "./mocks/mock-connection.ts";
import { MockReader } from "./mocks/mock-reader.ts";
import { n } from "./statements/Native.ts";
import { ExecuteStoredProcedureStatement } from "./statements/ExecuteStoredProcedureStatement.ts";
import { WhereText } from "./statements/WhereText.ts";
import { RowTypeConverter } from "./converters/RowTypeConverter.ts";
import type { WriteMode } from "./WriteMode.ts";

class Item {
  id = 0;
  name = "";
}

class Plain extends TypeConverter<Item, Cells> {
  convert = (row: Cells): Item => Object.assign(new Item(), row);
  reverseConvert = (row: Item): Cells => ({ id: row.id, name: row.name });
}

class OpenUpdate extends DefaultUpdateEnrichmentStrategy<Item> {
  stamp = (related: object, by: string, key: Scalar | object | undefined): void =>
    this.stampCell(related, by, key);

  payload = (parent: object, into: string): object[] => this.payloadOf(parent, into);

  key = (item: object, keys: readonly string[]): string => this.keyOf(item, keys);
}

class OpenCreate extends DefaultCreateEnrichmentStrategy<Item> {
  payload = (parent: object, into: string): object[] => this.payloadOf(parent, into);
}

class DropId extends TypeConverter<Item, Cells> {
  convert = (row: Cells): Item => Object.assign(new Item(), row);
  reverseConvert = (): Cells => ({ name: "a" });
}

const mock = (rows: Cells[] = []) => {
  const connection = new MockConnection();
  const command = new MockCommand(connection);
  command.reader = new MockReader(rows);
  command.response = { result: 9, rowsAffected: 1 };
  connection.command = command;
  return { connection, command };
};

const enrichment = (into: string, extra: object = {}) =>
  ({ destinationProperty: into, sourceProperty: "id", relatedKey: "parent_id", type: "list", ...extra });

describe("statement and repository edges", () => {
  it("names a native fragment", () => {
    expect(n("select 1").text).toBe("select 1");
  });

  it("reads without a connection and without statements", async () => {
    const repo = new ReadonlyRepository();
    await expect(collectAsync(repo.read(new WhereText("id = $id")))).rejects.toThrow();
    await expect(collectAsync(repo.read())).rejects.toThrow();
  });

  it("converts with a plain object converter", async () => {
    const { connection, command } = mock([{ generated: 9 }]);
    const repo = new CRUDRepository<Item>(connection, {
      type: Item,
      entity: "items",
      objectConverter: new Plain(),
    });
    expect(await collectAsync(repo.read())).toHaveLength(1);
    const converted = new ReadonlyRepository<Item>(connection, {
      type: Item,
      entity: "items",
      objectConverter: new Plain(),
      rowConverter: new RowTypeConverter({}),
    });
    command.reader = new MockReader([{ id: 1, name: "a" }]);
    expect(await collectAsync(converted.read())).toHaveLength(1);
    command.reader = new MockReader([]);
    await expect(
      collectAsync(converted.read(new ExecuteStoredProcedureStatement("save_item", []))),
    ).resolves.toEqual([]);
    command.reader = new MockReader([{ generated: 9 }]);
    const created = await repo.create(Object.assign(new Item(), { name: "a" }));
    expect(created.id).toBe(9);
  });

  it("bulk write covers transaction, strategy position, and where text", async () => {
    const { connection, command } = mock();
    const strategy = new SingleCommandIBulkWriteStrategy<Item>();
    const transactional = new BulkCRUDRepository<Item>(
      connection,
      { type: Item, entity: "items", write: { useTransaction: true, persist: async () => {} } },
      strategy,
    );
    const created = await transactional.createBulk([Object.assign(new Item(), { name: "a" })]);
    expect(created.rowsAffected).toBe(1);
    expect(connection.startTransactionCalls).toBe(1);

    expect(() => new BulkCRUDRepository<Item>(connection, strategy)).toThrow();

    const open = new MemoryDriver().connection();
    await open.open();
    const repo = new BulkCRUDRepository<Item>(open, { type: Item, entity: "items", ids: [] });
    await repo.create(Object.assign(new Item(), { name: "a" }));
    await repo.deleteBulk([Object.assign(new Item(), { name: "a" })], new WhereText("id = $id"));
    await repo.deleteBulk([new Item()], new WhereText("1 = 1"));

    const listed = repo.deletes([Object.assign(new Item(), { name: "a" })], new WhereText("id = $id"));
    expect(listed[0]?.text).toContain("$id");
    expect(repo.deletes([new Item()], new WhereText("1 = 1"))[0]?.parameters).toEqual([]);
    expect(repo.deletes([new Item()])[0]?.text).toContain("1 = 1");
    expect(await repo.createOne(new Item())).toBeUndefined();
    const both = new BulkCRUDRepository<Item>(connection, {
      type: Item,
      entity: "items",
      ids: ["id", "name"],
    });
    expect(() => both.updates([new Item()])).toThrow(/non-Id field/);
    const wide = new BulkCRUDRepository<Item>(connection, {
      type: Item,
      entity: "items",
      ids: ["id", "name"],
      rowConverter: new RowTypeConverter({}),
      write: { persist: async () => {} },
    });
    expect(wide.rows([new Item()])[0]).toMatchObject({ id: 0, name: "" });
    expect(await wide.createOne(new Item())).toBeUndefined();

    const dropped = new BulkCRUDRepository<Item>(connection, {
      type: Item,
      entity: "items",
      objectConverter: new DropId(),
    });
    command.response = { result: undefined, rowsAffected: 1 };
    await expect(dropped.deleteBulk([new Item()])).rejects.toThrow(/id/);
    expect(() => dropped.deletes([new Item()])).toThrow(/Missing Id value/);

    const plain = new BulkCRUDRepository<Item>(connection, { type: Item, entity: "items" });
    command.reader = new MockReader([]);
    expect(
      await plain.createOne(Object.assign(new Item(), { id: { nested: true } as unknown as number })),
    ).toBeUndefined();
  });

  it("create follows enrichment and inserted-id edges", async () => {
    const { connection, command } = mock([]);
    const writer = new CRUDRepository<Item>(connection, { type: Item, entity: "items" });
    const host = new CRUDRepository<Item>(connection, { type: Item, entity: "parents" });
    host.addEnriched({ destinationProperty: "Children", sourceProperty: "id", relatedKey: "parent_id", type: "list" }, writer);
    const objectId = Object.assign(new Item(), { id: { nested: true } as unknown as number, name: "a" });
    expect(await host.create(objectId)).toBe(objectId);
    const numeric = Object.assign(new Item(), { name: "b" });
    expect(await host.create(numeric)).toBe(numeric);
    const wide = new CRUDRepository<Item>(connection, {
      type: Item,
      entity: "parents",
      ids: ["id", "name"],
    });
    wide.addEnriched({ destinationProperty: "Children", sourceProperty: "id", relatedKey: "parent_id", type: "list" }, writer);
    expect(await wide.create(new Item())).toBeInstanceOf(Item);

    command.reader = new MockReader([undefined as unknown as Cells]);
    await expect(host.create(new Item())).rejects.toThrow("Reader current must be a row");

    const plainSide = mock([{ id: 9 }]);
    const plainRepo = new CRUDRepository<Item>(plainSide.connection, { type: Item, entity: "items" });
    expect((await plainRepo.create(Object.assign(new Item(), { name: "a" }))).id).toBe(9);
    const convertedSide = mock([{ generated: 4 }]);
    const convertedRepo = new CRUDRepository<Item>(convertedSide.connection, {
      type: Item,
      entity: "items",
      objectConverter: new Plain(),
      rowConverter: new RowTypeConverter({}),
    });
    expect((await convertedRepo.create(Object.assign(new Item(), { name: "a" }))).id).toBe(4);
  });
});

describe("enrichment edges", () => {
  it("requires a writer and an association", async () => {
    const owned = enrichment("Children");
    const cascade = new CascadeDeleteEnrichmentStrategy<Item>();
    await expect(cascade.deleteLinked(new CRUDRepository<Item>(mock([]).connection, { entity: "items" }), owned, {})).rejects.toThrow(
      "Missing association",
    );
    const update = new CascadeDeleteUpdateEnrichmentStrategy<Item>();
    await expect(update.updateLinked(new CRUDRepository<Item>(mock([]).connection, { entity: "items" }), owned, {})).rejects.toThrow(
      "Missing association",
    );
    const replacing = new DefaultUpdateEnrichmentStrategy<Item>();
    await expect(replacing.updateLinked(new CRUDRepository<Item>(mock([]).connection, { entity: "items" }), owned, {})).rejects.toThrow(
      "Missing association",
    );

    const { connection } = mock([]);
    const writer = new CRUDRepository<Item>(connection, { type: Item, entity: "items" });
    const tags = {
      destinationProperty: "Tags",
      sourceProperty: "id",
      relatedKey: "id",
      association: { parent: "parent_id", related: "tag_id" },
    };
    await cascade.deleteLinked(writer, tags, { id: 0 });
    await cascade.deleteLinked(writer, tags, { id: "" });
    await expect(DeleteEnrichmentStrategy.load(writer, owned, { id: { nested: true } })).resolves.toEqual([]);

    const host = new CRUDRepository<Item>(connection, { type: Item, entity: "items" });
    host.addEnriched({ destinationProperty: "Children", sourceProperty: "id", relatedKey: "parent_id", type: "list" }, writer);
    const tree = new RepositoryTree(new RepositoryTreeNode(host, ""));
    await expect(
      new DefaultWriteEnrichmentStrategy<Item>().persist(tree, [new Item()], "nope" as WriteMode),
    ).rejects.toThrow("Unknown persist mode");
    await new DefaultWriteEnrichmentStrategy<Item>().persist(
      new RepositoryTree(new RepositoryTreeNode({}, "")),
      [],
      "create",
    );

    await new HashsetOptimizedReadonlyEnrichmentStrategy<Item>().apply(
      writer,
      { destinationProperty: "name", sourceProperty: "missing", relatedKey: "id", relatedProperty: "name" },
      [{}],
    );
    await new NestedToObjectEnrichmentStrategy<Item>({ key: "id", fields: { id: "id" } }).apply(
      writer,
      { destinationProperty: "name", sourceProperty: "id", relatedKey: "id" },
      [],
    );
  });

  it("walks lookup, link, and replace edges", async () => {
    const tags = { destinationProperty: "Tags", sourceProperty: "id", relatedKey: "id", association: { parent: "parent_id", related: "tag_id" } };
    const empty = mock([]);
    const via = new CRUDRepository<Item>(empty.connection, { type: Item, entity: "tags" });
    const lookup = new DefaultReadonlyEnrichmentStrategy<Item>();
    await lookup.apply(via, { destinationProperty: "name", sourceProperty: "id", relatedKey: "id" }, [{ id: null }, { id: 1 }]);
    await lookup.apply(via, { destinationProperty: "Children", sourceProperty: "id", relatedKey: "parent_id", type: "list" }, [{ id: null }, {}]);
    await expect(lookup.applyLinked(via, via, { destinationProperty: "Tags", sourceProperty: "id", relatedKey: "id" }, [{ id: 1 }])).rejects.toThrow(
      "Missing association",
    );
    await lookup.applyLinked(via, via, tags, [{ id: null }]);
    const linkSide = mock([{ tag_id: null }, { tag_id: { nested: true } as never }, { tag_id: 2 }]);
    const linkRepo = new CRUDRepository<Item>(linkSide.connection, { entity: "links" });
    await lookup.applyLinked(via, linkRepo, tags, [{ id: 1 }]);

    const hash = new HashsetOptimizedReadonlyEnrichmentStrategy<Item>();
    await hash.apply(via, { destinationProperty: "name", sourceProperty: "id", relatedKey: "id", relatedProperty: "name", type: "list" }, [{}]);
    const named = mock([{ id: 1, name: "a" }]);
    const namedRepo = new CRUDRepository<Item>(named.connection, { type: Item, entity: "tags" });
    await hash.apply(namedRepo, { destinationProperty: "name", sourceProperty: "id", relatedKey: "id", relatedProperty: "name" }, [{ id: 1 }, {}]);
    const many = mock([{ parent_id: 1, name: "a" }]);
    const manyRepo = new CRUDRepository<Item>(many.connection, { type: Item, entity: "children" });
    await hash.apply(manyRepo, { destinationProperty: "Children", sourceProperty: "id", relatedKey: "parent_id", type: "list" }, [{ id: 1 }, { id: null }]);

    const children = { destinationProperty: "Children", sourceProperty: "id", relatedKey: "parent_id", type: "list" };
    const quiet = mock([]);
    const writer = new CRUDRepository<Item>(quiet.connection, { type: Item, entity: "children", ids: ["id"] });
    const replacing = new DefaultUpdateEnrichmentStrategy<Item>();
    await replacing.updateOwned(writer, children, { id: 1 });
    const existing = mock([
      { id: 1, parent_id: 1, name: "old" },
      { id: 9, parent_id: 1, name: "gone" },
    ]);
    const existingRepo = new CRUDRepository<Item>(existing.connection, { type: Item, entity: "children", ids: ["id"] });
    await replacing.updateOwned(existingRepo, children, {
      id: 1,
      Children: [{ id: 1, name: "kept" }, { name: "fresh" }, null, "skip"],
    });

    const linkedRows = mock([{ tag_id: 0 }, { tag_id: 2 }, {}]);
    const linkWriter = new CRUDRepository<Item>(linkedRows.connection, { entity: "links", ids: ["id"] });
    await replacing.updateLinked(linkWriter, tags, { id: 1, Tags: [{ id: 2 }, { id: 2 }, { id: 3 }] });
    await expect(new DefaultCreateEnrichmentStrategy<Item>().createLinked(writer, { destinationProperty: "Tags", sourceProperty: "id", relatedKey: "id" }, { id: 1 })).rejects.toThrow(
      "Missing association",
    );
    const noIds = new CRUDRepository<Item>(mock([]).connection, { entity: "children", ids: [] });
    await replacing.updateOwned(noIds, children, {
      id: 1,
      Children: [{ name: "a" }],
    });
    const creating = new DefaultCreateEnrichmentStrategy<Item>();
    await expect(creating.createLinked(writer, tags, { id: 0, Tags: [{ id: 1 }] })).rejects.toThrow(/requires/);
    await expect(creating.createLinked(writer, tags, { id: 1, Tags: [{ id: 0 }] })).rejects.toThrow(/requires/);

    const open = new OpenUpdate();
    expect(open.key({ name: "a" }, ["missing"])).toBe("");
    expect(open.key({ id: 5 }, ["id"])).toBe("5");
    expect(open.key({ id: null as unknown as number }, ["id"])).toBe("");
    expect(open["requiredKey"]({ id: 1 }, {
      destinationProperty: "Tags",
      sourceProperty: "id",
      relatedKey: "id",
    })).toBe(1);
    expect(() =>
      open["requiredKey"]({ id: 0 }, {
        destinationProperty: "Tags",
        sourceProperty: "id",
        relatedKey: "id",
      }),
    ).toThrow(/requires/);
    open.stamp({ parent: null as unknown as object }, "parent", 8);
    expect(open.payload({ Children: { not: "array" } }, "Children")).toEqual([]);
    expect(open.payload({ Children: "text" }, "Children")).toEqual([]);
    const creatingOpen = new OpenCreate();
    expect(creatingOpen.payload({ Children: { not: "array" } }, "Children")).toEqual([]);
    expect(creatingOpen.payload({ Children: [null, "skip", { id: 1n }] }, "Children")).toEqual([{ id: 1n }]);
    const stamped = { parent: { Id: 0 } };
    creatingOpen["stampCell"](stamped, "parent", 9);
    expect(stamped.parent.Id).toBe(9);
    const createSnake = { parent: { name: "a" } };
    creatingOpen["stampCell"](createSnake, "parent", 5);
    expect((createSnake.parent as { id?: number }).id).toBe(5);
    const createFlat = { parent_id: 0 };
    creatingOpen["stampCell"](createFlat, "parent_id", 6);
    expect(createFlat.parent_id).toBe(6);
    creatingOpen["stampCell"](createFlat, "parent_id", undefined);
    creatingOpen["stampCell"](createFlat, "parent_id", null);
    creatingOpen["stampCell"](createFlat, "parent_id", { nested: true });
    creatingOpen["stampCell"](createFlat, "parent_id", []);
    expect(open["namesOf"]({ ids: [] } as never, { id: 1, name: "a" })).toEqual(["id"]);
    expect(open["namesOf"]({ ids: ["id"] } as never, { id: 1 })).toEqual(["id"]);
    expect(() =>
      creatingOpen["relatedIds"]({ Tags: [{ id: 0 }] }, {
        destinationProperty: "Tags",
        sourceProperty: "id",
        relatedKey: "id",
      }),
    ).toThrow(/requires/);
    expect(() =>
      creatingOpen["relatedIds"]({ Tags: [{ id: 0n }] }, {
        destinationProperty: "Tags",
        sourceProperty: "id",
        relatedKey: "id",
      }),
    ).toThrow(/requires/);
    expect(() =>
      creatingOpen["relatedIds"]({ Tags: [{ id: "" }] }, {
        destinationProperty: "Tags",
        sourceProperty: "id",
        relatedKey: "id",
      }),
    ).toThrow(/requires/);
    expect(() =>
      creatingOpen["relatedIds"]({ Tags: [{ id: { nested: true } }] }, {
        destinationProperty: "Tags",
        sourceProperty: "id",
        relatedKey: "id",
      }),
    ).toThrow(/requires/);
    expect(
      creatingOpen["relatedIds"]({ Tags: [{ id: 2 }, { id: 2 }] }, {
        destinationProperty: "Tags",
        sourceProperty: "id",
        relatedKey: "id",
      }),
    ).toEqual([2]);
    expect(open.payload({ Children: [null, "skip", { id: 1 }] }, "Children")).toEqual([{ id: 1 }]);
    expect(() =>
      open["relatedIds"]({ Tags: [{ id: 0 }] }, {
        destinationProperty: "Tags",
        sourceProperty: "id",
        relatedKey: "id",
      }),
    ).toThrow(/requires/);
    expect(() =>
      open["relatedIds"]({ Tags: [{ id: 0n }] }, {
        destinationProperty: "Tags",
        sourceProperty: "id",
        relatedKey: "id",
      }),
    ).toThrow(/requires/);
    expect(() =>
      open["relatedIds"]({ Tags: [{ id: "" }] }, {
        destinationProperty: "Tags",
        sourceProperty: "id",
        relatedKey: "id",
      }),
    ).toThrow(/requires/);
    expect(
      open["relatedIds"]({ Tags: [{ id: 2 }, { id: 2 }] }, {
        destinationProperty: "Tags",
        sourceProperty: "id",
        relatedKey: "id",
      }),
    ).toEqual([2]);
    expect(open["hasKey"]({}, [])).toBe(false);
    expect(open["hasKey"]({ id: 1 }, ["id"])).toBe(true);
    expect(open["hasKey"]({ id: 0 }, ["id"])).toBe(false);
    expect(open["hasKey"]({ id: 1 }, ["id", "missing"])).toBe(false);
    const nested = { parent: { Id: 0 } };
    open.stamp(nested, "parent", 4);
    expect(nested.parent.Id).toBe(4);
    const snake = { parent: { name: "a" } };
    open.stamp(snake, "parent", 5);
    expect((snake.parent as { id?: number }).id).toBe(5);
    const flat = { parent_id: 0 };
    open.stamp(flat, "parent_id", 6);
    expect(flat.parent_id).toBe(6);
    open.stamp(flat, "parent_id", { nested: true });
    const withArray = { parent_id: [1, 2] as unknown as number };
    open.stamp(withArray, "parent_id", 7);
    expect(withArray.parent_id).toBe(7);

    const cascade = new CascadeDeleteEnrichmentStrategy<Item>();
    const cascadeRows = mock([{ tag_id: 0 }, { tag_id: 2 }]);
    const cascadeWriter = new CRUDRepository<Item>(cascadeRows.connection, { entity: "links", ids: ["id"] });
    await cascade.deleteLinked(cascadeWriter, tags, { id: 1 });
    const cascadeUpdate = new CascadeDeleteUpdateEnrichmentStrategy<Item>();
    const cascadeUpdateRows = mock([{ tag_id: 0 }, { tag_id: 3 }]);
    const cascadeUpdateWriter = new CRUDRepository<Item>(cascadeUpdateRows.connection, { entity: "links", ids: ["id"] });
    await cascadeUpdate.updateLinked(cascadeUpdateWriter, tags, { id: 1, Tags: [{ id: 4 }] });

    const parents = mock([]);
    const host = new CRUDRepository<Item>(parents.connection, { type: Item, entity: "parents" });
    host.addEnriched(children, writer);
    const tree = new RepositoryTree(new RepositoryTreeNode(host, ""));
    await new LazyWriteEnrichmentStrategy<Item>().persist(tree, [new Item()], "create");
    const associated = new CRUDRepository<Item>(parents.connection, { type: Item, entity: "parents" });
    associated.addEnriched(tags, writer);
    await expect(
      new DefaultWriteEnrichmentStrategy<Item>().persist(
        new RepositoryTree(new RepositoryTreeNode(associated, "")),
        [{ id: 1, Tags: [{ id: 2 }] } as unknown as Item],
        "create",
      ),
    ).rejects.toThrow(/Tags/);
  });

  it("rejects custom fills", async () => {
    const { connection } = mock([]);
    const via = new CRUDRepository<Item>(connection, { type: Item, entity: "items" });
    const read = new DefaultReadonlyEnrichmentStrategy<Item>();
    const fill = {
      destinationProperty: "name",
      sourceProperty: "id",
      relatedKey: "id",
      type: "custom" as const,
    };
    await expect(read.apply(via, fill, [])).rejects.toThrow('Unsupported enrichment type "custom"');
    const custom = new CRUDRepository<Item>(connection, { type: Item, entity: "parents" });
    custom.addEnriched(fill, via);
    await expect(
      new DefaultWriteEnrichmentStrategy<Item>().persist(
        new RepositoryTree(new RepositoryTreeNode(custom, "")),
        [new Item()],
        "create",
      ),
    ).rejects.toThrow('Unsupported enrichment type "custom"');
  });
});
