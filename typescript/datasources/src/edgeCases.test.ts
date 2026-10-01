import { describe, expect, it } from "vitest";
import { MemoryDriver } from "@datasources/memory";
import { collectAsync } from "./collectAsync.ts";
import { BulkCRUDRepository } from "./BulkCRUDRepository.ts";
import { CascadeDeleteEnrichmentStrategy } from "./CascadeDeleteEnrichmentStrategy.ts";
import { CascadeDeleteUpdateEnrichmentStrategy } from "./CascadeDeleteUpdateEnrichmentStrategy.ts";
import { CRUDRepository } from "./CRUDRepository.ts";
import { DatasourceError } from "./DatasourceError.ts";
import { DefaultUpdateEnrichmentStrategy } from "./DefaultUpdateEnrichmentStrategy.ts";
import { DefaultWriteEnrichmentStrategy } from "./DefaultWriteEnrichmentStrategy.ts";
import { DeleteEnrichmentStrategy } from "./DeleteEnrichmentStrategy.ts";
import { HashsetOptimizedReadonlyEnrichmentStrategy } from "./HashsetOptimizedReadonlyEnrichmentStrategy.ts";
import { NestedToObjectEnrichmentStrategy } from "./NestedToObjectEnrichmentStrategy.ts";
import { ReadonlyRepository } from "./ReadonlyRepository.ts";
import { RepositoryTree } from "./RepositoryTree.ts";
import { RepositoryTreeNode } from "./RepositoryTreeNode.ts";
import { SingleCommandIBulkWriteStrategy } from "./SingleCommandIBulkWriteStrategy.ts";
import { StoredEnrichment } from "./StoredEnrichment.ts";
import type { Cells } from "./converters/Cell.ts";
import { TypeConverter } from "./converters/TypeConverter.ts";
import { MockCommand } from "./mocks/mock-command.ts";
import { MockConnection } from "./mocks/mock-connection.ts";
import { MockReader } from "./mocks/mock-reader.ts";
import { n } from "./statements/Native.ts";
import { WhereText } from "./statements/WhereText.ts";
import type { WriteMode } from "./WriteMode.ts";

class Item {
  id = 0;
  name = "";
}

class Plain extends TypeConverter<Item, Cells> {
  convert = (row: Cells): Item => Object.assign(new Item(), row);
  reverseConvert = (row: Item): Cells => ({ id: row.id, name: row.name });
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

const entry = (into: string, extra: object = {}) =>
  new StoredEnrichment(
    { into, from: "id", by: "parent_id", many: true, ...extra },
    async () => {},
  );

describe("statement and repository edges", () => {
  it("names a native fragment", () => {
    expect(n("select 1").text).toBe("select 1");
  });

  it("reads without a connection and without statements", async () => {
    const repo = new ReadonlyRepository();
    await expect(collectAsync(repo.read(new WhereText("id = $id")))).rejects.toThrow();
  });

  it("converts with a plain object converter", async () => {
    const { connection, command } = mock([{ generated: 9 }]);
    const repo = new CRUDRepository<Item>(connection, {
      type: Item,
      entity: "items",
      objectConverter: new Plain(),
    });
    expect(await collectAsync(repo.read())).toHaveLength(1);
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

    const dropped = new BulkCRUDRepository<Item>(connection, {
      type: Item,
      entity: "items",
      objectConverter: new DropId(),
    });
    command.response = { result: undefined, rowsAffected: 1 };
    await expect(dropped.deleteBulk([new Item()])).rejects.toThrow(/id/);
  });
});

describe("enrichment edges", () => {
  it("requires a writer and an association", async () => {
    const owned = entry("Children");
    const cascade = new CascadeDeleteEnrichmentStrategy<Item>();
    await expect(cascade.deleteOwned(owned, owned.enrichment, {}, async () => {})).rejects.toThrow(/Children/);
    await expect(cascade.deleteLinked(owned, owned.enrichment, {})).rejects.toThrow("Missing association");
    const update = new CascadeDeleteUpdateEnrichmentStrategy<Item>();
    await expect(update.updateOwned(owned, owned.enrichment, {})).rejects.toThrow(/Children/);
    await expect(update.updateLinked(owned, owned.enrichment, {})).rejects.toThrow("Missing association");
    const replacing = new DefaultUpdateEnrichmentStrategy<Item>();
    await expect(replacing.updateOwned(owned, owned.enrichment, {})).rejects.toThrow(/Children/);
    await expect(replacing.updateLinked(owned, owned.enrichment, {})).rejects.toThrow("Missing association");

    const { connection } = mock([]);
    const writer = new CRUDRepository<Item>(connection, { type: Item, entity: "items" });
    const linked = new StoredEnrichment(
      { into: "Tags", from: "id", by: "id", association: { parent: "parent_id", related: "tag_id" } },
      async () => {},
      undefined,
      writer,
    );
    await cascade.deleteLinked(linked, linked.enrichment, { id: 0 });
    await cascade.deleteLinked(linked, linked.enrichment, { id: "" });
    await expect(DeleteEnrichmentStrategy.load(writer, owned.enrichment, { id: { nested: true } })).resolves.toEqual([]);

    const host = new CRUDRepository<Item>(connection, { type: Item, entity: "items" });
    host.addEnriched({ into: "Children", from: "id", by: "parent_id", many: true }, writer);
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
      { into: "name", from: "missing", by: "id", select: "name" },
      [{}],
    );
    await new NestedToObjectEnrichmentStrategy<Item>({ key: "id", fields: { id: "id" } }).apply(
      writer,
      { into: "name", from: "id", by: "id" },
      [],
    );
  });
});
