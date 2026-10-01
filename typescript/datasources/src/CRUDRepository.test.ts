import { describe, expect, it } from "vitest";
import { MemoryDriver } from "@datasources/memory";
import { collectAsync } from "./collectAsync.ts";
import { CRUDRepository } from "./CRUDRepository.ts";
import { CascadeDeleteUpdateEnrichmentStrategy } from "./CascadeDeleteUpdateEnrichmentStrategy.ts";
import { DefaultWriteEnrichmentStrategy } from "./DefaultWriteEnrichmentStrategy.ts";
import { ReadonlyRepository } from "./ReadonlyRepository.ts";
import type { IConnection } from "./IConnection.ts";
import type { IReadonlyRepository } from "./IReadonlyRepository.ts";
import type { IWriteEnrichmentStrategy } from "./IWriteEnrichmentStrategy.ts";
import type { Scalar } from "./builders/Scalar.ts";
import { DeleteEnrichmentStrategy } from "./DeleteEnrichmentStrategy.ts";
import { ChangeCasingMapper } from "./mappers/ChangeCasingMapper.ts";
import { DictionaryMapper } from "./mappers/DictionaryMapper.ts";
import type { IMapper } from "./mappers/IMapper.ts";
import { TransformMapper } from "./mappers/TransformMapper.ts";

class UserType {
  Id = 0;
  Name = "";
}

class Parent {
  Id = 0;
  Type = new UserType();
  TypeName = "";
  Children: Child[] = [];
}

class Child {
  Id = 0;
  ParentId = 0;
  Type = new UserType();
  TypeName = "";
  Label = "";
  Notes: Note[] = [];
}

class Note {
  Id = 0;
  ChildId = 0;
  Text = "";
}

class Tagged {
  Id = 0;
  Label = "";
  Tags: Tag[] = [];
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

class SkipDelete<T extends object> extends DeleteEnrichmentStrategy<T> {
  deleteOwned(): Promise<void> {
    return Promise.resolve();
  }

