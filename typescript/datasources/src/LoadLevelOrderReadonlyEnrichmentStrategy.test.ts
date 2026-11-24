import { describe, expect, it } from "vitest";
import { MemoryDriver } from "@deterministic-code/datasources-memory";
import { collectAsync } from "./collectAsync.ts";
import { CRUDRepository } from "./CRUDRepository.ts";
import { DefaultReadonlyEnrichmentStrategy } from "./DefaultReadonlyEnrichmentStrategy.ts";
import type { ICRUDRepository } from "./ICRUDRepository.ts";
import type { IConnection } from "./IConnection.ts";
import type { IReadonlyEnrichmentStrategy } from "./IReadonlyEnrichmentStrategy.ts";
import type { IReadonlyRepository } from "./IReadonlyRepository.ts";
import { LoadLevelOrderReadonlyEnrichmentStrategy } from "./LoadLevelOrderReadonlyEnrichmentStrategy.ts";
import { ChangeCasingMapper } from "./mappers/ChangeCasingMapper.ts";
import type { Parameter } from "./Parameter.ts";
import { Schema } from "./Schema.ts";
import type { ReadQuery } from "./IReadonlyRepository.ts";
import type { ReadonlyRepositoryMappingOptions } from "./ReadonlyRepositoryMappingOptions.ts";
import { EnrichmentSelection } from "./EnrichmentSelection.ts";

class Kind {
  Id = 0;
  Name = "";
}

class Child {
  Id = 0;
  ParentId = 0;
  Label = "";
  Notes: Note[] = [];
}

class Note {
  Id = 0;
  ChildId = 0;
  Text = "";
}

class Setting {
  Id = 0;
  ParentId = 0;
  Name = "";
  Value = "";
}

class GameAttribute {
  Id = 0;
  ParentId = 0;
  Name = "";
  Value = "";
}

class Tag {
  Id = 0;
  Name = "";
}

class ParentTag {
  Id = 0;
  ParentId = 0;
  TagId = 0;
}

class Mark {
  Id = 0;
  PlayId = 0;
  UserId = 0;
  Note = "";
}

class Seat {
  Id = 0;
  PlayId = 0;
  UserId = 0;
  Marks: Mark[] = [];
}

class Game {
  Id = 0;
  Name = "";
  Settings: Record<string, string> = {};
  Attributes: Record<string, string> = {};
}

class Parent {
  Id = 0;
  Kind: number | undefined = undefined;
  KindName = "";
  Children: Child[] = [];
  Settings: Record<string, string> = {};
  Tags: Tag[] = [];
}

const snake = new ChangeCasingMapper("pascalCase", "snakeCase");

const settingSchema = new Schema({
  Id: { type: "integer", nullable: false, hasDefault: false, isId: true },
  ParentId: { type: "integer", nullable: false, hasDefault: false },
  Name: { type: "string", nullable: false, hasDefault: false },
  Value: { type: "string", nullable: false, hasDefault: false },
});

const open = <T extends object>(type: new () => T, connection: IConnection, schema?: Schema) =>
  new CRUDRepository<T>(connection, {
    type,
    propertyMappings: snake,
    ids: ["Id"],
    schema: schema ?? new Schema({
      Id: { type: "integer", nullable: false, hasDefault: false, isId: true },
    }),
  });

const openConnection = async (): Promise<IConnection> => {
  const connection = new MemoryDriver().connection();
  await connection.open();
  return connection;
};

const assign = <T extends object>(type: new () => T, values: Partial<T>): T => Object.assign(new type(), values);

