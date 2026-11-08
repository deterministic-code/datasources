import { describe, expect, it } from "vitest";
import { MemoryDriver } from "@deterministic-code/datasources-memory";
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
import { Schema } from "./Schema.ts";

class UserType {
  Id = 0;
  Name = "";
}

class Pref {
  Locale = "";
  Timezone = "";
}

class Setting {
  Id = 0;
  ParentId = 0;
  Key = "";
  Value = "";
}

class PrefRow {
  Id = 0;
  ParentId = 0;
  Key = "";
  Locale = "";
  Timezone = "";
}

class Host {
  Id = 0;
  Label = "";
  Settings: Record<string, string> = {};
  Prefs: Record<string, Pref> = {};
}

class NamedSetting {
  Id = 0;
  ParentId = 0;
  Name = "";
  Value = "";
}

class NamedHost {
  Id = 0;
  Label = "";
  Settings: Record<string, string> = {};
}

class NamedSettingRow {
  id!: number;
  parentId!: number;
  name!: string;
  value!: string;
}

class NamedHostRow {
  id!: number;
  label!: string;
  settings: Record<string, string> = {};
}

class AsdfSettingRow {
  id!: number;
  parentId!: number;
  asdfasdfa!: string;
  value!: string;
}

class KeyedSettingRow {
  id!: number;
  parentId!: number;
  key!: string;
  value!: string;
}

class PrefValue {
  locale!: string;
  timezone!: string;
}

class PrefSettingRow {
  id!: number;
  parentId!: number;
  name!: string;
  locale!: string;
  timezone!: string;
}

class PrefHostRow {
  id!: number;
  label!: string;
  prefs: Record<string, PrefValue> = {};
}

class ChildRow {
  id!: number;
  parentId!: number;
  label!: string;
}

class ParentRow {
  id!: number;
  label!: string;
  children: ChildRow[] = [];
}

class TagRow {
  id!: number;
  name!: string;
}

class ParentTagRow {
  id!: number;
  parentId!: number;
  tagId!: number;
}

class TaggedRow {
  id!: number;
  label!: string;
  tags: TagRow[] = [];
}

class TypeRow {
  id!: number;
  name!: string;
}

class TypedRow {
  id!: number;
  typeId!: number;
  typeName = "";
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

class Named {
  Id = 0;
  TypeName = "";
}

class SkipDelete<T extends object> extends DeleteEnrichmentStrategy<T> {
  deleteChildren(): Promise<void> {
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
) =>
  new CRUDRepository<T>(connection, {
    type,
    propertyMappings: columns,
    write,
    ids: ["Id"],
    schema: new Schema({
      Id: { type: "integer", nullable: false, hasDefault: false, isId: true },
    }),
  });

const camel = new ChangeCasingMapper("camelCase", "snakeCase");

const generatedRepo = <T extends object>(connection: IConnection, entity: string, fields: string[]) =>
  new CRUDRepository<T>(connection, {
    entity,
    entityMapper: new DictionaryMapper([[entity, `${entity.toLowerCase()}s`]]),
    propertyMappings: camel,
    ids: ["id"],
    createProperties: fields.filter((name) => name !== "id"),
    updateProperties: fields.filter((name) => name !== "id"),
    readProperties: fields,
    schema: new Schema(
      Object.fromEntries(
        fields.map((name) => [
          name,
          {
            type: name === "id" || name.endsWith("Id") ? "integer" : "string",
            nullable: false,
            hasDefault: false,
            isId: name === "id",
          },
        ]),
      ),
    ),
  });

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
  it("opens a closed connection on create and read", async () => {
    const connection = new MemoryDriver().connection();
    expect(connection.isOpen).toBe(false);
    const created = await new CRUDRepository(connection, {}).create({ id: 1, name: "Ada" });
    expect(connection.isOpen).toBe(true);
    expect(created.id).toBe(1);
    const closed = new MemoryDriver().connection();
    expect(await collectAsync(new ReadonlyRepository(closed, {}).read())).toEqual([]);
    expect(closed.isOpen).toBe(true);
  });

