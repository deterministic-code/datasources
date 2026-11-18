import { describe, expect, it } from "vitest";
import { MemoryDriver } from "@deterministic-code/datasources-memory";
import { CRUDRepository } from "./CRUDRepository.ts";
import { DefaultWriteEnrichmentStrategy } from "./DefaultWriteEnrichmentStrategy.ts";
import type { IWriteEnrichmentStrategy } from "./IWriteEnrichmentStrategy.ts";
import { RepositoryTree } from "./RepositoryTree.ts";
import { RepositoryTreeNode } from "./RepositoryTreeNode.ts";
import { Schema } from "./Schema.ts";

class Parent {
  Id = 0;
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
    expect(record.type_id).toBe(7);
    expect(record).not.toHaveProperty("Type");
  });

  it("lookup keeps an assigned source and skips an empty source", async () => {
    const { write, tree } = graph();
    const assigned: Record<string, number> = { Type: 3 };
    await write.lookup(tree, [new Parent()], assigned, new Schema({}), column);
    expect(assigned.type_id).toBe(3);
    expect(assigned).not.toHaveProperty("Type");
    const empty: Record<string, number> = {};
    await write.lookup(tree, [new Parent()], empty, new Schema({}), column);
    expect(empty).not.toHaveProperty("Type");
    expect(empty).not.toHaveProperty("type_id");
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
    await repo.lookupRow(new Parent());
    await repo.create(Object.assign(new Parent(), { Id: 1, TypeName: "x" }));
    await repo.update(Object.assign(new Parent(), { Id: 1, TypeName: "y" }));
  });

  it("repository write without optional lookup methods maps the row", async () => {
    const write: IWriteEnrichmentStrategy<Parent> = {
      useTransaction: false,
      persist: async () => {},
    };
    const repo = new CRUDRepository(new MemoryDriver().connection(), {
      type: Parent,
      ids: ["Id"],
      write,
    });
    await repo.lookupRow(new Parent());
  });
});