describe("LoadLevelOrderReadonlyEnrichmentStrategy", () => {
  it("keyOf matches int text and nested Id", () => {
    const keys = new DefaultReadonlyEnrichmentStrategy<Parent>();
    expect(keys.keyOf(1)).toBe("1");
    expect(keys.keyOf("1")).toBe("1");
    expect(keys.keyOf({ Id: 5, Name: "x" })).toBe("5");
  });

  it("parity object missing duplicate and null keys", async () => {
    const connection = await openConnection();
    const kindsRepo = open(Kind, connection);
    const parentsRepo = open(Parent, connection);
    const person = await kindsRepo.create(assign(Kind, { Name: "person" }));
    await kindsRepo.create(assign(Kind, { Name: "gone" }));
    const kept = await parentsRepo.create(assign(Parent, { Kind: person.Id }));
    const missing = await parentsRepo.create(assign(Parent, { Kind: 99 }));
    const duplicate = await parentsRepo.create(assign(Parent, { Kind: person.Id }));
    await parentsRepo.create(new Parent());
    const expected = await readKinds(connection, new DefaultReadonlyEnrichmentStrategy());
    const actual = await readKinds(connection, new LoadLevelOrderReadonlyEnrichmentStrategy());
    expect(actual.map((row) => row.KindName)).toEqual(expected.map((row) => row.KindName));
    expect(actual.find((row) => row.Id === kept.Id)?.KindName).toBe("person");
    expect(actual.find((row) => row.Id === missing.Id)?.KindName).toBe("");
    expect(actual.find((row) => row.Id === duplicate.Id)?.KindName).toBe("person");
    expect(actual.find((row) => row.Kind === undefined)?.KindName).toBe("");
  });

  it("parity list missing duplicate and null keys", async () => {
    const connection = await openConnection();
    const parentsRepo = open(Parent, connection);
    const childrenRepo = open(Child, connection);
    const shared = await parentsRepo.create(new Parent());
    const lonely = await parentsRepo.create(new Parent());
    await parentsRepo.create(new Parent());
    await childrenRepo.create(assign(Child, { ParentId: shared.Id, Label: "a" }));
    await childrenRepo.create(assign(Child, { ParentId: shared.Id, Label: "b" }));
    await childrenRepo.create(assign(Child, { ParentId: lonely.Id, Label: "x" }));
    const expected = await readChildren(connection, new DefaultReadonlyEnrichmentStrategy());
    const actual = await readChildren(connection, new LoadLevelOrderReadonlyEnrichmentStrategy());
    expect(actual.map((row) => row.Children.map((child) => child.Label))).toEqual(
      expected.map((row) => row.Children.map((child) => child.Label)),
    );
  });

  it("parity dictionary missing duplicate and null keys", async () => {
    const connection = await openConnection();
    const parentsRepo = open(Parent, connection);
    const settingsRepo = open(Setting, connection, settingSchema);
    const shared = await parentsRepo.create(new Parent());
    const empty = await parentsRepo.create(new Parent());
    await parentsRepo.create(new Parent());
    await settingsRepo.create(assign(Setting, { ParentId: shared.Id, Name: "theme", Value: "dark" }));
    await settingsRepo.create(assign(Setting, { ParentId: shared.Id, Name: "density", Value: "comfortable" }));
    const expected = await readSettings(connection, new DefaultReadonlyEnrichmentStrategy());
    const actual = await readSettings(connection, new LoadLevelOrderReadonlyEnrichmentStrategy());
    expect(actual.map(settingsOf)).toEqual(expected.map(settingsOf));
    expect(actual.find((row) => row.Id === shared.Id)?.Settings.theme).toBe("dark");
    expect(actual.find((row) => row.Id === empty.Id)?.Settings).toEqual({});
  });

  it("parity linked missing duplicate and null keys", async () => {
    const connection = await openConnection();
    const parentsRepo = open(Parent, connection);
    const tagsRepo = open(Tag, connection);
    const linksRepo = open(ParentTag, connection);
    const keep = await tagsRepo.create(assign(Tag, { Name: "keep" }));
    const drop = await tagsRepo.create(assign(Tag, { Name: "drop" }));
    const tagged = await parentsRepo.create(new Parent());
    const other = await parentsRepo.create(new Parent());
    await parentsRepo.create(new Parent());
    await linksRepo.create(assign(ParentTag, { ParentId: tagged.Id, TagId: keep.Id }));
    await linksRepo.create(assign(ParentTag, { ParentId: tagged.Id, TagId: drop.Id }));
    await linksRepo.create(assign(ParentTag, { ParentId: other.Id, TagId: keep.Id }));
    const expected = await readTags(connection, new DefaultReadonlyEnrichmentStrategy());
    const actual = await readTags(connection, new LoadLevelOrderReadonlyEnrichmentStrategy());
    expect(actual.map((row) => row.Tags.map((tag) => tag.Name))).toEqual(expected.map((row) => row.Tags.map((tag) => tag.Name)));
    expect(actual.find((row) => row.Id !== tagged.Id && row.Id !== other.Id)?.Tags).toEqual([]);
  });

  it("one hundred parents is one read per enrichment", async () => {
    const connection = await openConnection();
    const parentsRepo = open(Parent, connection);
    const childrenRepo = open(Child, connection);
    for (let i = 0; i < 100; i++) {
      const parent = await parentsRepo.create(new Parent());
      await childrenRepo.create(assign(Child, { ParentId: parent.Id, Label: `${i}` }));
    }
    const reads = { count: 0 };
    parentsRepo.addEnriched(
      { destinationProperty: "Children", sourceProperty: "Id", relatedKey: "ParentId", type: "list" },
      new CountingRepository(childrenRepo, reads),
      new LoadLevelOrderReadonlyEnrichmentStrategy(),
    );
    const items = await collectAsync(parentsRepo.read());
    expect(items).toHaveLength(100);
    expect(items.reduce((sum, row) => sum + row.Children.length, 0)).toBe(100);
    expect(reads.count).toBe(1);
  });

  it("zero queries when parents empty", async () => {
    const connection = await openConnection();
    const reads = { count: 0 };
    await new LoadLevelOrderReadonlyEnrichmentStrategy<Parent>().apply(
      new CountingRepository(open(Child, connection), reads),
      { destinationProperty: "Children", sourceProperty: "Id", relatedKey: "ParentId", type: "list" },
      [],
    );
    expect(reads.count).toBe(0);
  });

  it("zero queries when keys null", async () => {
    const connection = await openConnection();
    const reads = { count: 0 };
    await new LoadLevelOrderReadonlyEnrichmentStrategy<Parent>().apply(
      new CountingRepository(open(Kind, connection), reads),
      { destinationProperty: "KindName", sourceProperty: "Kind", relatedKey: "Id", relatedProperty: "Name" },
      [new Parent(), new Parent()],
    );
    expect(reads.count).toBe(0);
  });

  it("chunks ten keys by three", async () => {
    const connection = await openConnection();
    const parentsRepo = open(Parent, connection);
    const childrenRepo = open(Child, connection);
    for (let i = 0; i < 10; i++) {
      const parent = await parentsRepo.create(new Parent());
      await childrenRepo.create(assign(Child, { ParentId: parent.Id, Label: `${i}` }));
    }
    const reads = { count: 0 };
    parentsRepo.addEnriched(
      { destinationProperty: "Children", sourceProperty: "Id", relatedKey: "ParentId", type: "list" },
      new CountingRepository(childrenRepo, reads),
      new LoadLevelOrderReadonlyEnrichmentStrategy<Parent>().withBatchSize(3),
    );
    const items = await collectAsync(parentsRepo.read());
    expect(items.reduce((sum, row) => sum + row.Children.length, 0)).toBe(10);
    expect(reads.count).toBe(4);
  });

  it("nested parent child grandchild is three queries", async () => {
    const connection = await openConnection();
    const parentsRepo = open(Parent, connection);
    const childrenRepo = open(Child, connection);
    const notesRepo = open(Note, connection);
    const parent = await parentsRepo.create(new Parent());
    const child = await childrenRepo.create(assign(Child, { ParentId: parent.Id, Label: "a" }));
    await notesRepo.create(assign(Note, { ChildId: child.Id, Text: "n" }));
    const reads = { count: 0 };
    childrenRepo.addEnriched(
      { destinationProperty: "Notes", sourceProperty: "Id", relatedKey: "ChildId", type: "list" },
      new CountingRepository(notesRepo, reads),
      new LoadLevelOrderReadonlyEnrichmentStrategy(),
    );
    parentsRepo.addEnriched(
      { destinationProperty: "Children", sourceProperty: "Id", relatedKey: "ParentId", type: "list" },
      new CountingRepository(childrenRepo, reads),
      new LoadLevelOrderReadonlyEnrichmentStrategy(),
    );
    const item = (await collectAsync(new CountingRepository(parentsRepo, reads).read()))[0];
    expect(item?.Children[0]?.Notes[0]?.Text).toBe("n");
    expect(reads.count).toBe(3);
  });

  it("composite keys match all pairs", async () => {
    const connection = await openConnection();
    const seatsRepo = open(Seat, connection);
    const marksRepo = open(Mark, connection);
    const left = await seatsRepo.create(assign(Seat, { PlayId: 1, UserId: 10 }));
    const right = await seatsRepo.create(assign(Seat, { PlayId: 1, UserId: 11 }));
    await marksRepo.create(assign(Mark, { PlayId: 1, UserId: 10, Note: "left" }));
    await marksRepo.create(assign(Mark, { PlayId: 1, UserId: 11, Note: "right" }));
    const reads = { count: 0 };
    seatsRepo.addEnriched(
      {
        destinationProperty: "Marks",
        sourceProperty: "PlayId",
        relatedKey: "PlayId",
        type: "list",
        keys: [
          { sourceProperty: "PlayId", relatedKey: "PlayId" },
          { sourceProperty: "UserId", relatedKey: "UserId" },
        ],
      },
      new CountingRepository(marksRepo, reads),
      new LoadLevelOrderReadonlyEnrichmentStrategy(),
    );
    const items = await collectAsync(seatsRepo.read());
    expect(items.find((row) => row.Id === left.Id)?.Marks.map((mark) => mark.Note)).toEqual(["left"]);
    expect(items.find((row) => row.Id === right.Id)?.Marks.map((mark) => mark.Note)).toEqual(["right"]);
    expect(reads.count).toBe(1);
  });

  it("linked two queries and empty when unlinked", async () => {
    const connection = await openConnection();
    const parentsRepo = open(Parent, connection);
    const tagsRepo = open(Tag, connection);
    const linksRepo = open(ParentTag, connection);
    const keep = await tagsRepo.create(assign(Tag, { Name: "keep" }));
    const tagged = await parentsRepo.create(new Parent());
    const bare = await parentsRepo.create(new Parent());
    await linksRepo.create(assign(ParentTag, { ParentId: tagged.Id, TagId: keep.Id }));
    const reads = { count: 0 };
    parentsRepo.addLinked(
      { destinationProperty: "Tags", sourceProperty: "Id", relatedKey: "Id", association: { parent: "ParentId", related: "TagId" } },
      new CountingRepository(tagsRepo, reads),
      new CountingCRUDRepository(linksRepo, reads),
      new LoadLevelOrderReadonlyEnrichmentStrategy(),
    );
    const items = await collectAsync(parentsRepo.read());
    expect(items.find((row) => row.Id === tagged.Id)?.Tags.map((tag) => tag.Name)).toEqual(["keep"]);
    expect(items.find((row) => row.Id === bare.Id)?.Tags).toEqual([]);
    expect(reads.count).toBe(2);
  });

  it("one hundred twenty one games three settings one attribute is three queries", async () => {
    await assertDictionaryQueryCount(await seedGames(), new LoadLevelOrderReadonlyEnrichmentStrategy<Game>());
  });

  it("omitted strategy dictionary is one read per enrichment", async () => {
    await assertDictionaryQueryCount(await seedGames());
  });

  it("default strategy dictionary is one read per enrichment", async () => {
    await assertDictionaryQueryCount(await seedGames(), new DefaultReadonlyEnrichmentStrategy<Game>());
  });

  it("enrich only settings skips other fills", async () => {
    const seeded = await seedGames();
    const settingsReads = { count: 0 };
    const attributeReads = { count: 0 };
    seeded.gamesRepo.addEnriched(
      { destinationProperty: "Settings", sourceProperty: "Id", relatedKey: "ParentId", relatedProperty: "Value", dictionaryKey: "Name", type: "dictionary" },
      new CountingRepository(seeded.settingsRepo, settingsReads),
    );
    seeded.gamesRepo.addEnriched(
      { destinationProperty: "Attributes", sourceProperty: "Id", relatedKey: "ParentId", relatedProperty: "Value", dictionaryKey: "Name", type: "dictionary" },
      new CountingRepository(seeded.attributesRepo, attributeReads),
    );
    const items = await collectAsync(
      seeded.gamesRepo.read(undefined, undefined, { enrich: EnrichmentSelection.only("Settings") }),
    );
    expect(items).toHaveLength(121);
    for (const game of items) expect(Object.keys(game.Settings)).toHaveLength(3);
    expect(settingsReads.count).toBe(1);
    expect(attributeReads.count).toBe(0);
  });

  it("default zero queries when parents empty", async () => {
    const connection = await openConnection();
    const reads = { count: 0 };
    await new DefaultReadonlyEnrichmentStrategy<Parent>().apply(
      new CountingRepository(open(Child, connection), reads),
      { destinationProperty: "Children", sourceProperty: "Id", relatedKey: "ParentId", type: "list" },
      [],
    );
    expect(reads.count).toBe(0);
  });

  it("default zero queries when keys null", async () => {
    const connection = await openConnection();
    const reads = { count: 0 };
    await new DefaultReadonlyEnrichmentStrategy<Parent>().apply(
      new CountingRepository(open(Kind, connection), reads),
      { destinationProperty: "KindName", sourceProperty: "Kind", relatedKey: "Id", relatedProperty: "Name" },
      [new Parent(), new Parent()],
    );
    expect(reads.count).toBe(0);
  });
});