  it("update keeps an explicit empty string on a nullable column", async () => {
    const connection = new MemoryDriver().connection();
    const repo = new CRUDRepository(connection, {
      schema: new Schema({
        id: { type: "integer", nullable: false, hasDefault: false },
        note: { type: "string", nullable: true, hasDefault: false },
      }),
      updateProperties: ["note"],
    });
    await repo.create({ id: 1, note: "side" });
    await repo.update({ id: 1, note: "" });
    expect((await repo.readSingle())?.note).toBe("");
  });

  it("round-trips string and object dictionaries", async () => {
    const connection = await setup();
    const hostsRepo = repo(Host, connection);
    const settingsRepo = repo(Setting, connection);
    const prefsRepo = repo(PrefRow, connection);
    hostsRepo.addEnriched(
      { destinationProperty: "Settings", sourceProperty: "Id", relatedKey: "ParentId", relatedProperty: "Value", type: "dictionary" },
      settingsRepo,
    );
    hostsRepo.addEnriched(
      { destinationProperty: "Prefs", sourceProperty: "Id", relatedKey: "ParentId", type: "dictionary" },
      prefsRepo,
    );
    const created = await hostsRepo.create(
      Object.assign(new Host(), {
        Label: "ada",
        Settings: { theme: "dark", density: "comfortable" },
        Prefs: {
          home: Object.assign(new Pref(), { Locale: "en-GB", Timezone: "Europe/London" }),
          travel: Object.assign(new Pref(), { Locale: "en-US", Timezone: "America/New_York" }),
        },
      }),
    );
    expect(created.Id).toBeGreaterThan(0);
    const loaded = await hostsRepo.readSingle();
    expect(loaded?.Settings).toEqual({ theme: "dark", density: "comfortable" });
    expect(loaded?.Prefs.home.Locale).toBe("en-GB");
    expect(loaded?.Prefs.home.Timezone).toBe("Europe/London");
    expect(loaded?.Prefs.travel.Locale).toBe("en-US");
    expect(loaded?.Prefs.travel.Timezone).toBe("America/New_York");
    await hostsRepo.update(
      Object.assign(new Host(), {
        Id: created.Id,
        Label: "ada",
        Settings: { theme: "light", locale: "en" },
        Prefs: {
          home: Object.assign(new Pref(), { Locale: "en-GB", Timezone: "Europe/Paris" }),
          work: Object.assign(new Pref(), { Locale: "en-US", Timezone: "America/New_York" }),
        },
      }),
    );
    const updated = await hostsRepo.readSingle();
    expect(updated?.Settings).toEqual({ theme: "light", locale: "en" });
    expect(updated?.Prefs.home.Locale).toBe("en-GB");
    expect(updated?.Prefs.home.Timezone).toBe("Europe/Paris");
    expect(updated?.Prefs.work.Locale).toBe("en-US");
    expect(updated?.Prefs.work.Timezone).toBe("America/New_York");
    expect(updated?.Prefs.travel).toBeUndefined();
  });

  it("round-trips a dictionary when the child key is Name", async () => {
    const connection = await setup();
    const hostsRepo = repo(NamedHost, connection);
    const settingsRepo = repo(NamedSetting, connection);
    hostsRepo.addEnriched(
      { destinationProperty: "Settings", sourceProperty: "Id", relatedKey: "ParentId", relatedProperty: "Value", type: "dictionary" },
      settingsRepo,
    );
    const created = await hostsRepo.create(
      Object.assign(new NamedHost(), { Label: "ada", Settings: { theme: "dark" } }),
    );
    expect(created.Id).toBeGreaterThan(0);
    const stored = (await collectAsync(settingsRepo.read()))[0];
    expect(stored?.Name).toBe("theme");
    expect(stored?.Value).toBe("dark");
    const loaded = await hostsRepo.readSingle();
    expect(loaded?.Settings).toEqual({ theme: "dark" });
  });

