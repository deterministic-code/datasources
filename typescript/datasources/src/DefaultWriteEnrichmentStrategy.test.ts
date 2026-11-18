import { describe, expect, it } from "vitest";
import { MemoryDriver } from "@deterministic-code/datasources-memory";
import { CRUDRepository } from "./CRUDRepository.ts";
import { DefaultWriteEnrichmentStrategy } from "./DefaultWriteEnrichmentStrategy.ts";
import { lookupRow, type IWriteEnrichmentStrategy } from "./IWriteEnrichmentStrategy.ts";
import { MetaDataRowTypeConverter } from "./converters/MetaDataRowTypeConverter.ts";
import { RepositoryTree } from "./RepositoryTree.ts";
import { RepositoryTreeNode } from "./RepositoryTreeNode.ts";
import { Schema } from "./Schema.ts";
import { RowTypeConverter } from "./converters/RowTypeConverter.ts";
import { TypeConverter } from "./converters/TypeConverter.ts";
import type { Value } from "./converters/Value.ts";

class Parent {
  Id = 0;
  Type = "";
  Active = false;
  TypeName = "";
  Children: Child[] = [];
}

class Child {
  Id = 0;
  ParentId = 0;
}

class UserType {
  Id = 0;
  Name = "";
}

const column = (property: string): string => (property === "Type" ? "type_id" : property);

const graph = () => {
  const connection = new MemoryDriver().connection();
  const typesRepo = new CRUDRepository(UserType, connection);
  const childrenRepo = new CRUDRepository(Child, connection);
  const parentsRepo = new CRUDRepository(Parent, connection);
  parentsRepo.addEnriched(
    { destinationProperty: "TypeName", sourceProperty: "Type", relatedKey: "Id", relatedProperty: "Name" },
    typesRepo,
  );
  parentsRepo.addEnriched(
    { destinationProperty: "Children", sourceProperty: "Id", relatedKey: "ParentId", type: "list" },
    childrenRepo,
  );
  const write = new DefaultWriteEnrichmentStrategy<Parent>();
  const tree = new RepositoryTree(new RepositoryTreeNode(parentsRepo, ""));
  return { write, tree };
};