  deleteLinked(): Promise<void> {
    return Promise.resolve();
  }
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

const repo = <T extends object>(
  type: new () => T,
  connection: IConnection,
  write?: IWriteEnrichmentStrategy<T>,
) => new CRUDRepository<T>(connection, { type, propertyMappings: columns, write });

const read = <T extends object>(type: new () => T, connection: IConnection) =>
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

const relate = (
  parentsRepo: CRUDRepository<Parent>,
  typesRepo: IReadonlyRepository<UserType>,
  childrenRepo: IReadonlyRepository<Child>,
) => {
  parentsRepo.addEnriched({ destinationProperty: "TypeName", sourceProperty: "Type", relatedKey: "Id", relatedProperty: "Name" }, typesRepo);
  parentsRepo.addEnriched({ destinationProperty: "Children", sourceProperty: "Id", relatedKey: "ParentId", type: "list" }, childrenRepo);
};

describe("CRUDRepository", () => {
  it("creates children, stamps FK, and skips select", async () => {
    const connection = await setup();
    const parentsRepo = repo(Parent, connection);
    const typesRepo = repo(UserType, connection);
    const childrenRepo = repo(Child, connection);
    const type = await typesRepo.create(Object.assign(new UserType(), { Name: "person" }));
    expect(type.Id).toBeGreaterThan(0);
    relate(parentsRepo, typesRepo, childrenRepo);
    const created = await parentsRepo.create(Object.assign(new Parent(), {
      Type: Object.assign(new UserType(), { Id: type.Id }),
      Children: [
        Object.assign(new Child(), { Label: "a" }),
        Object.assign(new Child(), { Label: "b" }),
      ],
    }));
    expect(created.Id).toBeGreaterThan(0);
    const parent = (await collectAsync(parentsRepo.read()))[0];
    const children = await collectAsync(childrenRepo.read());
    expect(parent?.Id).toBe(created.Id);
    expect(children).toHaveLength(2);
    expect(children.every((child) => child.ParentId === parent?.Id)).toBe(true);
    expect(children.map((child) => child.Label)).toEqual(["a", "b"]);
    expect((await typesRepo.readSingle())?.Name).toBe("person");
  });

  it("applies generated parent id before child FKs", async () => {
    const connection = await setup();
    const parentsRepo = repo(Parent, connection);
    const childrenRepo = repo(Child, connection);
    parentsRepo.addEnriched({ destinationProperty: "Children", sourceProperty: "Id", relatedKey: "ParentId", type: "list" }, childrenRepo);
    const created = await parentsRepo.create(Object.assign(new Parent(), {
      Children: [Object.assign(new Child(), { Label: "n" })],
    }));
    expect(created.Id).toBeGreaterThan(0);
    const parent = await parentsRepo.readSingle();
    const child = await childrenRepo.readSingle();
    expect(parent?.Id).toBe(created.Id);
    expect(child?.ParentId).toBe(parent?.Id);
  });

  it("update replace-syncs children", async () => {
    const connection = await setup();
    const parentsRepo = repo(Parent, connection);
    const childrenRepo = repo(Child, connection);
    const parent = await parentsRepo.create(new Parent());
    expect(parent.Id).toBeGreaterThan(0);
    const kept = await childrenRepo.create(Object.assign(new Child(), { ParentId: parent.Id, Label: "a" }));
    expect(kept.Id).toBeGreaterThan(0);
    const dropped = await childrenRepo.create(Object.assign(new Child(), { ParentId: parent.Id, Label: "b" }));
    expect(dropped.Id).toBeGreaterThan(0);
    parentsRepo.addEnriched({ destinationProperty: "Children", sourceProperty: "Id", relatedKey: "ParentId", type: "list" }, childrenRepo);
    const updated = await parentsRepo.update(Object.assign(new Parent(), {
      Id: parent.Id,
      Children: [
        Object.assign(new Child(), { Id: kept.Id, Label: "aa" }),
        Object.assign(new Child(), { Label: "c" }),
      ],
    }));
    expect(updated.rowsAffected).toBe(1);
    const children = await collectAsync(childrenRepo.read());
    expect(children).toHaveLength(2);
    expect(children.find((child) => child.Id === kept?.Id)?.Label).toBe("aa");
    expect(children.find((child) => child.Id !== kept?.Id)?.Label).toBe("c");
    expect(children.every((child) => child.ParentId === parent?.Id)).toBe(true);
  });

  it("cascade update deletes children and recreates", async () => {
    const connection = await setup();
    const parentsRepo = repo(
      Parent,
      connection,
      new DefaultWriteEnrichmentStrategy(undefined, new CascadeDeleteUpdateEnrichmentStrategy()),
    );
    const childrenRepo = repo(Child, connection);
    const notesRepo = repo(Note, connection);
    const parent = await parentsRepo.create(new Parent());
    expect(parent.Id).toBeGreaterThan(0);
    const child = await childrenRepo.create(Object.assign(new Child(), { ParentId: parent.Id, Label: "a" }));
    expect(child.Id).toBeGreaterThan(0);
    const other = await childrenRepo.create(Object.assign(new Child(), { Label: "other" }));
    expect(other.Id).toBeGreaterThan(0);
    const note = await notesRepo.create(Object.assign(new Note(), { ChildId: child.Id, Text: "n" }));
    expect(note.Id).toBeGreaterThan(0);
    childrenRepo.addEnriched({ destinationProperty: "Notes", sourceProperty: "Id", relatedKey: "ChildId", type: "list" }, notesRepo);
    parentsRepo.addEnriched({ destinationProperty: "Children", sourceProperty: "Id", relatedKey: "ParentId", type: "list" }, childrenRepo);
    const updated = await parentsRepo.update(Object.assign(new Parent(), {
      Id: parent.Id,
      Children: [Object.assign(new Child(), { Id: child.Id, Label: "fresh" })],
    }));
    expect(updated.rowsAffected).toBe(1);
    const created = (await collectAsync(childrenRepo.read())).find((row) => row.Label === "fresh");
    expect(created?.Id).not.toBe(child?.Id);
    expect(created?.Label).toBe("fresh");
    expect(created?.ParentId).toBe(parent?.Id);
    expect(await collectAsync(notesRepo.read())).toEqual([]);
  });

  it("cascade update replaces links and keeps related rows", async () => {
    const connection = await setup();
    const parentsRepo = repo(
      Tagged,
      connection,
      new DefaultWriteEnrichmentStrategy(undefined, new CascadeDeleteUpdateEnrichmentStrategy()),
    );
    const tagsRepo = repo(Tag, connection);
    const linksRepo = repo(ParentTag, connection);
    const parent = await parentsRepo.create(new Tagged());
    expect(parent.Id).toBeGreaterThan(0);
    const keep = await tagsRepo.create(Object.assign(new Tag(), { Name: "keep" }));
    expect(keep.Id).toBeGreaterThan(0);
    const drop = await tagsRepo.create(Object.assign(new Tag(), { Name: "drop" }));
    expect(drop.Id).toBeGreaterThan(0);
    const oldLink = await linksRepo.create(Object.assign(new ParentTag(), { ParentId: parent.Id, TagId: keep.Id }));
    expect(oldLink.Id).toBeGreaterThan(0);
    const droppedLink = await linksRepo.create(Object.assign(new ParentTag(), { ParentId: parent.Id, TagId: drop.Id }));
    expect(droppedLink.Id).toBeGreaterThan(0);
    const leftover = await linksRepo.create(Object.assign(new ParentTag(), { TagId: keep.Id }));
    expect(leftover.Id).toBeGreaterThan(0);
    parentsRepo.addLinked(
      { destinationProperty: "Tags", sourceProperty: "Id", relatedKey: "Id", association: { parent: "ParentId", related: "TagId" } },
      tagsRepo,
      linksRepo,
    );
    const updated = await parentsRepo.update(Object.assign(new Tagged(), {
      Id: parent.Id,
      Label: "p",
      Tags: [Object.assign(new Tag(), { Id: keep.Id })],
    }));
    expect(updated.rowsAffected).toBe(1);
    const links = (await collectAsync(linksRepo.read())).filter((link) => link.ParentId === parent.Id);
    expect(links).toHaveLength(1);
    expect(links[0]?.TagId).toBe(keep.Id);
    expect(links[0]?.ParentId).toBe(parent.Id);
    expect(links[0]?.Id).not.toBe(oldLink?.Id);
    expect(await collectAsync(tagsRepo.read())).toHaveLength(2);
  });

  it("delete cascades children then parent", async () => {
    const connection = await setup();
    const parentsRepo = repo(Parent, connection);
    const childrenRepo = repo(Child, connection);
    const notesRepo = repo(Note, connection);
    const parent = await parentsRepo.create(new Parent());
    expect(parent.Id).toBeGreaterThan(0);
    const child = await childrenRepo.create(Object.assign(new Child(), { ParentId: parent.Id, Label: "a" }));
    expect(child.Id).toBeGreaterThan(0);
    const note = await notesRepo.create(Object.assign(new Note(), { ChildId: child.Id, Text: "n" }));
    expect(note.Id).toBeGreaterThan(0);
    const other = await notesRepo.create(Object.assign(new Note(), { Text: "other" }));
    expect(other.Id).toBeGreaterThan(0);
    childrenRepo.addEnriched({ destinationProperty: "Notes", sourceProperty: "Id", relatedKey: "ChildId", type: "list" }, notesRepo);
    parentsRepo.addEnriched({ destinationProperty: "Children", sourceProperty: "Id", relatedKey: "ParentId", type: "list" }, childrenRepo);
    const removed = await parentsRepo.delete(Object.assign(new Parent(), { Id: parent.Id }));
    expect(removed.rowsAffected).toBe(1);
    expect(await collectAsync(parentsRepo.read())).toEqual([]);
    expect(await collectAsync(childrenRepo.read())).toEqual([]);
    expect((await notesRepo.readSingle())?.Text).toBe("other");
  });

  it("delete strategy can skip children", async () => {
    const connection = await setup();
    const childrenRepo = repo(Child, connection);
    const parentsRepo = repo(Parent, connection, new DefaultWriteEnrichmentStrategy(new SkipDelete()));
    const parent = await parentsRepo.create(new Parent());
    expect(parent.Id).toBeGreaterThan(0);
    const child = await childrenRepo.create(Object.assign(new Child(), { ParentId: parent.Id, Label: "a" }));
    expect(child.Id).toBeGreaterThan(0);
    parentsRepo.addEnriched({ destinationProperty: "Children", sourceProperty: "Id", relatedKey: "ParentId", type: "list" }, childrenRepo);
    const removed = await parentsRepo.delete(Object.assign(new Parent(), { Id: parent.Id }));
    expect(removed.rowsAffected).toBe(1);
    expect(await collectAsync(parentsRepo.read())).toEqual([]);
    expect((await childrenRepo.readSingle())?.Label).toBe("a");
  });

  it("writes nested many and skips nested select", async () => {
    const connection = await setup();
    const parentsRepo = repo(Parent, connection);
    const childrenRepo = repo(Child, connection);
    const notesRepo = repo(Note, connection);
    const childTypesRepo = repo(UserType, connection);
    const home = await childTypesRepo.create(Object.assign(new UserType(), { Name: "home" }));
    expect(home.Id).toBeGreaterThan(0);
    childrenRepo.addEnriched({ destinationProperty: "TypeName", sourceProperty: "Type", relatedKey: "Id", relatedProperty: "Name" }, childTypesRepo);
    childrenRepo.addEnriched({ destinationProperty: "Notes", sourceProperty: "Id", relatedKey: "ChildId", type: "list" }, notesRepo);
    parentsRepo.addEnriched({ destinationProperty: "Children", sourceProperty: "Id", relatedKey: "ParentId", type: "list" }, childrenRepo);
    const created = await parentsRepo.create(Object.assign(new Parent(), {
      Children: [Object.assign(new Child(), {
        Type: Object.assign(new UserType(), { Id: home.Id }),
        Label: "a",
        Notes: [Object.assign(new Note(), { Text: "hi" })],
      })],
    }));
    expect(created.Id).toBeGreaterThan(0);
    const note = await notesRepo.readSingle();
    const child = await childrenRepo.readSingle();
    expect(note?.Text).toBe("hi");
    expect(note?.ChildId).toBe(child?.Id);
    expect((await childTypesRepo.readSingle())?.Name).toBe("home");
  });

  it("throws when many via is readonly", async () => {
    const connection = await setup();
    const parentsRepo = repo(Parent, connection);
    parentsRepo.addEnriched(
      { destinationProperty: "Children", sourceProperty: "Id", relatedKey: "ParentId", type: "list" },
      read(Child, connection),
    );
    await expect(parentsRepo.create(Object.assign(new Parent(), {
      Children: [Object.assign(new Child(), { Label: "a" })],
    }))).rejects.toThrow(/Children/);
  });

  it("get still enriches", async () => {
    const org: Seed = { name: "org" };
    const typesConnection = await setup([org]);
    const parent: Seed = { type_id: org.id };
    const connection = await setup([parent]);
    const parentsRepo = repo(Parent, connection);
    const childrenConnection = await setup([{ parent_id: parent.id, label: "a" }]);
    relate(
      parentsRepo,
      read(UserType, typesConnection),
      read(Child, childrenConnection),
    );
    const item = await parentsRepo.readSingle();
    expect(item?.TypeName).toBe("org");
    expect(item?.Children.map((child) => child.Label)).toEqual(["a"]);
  });
});