const seedGames = async () => {
  const connection = await openConnection();
  const gamesRepo = open(Game, connection);
  const settingsRepo = open(Setting, connection, settingSchema);
  const attributesRepo = open(GameAttribute, connection, settingSchema);
  for (let i = 0; i < 121; i++) {
    const game = await gamesRepo.create(assign(Game, { Name: `g${i}` }));
    await settingsRepo.create(assign(Setting, { ParentId: game.Id, Name: "a", Value: "1" }));
    await settingsRepo.create(assign(Setting, { ParentId: game.Id, Name: "b", Value: "2" }));
    await settingsRepo.create(assign(Setting, { ParentId: game.Id, Name: "c", Value: "3" }));
    await attributesRepo.create(assign(GameAttribute, { ParentId: game.Id, Name: "theme", Value: "dark" }));
  }
  return { gamesRepo, settingsRepo, attributesRepo };
};

const assertDictionaryQueryCount = async (
  seeded: Awaited<ReturnType<typeof seedGames>>,
  strategy?: IReadonlyEnrichmentStrategy<Game>,
) => {
  const reads = { count: 0 };
  seeded.gamesRepo.addEnriched(
    { destinationProperty: "Settings", sourceProperty: "Id", relatedKey: "ParentId", relatedProperty: "Value", dictionaryKey: "Name", type: "dictionary" },
    new CountingRepository(seeded.settingsRepo, reads),
    strategy,
  );
  seeded.gamesRepo.addEnriched(
    { destinationProperty: "Attributes", sourceProperty: "Id", relatedKey: "ParentId", relatedProperty: "Value", dictionaryKey: "Name", type: "dictionary" },
    new CountingRepository(seeded.attributesRepo, reads),
    strategy,
  );
  const items = await collectAsync(new CountingRepository(seeded.gamesRepo, reads).read());
  expect(items).toHaveLength(121);
  for (const game of items) {
    expect(Object.keys(game.Settings)).toHaveLength(3);
    expect(game.Attributes.theme).toBe("dark");
  }
  expect(reads.count).toBe(3);
};