  it("reads a name-keyed dictionary from child rows", async () => {
    const connection = await setup();
    const hostsRepo = generatedRepo<NamedHostRow>(connection, "Host", ["id", "label"]);
    const settingsRepo = generatedRepo<NamedSettingRow>(connection, "Setting", ["id", "parentId", "name", "value"]);
    hostsRepo.addEnriched(
      { destinationProperty: "settings", sourceProperty: "id", relatedKey: "parentId", relatedProperty: "value", type: "dictionary" },
      settingsRepo,
    );
    const created = await hostsRepo.create({ label: "ada", settings: {} } as NamedHostRow);
    await settingsRepo.create({ parentId: created.id, name: "lives", value: "3" } as NamedSettingRow);
    await settingsRepo.create({ parentId: created.id, name: "mode", value: "arcade" } as NamedSettingRow);
    expect((await hostsRepo.readSingle())?.settings).toEqual({ lives: "3", mode: "arcade" });
  });

  it("reads a dictionary keyed by asdfasdfa, not name", async () => {
    const connection = await setup();
    const hostsRepo = generatedRepo<NamedHostRow>(connection, "Host", ["id", "label"]);
    const settingsRepo = generatedRepo<AsdfSettingRow>(connection, "Setting", ["id", "parentId", "asdfasdfa", "value"]);
    hostsRepo.addEnriched(
      { destinationProperty: "settings", sourceProperty: "id", relatedKey: "parentId", relatedProperty: "value", type: "dictionary" },
      settingsRepo,
    );
    const created = await hostsRepo.create({ label: "ada", settings: { alpha: "one", beta: "two" } } as NamedHostRow);
    const stored = await collectAsync(settingsRepo.read());
    expect(stored.map((row) => row.asdfasdfa).sort()).toEqual(["alpha", "beta"]);
    expect(stored.find((row) => row.asdfasdfa === "alpha")?.value).toBe("one");
    expect((created as { name?: string }).name).toBeUndefined();
    expect((await hostsRepo.readSingle())?.settings).toEqual({ alpha: "one", beta: "two" });
  });

  it("updates a name-keyed dictionary by key and keeps surviving ids", async () => {
    const connection = await setup();
    const hostsRepo = generatedRepo<NamedHostRow>(connection, "Host", ["id", "label"]);
    const settingsRepo = generatedRepo<NamedSettingRow>(connection, "Setting", ["id", "parentId", "name", "value"]);
    hostsRepo.addEnriched(
      { destinationProperty: "settings", sourceProperty: "id", relatedKey: "parentId", relatedProperty: "value", type: "dictionary" },
      settingsRepo,
    );
    const created = await hostsRepo.create({ label: "ada", settings: {} } as NamedHostRow);
    const lives = await settingsRepo.create({ parentId: created.id, name: "lives", value: "3" } as NamedSettingRow);
    await settingsRepo.create({ parentId: created.id, name: "mode", value: "arcade" } as NamedSettingRow);
    const loaded = await hostsRepo.readSingle();
    await hostsRepo.update({ id: created.id, label: "ada", settings: { lives: "5", density: "high" } } as NamedHostRow);
    const rows = await collectAsync(settingsRepo.read());
    expect(rows).toHaveLength(2);
    expect(rows.find((row) => row.name === "lives")?.id).toBe(lives.id);
    expect(rows.find((row) => row.name === "lives")?.value).toBe("5");
    expect(rows.find((row) => row.name === "density")?.value).toBe("high");
    expect(rows.find((row) => row.name === "mode")).toBeUndefined();
    expect((await hostsRepo.readSingle())?.settings).toEqual({ lives: "5", density: "high" });
    expect(loaded?.settings).toEqual({ lives: "3", mode: "arcade" });
  });

