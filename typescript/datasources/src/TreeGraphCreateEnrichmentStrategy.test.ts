import { describe, expect, it } from "vitest";
import { MemoryDriver } from "@deterministic-code/datasources-memory";
import { collectAsync } from "./collectAsync.ts";
import { CRUDRepository } from "./CRUDRepository.ts";
import { DefaultWriteEnrichmentStrategy } from "./DefaultWriteEnrichmentStrategy.ts";
import { GraphCreateEnrichmentStrategy } from "./GraphCreateEnrichmentStrategy.ts";
import { GraphReadonlyEnrichmentStrategy } from "./GraphReadonlyEnrichmentStrategy.ts";
import { TreeCreateEnrichmentStrategy } from "./TreeCreateEnrichmentStrategy.ts";
import { TreeReadonlyEnrichmentStrategy } from "./TreeReadonlyEnrichmentStrategy.ts";
import type { IConnection } from "./IConnection.ts";
import { ChangeCasingMapper } from "./mappers/ChangeCasingMapper.ts";
import { Operator, Where } from "./statements/index.ts";

class Category {
  Id = 0;
  ParentCategoryId: number | null = null;
  Name = "";
  Children: Category[] = [];
}

class RegionCategory {
  RegionId = 0;
  CategoryId = 0;
  ParentRegionId: number | null = null;
  ParentCategoryId: number | null = null;
  Name = "";
  Children: RegionCategory[] = [];
}

class Node {
  Id = 0;
  Value = "";
  Edges: Node[] = [];
}

class NodeEdge {
  FromNodeId = 0;
  ToNodeId = 0;
}

const snake = new ChangeCasingMapper("pascalCase", "snakeCase");

const repo = <T extends object>(type: new () => T, connection: IConnection, write?: DefaultWriteEnrichmentStrategy<T>, ids = ["Id"]) =>
  new CRUDRepository<T>(connection, { type, propertyMappings: snake, write, ids });

const open = async (): Promise<IConnection> => {
  const connection = new MemoryDriver().connection();
  await connection.open();
  return connection;
};

describe("TreeCreateEnrichmentStrategy", () => {
  it("writes parents first by name", async () => {
    const connection = await open();
    const via = repo(Category, connection);
    const trees = repo(Category, connection, new DefaultWriteEnrichmentStrategy(undefined, undefined, new TreeCreateEnrichmentStrategy()));
    trees.addEnriched(
      { destinationProperty: "Children", sourceProperty: "Id", relatedKey: "ParentCategoryId", type: "list" },
      via,
      new TreeReadonlyEnrichmentStrategy(),
    );
    await trees.create(
      Object.assign(new Category(), {
        Name: "food",
        Children: [
          Object.assign(new Category(), {
            Name: "fruit",
            Children: [Object.assign(new Category(), { Name: "apple" })],
          }),
        ],
      }),
    );
    const food = await trees.readSingle(new Where(new Operator.Equals("Name", "food")));
    expect(food?.Children[0]?.Name).toBe("fruit");
    expect(food?.Children[0]?.Children[0]?.Name).toBe("apple");
    expect(food?.Children[0]?.Children[0]?.Children).toEqual([]);
  });

  it("throws on a cycle", async () => {
    const connection = await open();
    const via = repo(Category, connection);
    const trees = repo(Category, connection, new DefaultWriteEnrichmentStrategy(undefined, undefined, new TreeCreateEnrichmentStrategy()));
    trees.addEnriched({ destinationProperty: "Children", sourceProperty: "Id", relatedKey: "ParentCategoryId", type: "list" }, via);
    const food = Object.assign(new Category(), { Name: "food" });
    const apple = Object.assign(new Category(), { Name: "apple", Children: [food] });
    food.Children = [Object.assign(new Category(), { Name: "fruit", Children: [apple] })];
    await expect(trees.create(food)).rejects.toThrow(/food/);
  });

  it("stamps composite parents", async () => {
    const connection = await open();
    const via = repo(RegionCategory, connection, undefined, ["RegionId", "CategoryId"]);
    const trees = repo(
      RegionCategory,
      connection,
      new DefaultWriteEnrichmentStrategy(undefined, undefined, new TreeCreateEnrichmentStrategy()),
      ["RegionId", "CategoryId"],
    );
    trees.addEnriched(
      {
        destinationProperty: "Children",
        sourceProperty: "RegionId",
        relatedKey: "ParentRegionId",
        type: "list",
        keys: [
          { sourceProperty: "RegionId", relatedKey: "ParentRegionId" },
          { sourceProperty: "CategoryId", relatedKey: "ParentCategoryId" },
        ],
      },
      via,
      new TreeReadonlyEnrichmentStrategy(),
    );
    await trees.create(
      Object.assign(new RegionCategory(), {
        RegionId: 1,
        CategoryId: 1,
        Name: "west",
        Children: [Object.assign(new RegionCategory(), { RegionId: 1, CategoryId: 2, Name: "coast" })],
      }),
    );
    const west = await trees.readSingle(new Where(new Operator.Equals("Name", "west")));
    expect(west?.Children[0]?.Name).toBe("coast");
    expect(west?.Children[0]?.ParentRegionId).toBe(1);
    expect(west?.Children[0]?.ParentCategoryId).toBe(1);
  });
});

