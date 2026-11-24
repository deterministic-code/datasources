import { describe, expect, it } from "vitest";
import { MemoryDriver } from "@deterministic-code/datasources-memory";
import { BatchDeleteEnrichmentStrategy } from "./BatchDeleteEnrichmentStrategy.ts";
import { BatchUpdateEnrichmentStrategy } from "./BatchUpdateEnrichmentStrategy.ts";
import { CascadeDeleteEnrichmentStrategy } from "./CascadeDeleteEnrichmentStrategy.ts";
import { collectAsync } from "./collectAsync.ts";
import { CRUDRepository } from "./CRUDRepository.ts";
import { DefaultReadonlyEnrichmentStrategy } from "./DefaultReadonlyEnrichmentStrategy.ts";
import { DefaultUpdateEnrichmentStrategy } from "./DefaultUpdateEnrichmentStrategy.ts";
import { DefaultWriteEnrichmentStrategy } from "./DefaultWriteEnrichmentStrategy.ts";
import { RepositoryTree } from "./RepositoryTree.ts";
import { RepositoryTreeNode } from "./RepositoryTreeNode.ts";
import type { Enrichment } from "./Enrichment.ts";
import type { ICRUDRepository } from "./ICRUDRepository.ts";
import type { IConnection } from "./IConnection.ts";
import type { IReadonlyRepository, ReadQuery } from "./IReadonlyRepository.ts";
import { ChangeCasingMapper } from "./mappers/ChangeCasingMapper.ts";
import type { Parameter } from "./Parameter.ts";
import type { ReadonlyRepositoryMappingOptions } from "./ReadonlyRepositoryMappingOptions.ts";
import { Schema } from "./Schema.ts";

const PARENTS = 100;

class Tag {
  Id = 0;
  Name = "";
}

class ParentTag {
  Id = 0;
  ParentId = 0;
  TagId = 0;
}

class Child {
  Id = 0;
  ParentId = 0;
  Label = "";
}

class Parent {
  Id = 0;
  Label = "";
  Children: Child[] = [];
  Tags: Tag[] = [];
}

const snake = new ChangeCasingMapper("pascalCase", "snakeCase");

const open = <T extends object>(type: new () => T, connection: IConnection) =>
  new CRUDRepository<T>(connection, {
    type,
    propertyMappings: snake,
    ids: ["Id"],
    schema: new Schema({ Id: { type: "integer", nullable: false, hasDefault: false, isId: true } }),
  });

const openConnection = async (): Promise<IConnection> => {
  const connection = new MemoryDriver().connection();
  await connection.open();
  return connection;
};

const assign = <T extends object>(type: new () => T, values: Partial<T>): T => Object.assign(new type(), values);

const childrenFill: Enrichment = { destinationProperty: "Children", sourceProperty: "Id", relatedKey: "ParentId", type: "list" };
const tagsFill: Enrichment = {
  destinationProperty: "Tags",
  sourceProperty: "Id",
  relatedKey: "Id",
  association: { parent: "ParentId", related: "TagId" },
};

interface Fixture {
  connection: IConnection;
  parents: Parent[];
  host: CRUDRepository<Parent>;
  children: CRUDRepository<Child>;
  links: CRUDRepository<ParentTag>;
  childReads: { count: number };
  linkReads: { count: number };
}

const seed = async (): Promise<Fixture> => {
  const connection = await openConnection();
  const parentsRepo = open(Parent, connection);
  const children = open(Child, connection);
  const links = open(ParentTag, connection);
  const parents: Parent[] = [];
  for (let i = 0; i < PARENTS; i += 1) {
    const parent = await parentsRepo.create(assign(Parent, { Label: `p${i}` }));
    await children.create(assign(Child, { ParentId: parent.Id, Label: `child${i}` }));
    await links.create(assign(ParentTag, { ParentId: parent.Id, TagId: i + 1 }));
    parents.push(parent);
  }
  return { connection, parents, host: parentsRepo, children, links, childReads: { count: 0 }, linkReads: { count: 0 } };
};