const readKinds = (connection: IConnection, strategy: IReadonlyEnrichmentStrategy<Parent>) => {
  const clone = open(Parent, connection);
  clone.addEnriched(
    { destinationProperty: "KindName", sourceProperty: "Kind", relatedKey: "Id", relatedProperty: "Name" },
    open(Kind, connection),
    strategy,
  );
  return collectAsync(clone.read());
};

const readChildren = (connection: IConnection, strategy: IReadonlyEnrichmentStrategy<Parent>) => {
  const clone = open(Parent, connection);
  clone.addEnriched(
    { destinationProperty: "Children", sourceProperty: "Id", relatedKey: "ParentId", type: "list" },
    open(Child, connection),
    strategy,
  );
  return collectAsync(clone.read());
};

const readSettings = (connection: IConnection, strategy: IReadonlyEnrichmentStrategy<Parent>) => {
  const clone = open(Parent, connection);
  clone.addEnriched(
    { destinationProperty: "Settings", sourceProperty: "Id", relatedKey: "ParentId", relatedProperty: "Value", dictionaryKey: "Name", type: "dictionary" },
    open(Setting, connection, settingSchema),
    strategy,
  );
  return collectAsync(clone.read());
};

const readTags = (connection: IConnection, strategy: IReadonlyEnrichmentStrategy<Parent>) => {
  const clone = open(Parent, connection);
  clone.addLinked(
    { destinationProperty: "Tags", sourceProperty: "Id", relatedKey: "Id", association: { parent: "ParentId", related: "TagId" } },
    open(Tag, connection),
    open(ParentTag, connection),
    strategy,
  );
  return collectAsync(clone.read());
};

const settingsOf = (parent: Parent): Record<string, string> =>
  Object.fromEntries(Object.entries(parent.Settings).sort(([left], [right]) => left.localeCompare(right)));

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