  it("clears a name-keyed dictionary when the map is empty", async () => {
    const connection = await setup();
    const hostsRepo = generatedRepo<NamedHostRow>(connection, "Host", ["id", "label"]);
    const settingsRepo = generatedRepo<NamedSettingRow>(connection, "Setting", ["id", "parentId", "name", "value"]);
    hostsRepo.addEnriched(
      { destinationProperty: "settings", sourceProperty: "id", relatedKey: "parentId", relatedProperty: "value", type: "dictionary" },
      settingsRepo,
    );
    const created = await hostsRepo.create({ label: "ada", settings: { lives: "3" } } as NamedHostRow);
    await hostsRepo.update({ id: created.id, label: "ada", settings: {} } as NamedHostRow);
    expect(await collectAsync(settingsRepo.read())).toEqual([]);
    expect((await hostsRepo.readSingle())?.settings).toEqual({});
  });

  it("round-trips an object dictionary keyed by name", async () => {
    const connection = await setup();
    const hostsRepo = generatedRepo<PrefHostRow>(connection, "Host", ["id", "label"]);
    const prefsRepo = generatedRepo<PrefSettingRow>(connection, "Pref", ["id", "parentId", "name", "locale", "timezone"]);
    hostsRepo.addEnriched(
      { destinationProperty: "prefs", sourceProperty: "id", relatedKey: "parentId", type: "dictionary" },
      prefsRepo,
    );
    const created = await hostsRepo.create({
      label: "ada",
      prefs: { home: { locale: "en-GB", timezone: "Europe/London" } as PrefValue },
    } as PrefHostRow);
    const stored = (await collectAsync(prefsRepo.read()))[0];
    expect(stored?.name).toBe("home");
    expect(stored?.locale).toBe("en-GB");
    const loaded = await hostsRepo.readSingle();
    expect(loaded?.prefs.home.locale).toBe("en-GB");
    expect(loaded?.prefs.home.timezone).toBe("Europe/London");
    await hostsRepo.update({
      id: created.id,
      label: "ada",
      prefs: {
        home: { locale: "en-GB", timezone: "Europe/Paris" } as PrefValue,
        work: { locale: "en-US", timezone: "America/New_York" } as PrefValue,
      },
    } as PrefHostRow);
    const updated = await hostsRepo.readSingle();
    expect(updated?.prefs.home.timezone).toBe("Europe/Paris");
    expect(updated?.prefs.work.locale).toBe("en-US");
    expect((await collectAsync(prefsRepo.read())).find((row) => row.name === "home")?.id).toBe(stored?.id);
  });

  it("round-trips a dictionary whose column is key", async () => {
    const connection = await setup();
    const hostsRepo = generatedRepo<NamedHostRow>(connection, "Host", ["id", "label"]);
    const settingsRepo = generatedRepo<KeyedSettingRow>(connection, "Setting", ["id", "parentId", "key", "value"]);
    hostsRepo.addEnriched(
      { destinationProperty: "settings", sourceProperty: "id", relatedKey: "parentId", relatedProperty: "value", type: "dictionary" },
      settingsRepo,
    );
    const created = await hostsRepo.create({ label: "ada", settings: { theme: "dark" } } as NamedHostRow);
    expect((await collectAsync(settingsRepo.read()))[0]?.key).toBe("theme");
    expect((await hostsRepo.readSingle())?.settings).toEqual({ theme: "dark" });
    await hostsRepo.update({ id: created.id, label: "ada", settings: { theme: "sepia" } } as NamedHostRow);
    expect((await hostsRepo.readSingle())?.settings).toEqual({ theme: "sepia" });
  });