const labels = async (children: CRUDRepository<Child>): Promise<string[]> =>
  (await collectAsync(children.read())).map((child) => child.Label).sort();

const withChangedChildren = (f: Fixture, stored: Child[]): Parent[] =>
  f.parents.map((parent) =>
    assign(Parent, {
      Id: parent.Id,
      Children: stored
        .filter((child) => child.ParentId === parent.Id)
        .map((child) => assign(Child, { Id: child.Id, ParentId: child.ParentId, Label: `${child.Label}!` })),
    }),
  );

describe("batch enrichment", () => {
  it("default strategy linked is one read per table", async () => {
    const connection = await openConnection();
    const parentsRepo = open(Parent, connection);
    const tagsRepo = open(Tag, connection);
    const linksRepo = open(ParentTag, connection);
    const tag = await tagsRepo.create(assign(Tag, { Name: "t" }));
    for (let i = 0; i < PARENTS; i += 1) {
      const parent = await parentsRepo.create(new Parent());
      await linksRepo.create(assign(ParentTag, { ParentId: parent.Id, TagId: tag.Id }));
    }
    const tagReads = { count: 0 };
    const linkReads = { count: 0 };
    parentsRepo.addLinked(
      tagsFill,
      new CountingRepository(tagsRepo, tagReads),
      new CountingCRUDRepository(linksRepo, linkReads),
      new DefaultReadonlyEnrichmentStrategy(),
    );
    const items = await collectAsync(parentsRepo.read());
    expect(items).toHaveLength(PARENTS);
    expect(items.every((item) => item.Tags.length === 1)).toBe(true);
    expect(linkReads.count).toBe(1);
    expect(tagReads.count).toBe(1);
  });

  it("create resolves twenty linked names with one tag lookup", async () => {
    const connection = await openConnection();
    const parentsRepo = open(Parent, connection);
    const tagsRepo = open(Tag, connection);
    const linksRepo = open(ParentTag, connection);
    const wanted: Tag[] = [];
    for (let i = 0; i < 20; i += 1) wanted.push(await tagsRepo.create(assign(Tag, { Name: `tag${i}` })));
    const reads = { count: 0 };
    parentsRepo.addLinked(
      { ...tagsFill, relatedProperty: "Name" },
      new CountingRepository(tagsRepo, reads),
      linksRepo,
    );
    const created = await parentsRepo.create(
      assign(Parent, { Tags: wanted.map((tag) => assign(Tag, { Name: tag.Name })) }),
    );
    const linked = (await collectAsync(linksRepo.read())).filter((link) => link.ParentId === created.Id).map((link) => link.TagId);
    expect(linked.sort()).toEqual(wanted.map((tag) => tag.Id).sort());
    // one batched name lookup, plus one read when the created row is enriched; was 21
    expect(reads.count).toBe(2);
  });

  it("updateChildrenBatch reads once and matches the default strategy", async () => {
    const batch = await seed();
    const single = await seed();
    const batchParents = withChangedChildren(batch, await collectAsync(batch.children.read()));
    const singleParents = withChangedChildren(single, await collectAsync(single.children.read()));
    await new BatchUpdateEnrichmentStrategy<Parent>().updateChildrenBatch(
      new CountingCRUDRepository(batch.children, batch.childReads),
      childrenFill,
      batchParents,
    );
    expect(batch.childReads.count).toBe(1);
    for (const parent of singleParents)
      await new DefaultUpdateEnrichmentStrategy<Parent>().updateChildren(
        new CountingCRUDRepository(single.children, single.childReads),
        childrenFill,
        parent,
      );
    expect(single.childReads.count).toBe(PARENTS);
    expect(await labels(batch.children)).toEqual(await labels(single.children));
    expect((await labels(batch.children)).every((label) => label.endsWith("!"))).toBe(true);
  });

  it("updateChildrenBatch deletes children dropped from a parent", async () => {
    const f = await seed();
    const emptied = f.parents.map((parent) => assign(Parent, { Id: parent.Id }));
    await new BatchUpdateEnrichmentStrategy<Parent>().updateChildrenBatch(f.children, childrenFill, emptied);
    expect(await collectAsync(f.children.read())).toHaveLength(0);
  });

  it("deleteChildrenBatch reads once and matches the default strategy", async () => {
    const batch = await seed();
    const single = await seed();
    const keep = PARENTS - 1;
    let nested = 0;
    await new BatchDeleteEnrichmentStrategy<Parent>().deleteChildrenBatch(
      new CountingCRUDRepository(batch.children, batch.childReads),
      childrenFill,
      batch.parents.slice(0, keep),
      async (rows) => {
        nested += rows.length;
      },
    );
    expect(batch.childReads.count).toBe(1);
    expect(nested).toBe(keep);
    for (const parent of single.parents.slice(0, keep))
      await new CascadeDeleteEnrichmentStrategy<Parent>().deleteChildren(
        new CountingCRUDRepository(single.children, single.childReads),
        childrenFill,
        parent,
        async () => {},
      );
    expect(single.childReads.count).toBe(keep);
    expect(await collectAsync(batch.children.read())).toHaveLength(1);
    expect(await labels(batch.children)).toEqual(await labels(single.children));
  });

  it("deleteLinkedBatch reads once and matches the default strategy", async () => {
    const batch = await seed();
    const single = await seed();
    const keep = PARENTS - 1;
    await new BatchDeleteEnrichmentStrategy<Parent>().deleteLinkedBatch(
      new CountingCRUDRepository(batch.links, batch.linkReads),
      tagsFill,
      batch.parents.slice(0, keep),
    );
    expect(batch.linkReads.count).toBe(1);
    for (const parent of single.parents.slice(0, keep))
      await new CascadeDeleteEnrichmentStrategy<Parent>().deleteLinked(
        new CountingCRUDRepository(single.links, single.linkReads),
        tagsFill,
        parent,
      );
    expect(single.linkReads.count).toBe(keep);
    expect(await collectAsync(batch.links.read())).toHaveLength(1);
    expect(await collectAsync(single.links.read())).toHaveLength(1);
  });

  it("updateLinkedBatch reads once and matches the default strategy", async () => {
    const batch = await seed();
    const single = await seed();
    const wanted = (f: Fixture): Parent[] =>
      f.parents.map((parent) => assign(Parent, { Id: parent.Id, Tags: [assign(Tag, { Id: 1000 })] }));
    await new BatchUpdateEnrichmentStrategy<Parent>().updateLinkedBatch(
      new CountingCRUDRepository(batch.links, batch.linkReads),
      tagsFill,
      wanted(batch),
    );
    expect(batch.linkReads.count).toBe(1);
    for (const parent of wanted(single))
      await new DefaultUpdateEnrichmentStrategy<Parent>().updateLinked(
        new CountingCRUDRepository(single.links, single.linkReads),
        tagsFill,
        parent,
      );
    expect(single.linkReads.count).toBe(PARENTS);
    const tags = async (f: Fixture) => (await collectAsync(f.links.read())).map((link) => link.TagId).sort();
    expect(await tags(batch)).toEqual(await tags(single));
    expect((await tags(batch)).every((tag) => tag === 1000)).toBe(true);
  });

  it("persistBulk update children reads once and drops an emptied parent", async () => {
    const batch = await seed();
    const single = await seed();
    const batchParents = withChangedChildren(batch, await collectAsync(batch.children.read()));
    batchParents[0]!.Children = [];
    const singleParents = withChangedChildren(single, await collectAsync(single.children.read()));
    singleParents[0]!.Children = [];
    batch.host.addEnriched(childrenFill, new CountingCRUDRepository(batch.children, batch.childReads));
    await new DefaultWriteEnrichmentStrategy<Parent>().persistBulk(
      new RepositoryTree(new RepositoryTreeNode(batch.host, "")),
      batchParents,
      "update",
    );
    expect(batch.childReads.count).toBe(1);
    for (const parent of singleParents)
      await new DefaultUpdateEnrichmentStrategy<Parent>().updateChildren(
        new CountingCRUDRepository(single.children, single.childReads),
        childrenFill,
        parent,
      );
    expect(single.childReads.count).toBe(PARENTS);
    expect(await labels(batch.children)).toEqual(await labels(single.children));
    expect((await collectAsync(batch.children.read())).some((child) => child.ParentId === batch.parents[0]!.Id)).toBe(false);
    expect((await labels(batch.children)).every((label) => label.endsWith("!"))).toBe(true);
  });

  it("persistBulk update linked reads once and matches the default strategy", async () => {
    const batch = await seed();
    const single = await seed();
    const wanted = (f: Fixture): Parent[] =>
      f.parents.map((parent) => assign(Parent, { Id: parent.Id, Tags: [assign(Tag, { Id: 1000 })] }));
    batch.host.addLinked(tagsFill, open(Tag, batch.connection), new CountingCRUDRepository(batch.links, batch.linkReads));
    await new DefaultWriteEnrichmentStrategy<Parent>().persistBulk(
      new RepositoryTree(new RepositoryTreeNode(batch.host, "")),
      wanted(batch),
      "update",
    );
    expect(batch.linkReads.count).toBe(1);
    for (const parent of wanted(single))
      await new DefaultUpdateEnrichmentStrategy<Parent>().updateLinked(
        new CountingCRUDRepository(single.links, single.linkReads),
        tagsFill,
        parent,
      );
    expect(single.linkReads.count).toBe(PARENTS);
    const tags = async (f: Fixture) => (await collectAsync(f.links.read())).map((link) => link.TagId).sort();
    expect(await tags(batch)).toEqual(await tags(single));
    expect((await tags(batch)).every((tag) => tag === 1000)).toBe(true);
  });
});