describe("DefaultWriteEnrichmentStrategy", () => {
  it("sources include object foreign keys", () => {
    const { write, tree } = graph();
    const sources = write.sources(tree, column);
    expect(sources.has("Type")).toBe(true);
    expect(sources.has("type_id")).toBe(true);
    expect(sources.has("Children")).toBe(false);
    expect(sources.has("Id")).toBe(false);
  });

  it("destination is nested property", () => {
    const { write, tree } = graph();
    expect(write.destination(tree, "TypeName")).toBe(true);
    expect(write.destination(tree, "Children")).toBe(true);
    expect(write.destination(tree, "Type")).toBe(false);
  });

  it("lookup remaps column onto source", async () => {
    const { write, tree } = graph();
    const record: Record<string, number> = { type_id: 7 };
    await write.lookup(tree, [new Parent()], record, new Schema({}), column);
    expect(record.Type).toBe(7);
    expect(record).not.toHaveProperty("type_id");
  });

  it("lookup keeps an assigned source and skips an empty source", async () => {
    const { write, tree } = graph();
    const assigned: Record<string, number> = { Type: 3 };
    await write.lookup(tree, [new Parent()], assigned, new Schema({}), column);
    expect(assigned.Type).toBe(3);
    expect(assigned).not.toHaveProperty("type_id");
    const empty: Record<string, number> = {};
    await write.lookup(tree, [new Parent()], empty, new Schema({}), column);
    expect(empty).not.toHaveProperty("Type");
    expect(empty).not.toHaveProperty("type_id");
  });

  it("lookup then reverseConvert converts boolean on property keys", async () => {
    const { write, tree } = graph();
    const record: Record<string, number | boolean> = { type_id: 7, Active: true };
    await write.lookup(tree, [new Parent()], record, new Schema({}), column);
    class Flag extends TypeConverter<Value, Value> {
      convert = (value: Value): Value => value === 1;
      reverseConvert = (value: Value): Value => (value === true ? 1 : 0);
    }
    const converted = RowTypeConverter.from({ boolean: new Flag() }, { Type: "integer", Active: "boolean" }).reverseConvert(
      record,
    );
    expect(converted.Active).toBe(1);
    expect(converted.Type).toBe(7);
  });

  it("lookupRow converts uuid and boolean on property keys", async () => {
    const { write, tree } = graph();
    const id = "11111111-1111-1111-1111-111111111111";
    const item = Object.assign(new Parent(), { Type: id, Active: true });
    class Flag extends TypeConverter<Value, Value> {
      convert = (value: Value): Value => value === 1;
      reverseConvert = (value: Value): Value => (value === true ? 1 : 0);
    }
    class UuidText extends TypeConverter<Value, Value> {
      convert = (value: Value): Value => value;
      reverseConvert = (value: Value): Value => value;
    }
    const converted = await write.lookupRow(
      tree,
      item,
      new Schema({
        Type: { type: "uuid", nullable: false, hasDefault: false },
        Active: { type: "boolean", nullable: false, hasDefault: false },
      }),
      column,
      new MetaDataRowTypeConverter(),
      RowTypeConverter.from({ boolean: new Flag(), uuid: new UuidText() }, { Type: "uuid", Active: "boolean" }),
    );
    expect(converted).toHaveProperty("Type");
    expect(converted).not.toHaveProperty("type_id");
    expect(converted.Type).toBe(id);
    expect(converted.Active).toBe(1);
    const skipped = await write.lookupRow(tree, item, new Schema({}), column, new MetaDataRowTypeConverter());
    expect(skipped.Type).toBe(id);
    expect(skipped.Active).toBe(true);
  });

  it("sources and destination are empty without enrichments", async () => {
    const write = new DefaultWriteEnrichmentStrategy<Parent>();
    const tree = new RepositoryTree(new RepositoryTreeNode({}, ""));
    expect(write.sources(tree, column).size).toBe(0);
    expect(write.destination(tree, "TypeName")).toBe(false);
    await write.lookup(tree, [new Parent()], {}, new Schema({}), column);
  });

  it("repository write without lookup uses empty extras", async () => {
    const write: IWriteEnrichmentStrategy<Parent> = {
      useTransaction: false,
      persist: async () => {},
      resolveLookups: async () => {},
    };
    const repo = new CRUDRepository(new MemoryDriver().connection(), {
      type: Parent,
      ids: ["Id"],
      write,
    });
    expect(repo.writeSources().size).toBe(0);
    await lookupRow(
      write,
      new RepositoryTree(new RepositoryTreeNode(repo, "")),
      new Parent(),
      new Schema({}),
      column,
      new MetaDataRowTypeConverter(),
    );
    await repo.create(Object.assign(new Parent(), { Id: 1, TypeName: "x" }));
    await repo.update(Object.assign(new Parent(), { Id: 1, TypeName: "y" }));
  });

  it("lookupRow helper uses lookup and reverseConvert when lookupRow is missing", async () => {
    const write: IWriteEnrichmentStrategy<Parent> = {
      useTransaction: false,
      persist: async () => {},
      lookup: async (_tree, _items, record) => {
        record.Type = 7;
      },
    };
    class Flag extends TypeConverter<Value, Value> {
      convert = (value: Value): Value => value === 1;
      reverseConvert = (value: Value): Value => (value === true ? 1 : 0);
    }
    const converted = await lookupRow(
      write,
      new RepositoryTree(new RepositoryTreeNode({}, "")),
      Object.assign(new Parent(), { Active: true }),
      new Schema({}),
      column,
      new MetaDataRowTypeConverter(),
      RowTypeConverter.from({ boolean: new Flag() }, { Active: "boolean" }),
    );
    expect(converted.Type).toBe(7);
    expect(converted.Active).toBe(1);
  });

  it("repository write without optional lookup methods maps the row", async () => {
    const write: IWriteEnrichmentStrategy<Parent> = {
      useTransaction: false,
      persist: async () => {},
    };
    class Flag extends TypeConverter<Value, Value> {
      convert = (value: Value): Value => value === 1;
      reverseConvert = (value: Value): Value => (value === true ? 1 : 0);
    }
    const repo = new CRUDRepository(new MemoryDriver().connection(), {
      type: Parent,
      ids: ["Id"],
      write,
      rowConverter: RowTypeConverter.from({ boolean: new Flag() }, { Active: "boolean" }),
    });
    await lookupRow(
      write,
      new RepositoryTree(new RepositoryTreeNode(repo, "")),
      new Parent(),
      new Schema({}),
      column,
      new MetaDataRowTypeConverter(),
    );
    await repo.create(Object.assign(new Parent(), { Id: 1, TypeName: "x" }));
    await repo.delete(Object.assign(new Parent(), { Id: 1, Active: true }));
  });
});