  it("keeps list children when the read payload is written back", async () => {
    const connection = await setup();
    const parentsRepo = generatedRepo<ParentRow>(connection, "Parent", ["id", "label"]);
    const childrenRepo = generatedRepo<ChildRow>(connection, "Child", ["id", "parentId", "label"]);
    parentsRepo.addEnriched({ destinationProperty: "children", sourceProperty: "id", relatedKey: "parentId", type: "list" }, childrenRepo);
    const created = await parentsRepo.create({ label: "p", children: [{ label: "a" } as ChildRow, { label: "b" } as ChildRow] } as ParentRow);
    const loaded = await parentsRepo.readSingle();
    expect(loaded?.children.map((child) => child.label).sort()).toEqual(["a", "b"]);
    await parentsRepo.update({ id: created.id, label: "p", children: loaded?.children ?? [] } as ParentRow);
    const kept = await collectAsync(childrenRepo.read());
    expect(kept).toHaveLength(2);
    expect(kept.map((child) => child.id).sort()).toEqual((loaded?.children.map((child) => child.id) ?? []).sort());
  });

  it("keeps linked children when the read payload is written back", async () => {
    const connection = await setup();
    const parentsRepo = generatedRepo<TaggedRow>(connection, "Tagged", ["id", "label"]);
    const tagsRepo = generatedRepo<TagRow>(connection, "Tag", ["id", "name"]);
    const linksRepo = generatedRepo<ParentTagRow>(connection, "ParentTag", ["id", "parentId", "tagId"]);
    const keep = await tagsRepo.create({ name: "keep" } as TagRow);
    const extra = await tagsRepo.create({ name: "extra" } as TagRow);
    expect(extra.id).toBeGreaterThan(0);
    parentsRepo.addLinked(
      { destinationProperty: "tags", sourceProperty: "id", relatedKey: "id", relatedProperty: "name", association: { parent: "parentId", related: "tagId" }, type: "list" },
      tagsRepo,
      linksRepo,
    );
    const created = await parentsRepo.create({ label: "p", tags: [{ id: keep.id, name: keep.name } as TagRow] } as TaggedRow);
    const loaded = await parentsRepo.readSingle();
    expect(loaded?.tags.map((tag) => tag.name)).toEqual(["keep"]);
    await parentsRepo.update({ id: created.id, label: "p", tags: loaded?.tags ?? [] } as TaggedRow);
    const links = await collectAsync(linksRepo.read());
    expect(links).toHaveLength(1);
    expect(links[0]?.tagId).toBe(keep.id);
    expect(await collectAsync(tagsRepo.read())).toHaveLength(2);
  });

  it("copies an object enrichment related field on read", async () => {
    const connection = await setup();
    const typesRepo = generatedRepo<TypeRow>(connection, "Type", ["id", "name"]);
    const hostsRepo = generatedRepo<TypedRow>(connection, "Host", ["id", "typeId"]);
    const type = await typesRepo.create({ name: "arcade" } as TypeRow);
    hostsRepo.addEnriched({ destinationProperty: "typeName", sourceProperty: "typeId", relatedKey: "id", relatedProperty: "name", type: "object" }, typesRepo);
    await hostsRepo.create({ typeId: type.id } as TypedRow);
    expect((await hostsRepo.readSingle())?.typeName).toBe("arcade");
  });

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
    expect(created?.Id).toBe(child?.Id);
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

  it("create-resolves-lookup-name", async () => {
    const connection = await setup();
    const typesRepo = repo(UserType, connection);
    const parentsRepo = repo(Parent, connection);
    await typesRepo.create(Object.assign(new UserType(), { Name: "work" }));
    parentsRepo.addEnriched({ destinationProperty: "TypeName", sourceProperty: "Type", relatedKey: "Id", relatedProperty: "Name" }, typesRepo);
    const created = await parentsRepo.create(Object.assign(new Parent(), { TypeName: "work" }));
    expect(created.Id).toBeGreaterThan(0);
    expect(created.TypeName).toBe("work");
    expect((await parentsRepo.readSingle())?.TypeName).toBe("work");
  });