describe("GraphCreateEnrichmentStrategy", () => {
  it("writes nodes then edges and skips a diamond", async () => {
    const nodes = await open();
    const edges = await open();
    const via = repo(Node, nodes);
    const link = repo(NodeEdge, edges);
    const graphs = repo(
      Node,
      nodes,
      new DefaultWriteEnrichmentStrategy(undefined, undefined, new GraphCreateEnrichmentStrategy(link)),
    );
    graphs.addEnriched(
      {
        destinationProperty: "Edges",
        sourceProperty: "Id",
        relatedKey: "Id",
        type: "list",
        association: { parent: "FromNodeId", related: "ToNodeId" },
      },
      via,
      new GraphReadonlyEnrichmentStrategy(link),
    );
    await graphs.create(
      Object.assign(new Node(), {
        Value: "a",
        Edges: [
          Object.assign(new Node(), { Value: "b", Edges: [Object.assign(new Node(), { Value: "d" })] }),
          Object.assign(new Node(), { Value: "c", Edges: [Object.assign(new Node(), { Value: "d" })] }),
        ],
      }),
    );
    expect(await collectAsync(via.read())).toHaveLength(4);
    expect(await collectAsync(link.read())).toHaveLength(4);
    const loaded = await graphs.readSingle(new Where(new Operator.Equals("Value", "a")));
    expect(loaded?.Edges).toHaveLength(2);
  });

  it("writes a loop", async () => {
    const nodes = await open();
    const edges = await open();
    const via = repo(Node, nodes);
    const link = repo(NodeEdge, edges);
    const graphs = repo(
      Node,
      nodes,
      new DefaultWriteEnrichmentStrategy(undefined, undefined, new GraphCreateEnrichmentStrategy(link)),
    );
    graphs.addEnriched(
      {
        destinationProperty: "Edges",
        sourceProperty: "Id",
        relatedKey: "Id",
        type: "list",
        association: { parent: "FromNodeId", related: "ToNodeId" },
      },
      via,
      new GraphReadonlyEnrichmentStrategy(link),
    );
    const a = Object.assign(new Node(), { Value: "a" });
    const b = Object.assign(new Node(), { Value: "b", Edges: [a] });
    a.Edges = [b];
    await graphs.create(a);
    expect(await collectAsync(via.read())).toHaveLength(2);
    expect(await collectAsync(link.read())).toHaveLength(2);
    const loadedA = await graphs.readSingle(new Where(new Operator.Equals("Value", "a")));
    expect(loadedA?.Edges).toHaveLength(1);
    expect(loadedA?.Edges[0]?.Value).toBe("b");
    const loadedB = await graphs.readSingle(new Where(new Operator.Equals("Value", "b")));
    expect(loadedB?.Edges).toHaveLength(1);
    expect(loadedB?.Edges[0]?.Value).toBe("a");
  });
});