class CountingRepository<T extends object> implements IReadonlyRepository<T> {
  constructor(
    private readonly inner: IReadonlyRepository<T>,
    private readonly reads: { count: number },
  ) {}

  get type() {
    return this.inner.type;
  }

  get schema() {
    return this.inner.schema;
  }

  get isSchemaRequired() {
    return this.inner.isSchemaRequired;
  }

  readSingle = (
    query?: ReadQuery,
    parameters?: Parameter[],
    options?: ReadonlyRepositoryMappingOptions<T>,
  ): Promise<T | undefined> => {
    this.reads.count += 1;
    return this.inner.readSingle(query, parameters, options);
  };

  read = (
    query?: ReadQuery,
    parameters?: Parameter[],
    options?: ReadonlyRepositoryMappingOptions<T>,
  ): AsyncIterable<T> => {
    this.reads.count += 1;
    return this.inner.read(query, parameters, options);
  };
}

class CountingCRUDRepository<T extends object> extends CountingRepository<T> implements ICRUDRepository<T> {
  constructor(
    private readonly crud: ICRUDRepository<T>,
    reads: { count: number },
  ) {
    super(crud, reads);
  }

  get ids() {
    return this.crud.ids;
  }

  create = (item: T) => this.crud.create(item);
  update = (item: T, byWhere?: Parameters<ICRUDRepository<T>["update"]>[1]) => this.crud.update(item, byWhere);
  delete = (item: T, byWhere?: Parameters<ICRUDRepository<T>["delete"]>[1]) => this.crud.delete(item, byWhere);
  readSingleById = (id: Parameters<ICRUDRepository<T>["readSingleById"]>[0]) => this.crud.readSingleById(id);
}