  it("update-resolves-lookup-name", async () => {
    const connection = await setup();
    const typesRepo = repo(UserType, connection);
    const parentsRepo = repo(Parent, connection);
    const home = await typesRepo.create(Object.assign(new UserType(), { Name: "home" }));
    await typesRepo.create(Object.assign(new UserType(), { Name: "work" }));
    parentsRepo.addEnriched({ destinationProperty: "TypeName", sourceProperty: "Type", relatedKey: "Id", relatedProperty: "Name" }, typesRepo);
    const created = await parentsRepo.create(Object.assign(new Parent(), { Type: Object.assign(new UserType(), { Id: home.Id }) }));
    await parentsRepo.update(Object.assign(new Parent(), { Id: created.Id, TypeName: "work" }));
    expect((await parentsRepo.readSingle())?.TypeName).toBe("work");
  });

  it("create-keeps-existing-lookup-key", async () => {
    const connection = await setup();
    const typesRepo = repo(UserType, connection);
    const parentsRepo = repo(Parent, connection);
    const home = await typesRepo.create(Object.assign(new UserType(), { Name: "home" }));
    await typesRepo.create(Object.assign(new UserType(), { Name: "work" }));
    parentsRepo.addEnriched({ destinationProperty: "TypeName", sourceProperty: "Type", relatedKey: "Id", relatedProperty: "Name" }, typesRepo);
    await parentsRepo.create(Object.assign(new Parent(), { Type: Object.assign(new UserType(), { Id: home.Id }), TypeName: "work" }));
    expect((await parentsRepo.readSingle())?.TypeName).toBe("home");
    expect((await collectAsync(typesRepo.read())).length).toBe(2);
  });

  it("create-resolves-lookup-name-without-source-property", async () => {
    const connection = await setup();
    const typesRepo = repo(UserType, connection);
    const parentsRepo = repo(Named, connection);
    await typesRepo.create(Object.assign(new UserType(), { Name: "work" }));
    parentsRepo.addEnriched({ destinationProperty: "TypeName", sourceProperty: "Type", relatedKey: "Id", relatedProperty: "Name" }, typesRepo);
    const created = await parentsRepo.create(Object.assign(new Named(), { TypeName: "work" }));
    expect(created.Id).toBeGreaterThan(0);
    expect(created.TypeName).toBe("work");
    expect((await parentsRepo.readSingle())?.TypeName).toBe("work");
  });

  it("create-unknown-lookup-name-throws", async () => {
    const connection = await setup();
    const typesRepo = repo(UserType, connection);
    const parentsRepo = repo(Parent, connection);
    parentsRepo.addEnriched({ destinationProperty: "TypeName", sourceProperty: "Type", relatedKey: "Id", relatedProperty: "Name" }, typesRepo);
    await expect(parentsRepo.create(Object.assign(new Parent(), { TypeName: "missing" }))).rejects.toThrow(/TypeName/);
  });

  it("nested-child-resolves-lookup-name", async () => {
    const connection = await setup();
    const parentsRepo = repo(Parent, connection);
    const childrenRepo = repo(Child, connection);
    const childTypesRepo = repo(UserType, connection);
    await childTypesRepo.create(Object.assign(new UserType(), { Name: "home" }));
    childrenRepo.addEnriched({ destinationProperty: "TypeName", sourceProperty: "Type", relatedKey: "Id", relatedProperty: "Name" }, childTypesRepo);
    parentsRepo.addEnriched({ destinationProperty: "Children", sourceProperty: "Id", relatedKey: "ParentId", type: "list" }, childrenRepo);
    await parentsRepo.create(Object.assign(new Parent(), {
      Children: [Object.assign(new Child(), { TypeName: "home", Label: "a" })],
    }));
    const child = await childrenRepo.readSingle();
    expect(child?.Label).toBe("a");
    expect(child?.TypeName).toBe("home");
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
