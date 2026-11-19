import { describe, expect, it } from "vitest";
import { MemoryDriver } from "@deterministic-code/datasources-memory";
import { collectAsync } from "./collectAsync.ts";
import { ChangeCasingMapper } from "./mappers/ChangeCasingMapper.ts";
import { ReadonlyRepository } from "./ReadonlyRepository.ts";
import { TreeReadonlyEnrichmentStrategy } from "./TreeReadonlyEnrichmentStrategy.ts";
import { GraphReadonlyEnrichmentStrategy } from "./GraphReadonlyEnrichmentStrategy.ts";
import type { IConnection } from "./IConnection.ts";
import type { Scalar } from "./builders/Scalar.ts";
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

type Seed = Record<string, Scalar>;

const snake = new ChangeCasingMapper("pascalCase", "snakeCase");

const repo = <T>(type: new () => T, connection: IConnection) =>
  new ReadonlyRepository(connection, { type, propertyMappings: snake });

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

describe("TreeReadonlyEnrichmentStrategy", () => {
  it("fills three levels and stops on a cycle", async () => {
    const food: Seed = { name: "food" };
    const fruit: Seed = { name: "fruit" };
    const apple: Seed = { name: "apple" };
    const connection = await setup([food, fruit, apple]);
    await connection.createCommand().execute("update", [
      { name: "id", value: fruit.id },
      { name: "parent_category_id", value: food.id },
    ]);
    await connection.createCommand().execute("update", [
      { name: "id", value: apple.id },
      { name: "parent_category_id", value: fruit.id },
    ]);
    await connection.createCommand().execute("update", [
      { name: "id", value: food.id },
      { name: "parent_category_id", value: apple.id },
    ]);
    const via = repo(Category, connection);
    const categories = repo(Category, connection);
    categories.addEnriched(
      { destinationProperty: "Children", sourceProperty: "Id", relatedKey: "ParentCategoryId", type: "list" },
      via,
      new TreeReadonlyEnrichmentStrategy(),
    );
    const loaded = await categories.readSingle(new Where(new Operator.Equals("Id", food.id)));
    expect(loaded?.Name).toBe("food");
    expect(loaded?.Children).toHaveLength(1);
    expect(loaded?.Children[0]?.Name).toBe("fruit");
    expect(loaded?.Children[0]?.Children[0]?.Name).toBe("apple");
    expect(loaded?.Children[0]?.Children[0]?.Children).toEqual([]);
  });

  it("matches a composite parent", async () => {
    const connection = new MemoryDriver().connection();
    await connection.open();
    await connection.createCommand().execute("insert", [
      { name: "region_id", value: 1 },
      { name: "category_id", value: 10 },
      { name: "name", value: "food" },
    ]);
    await connection.createCommand().execute("insert", [
      { name: "region_id", value: 1 },
      { name: "category_id", value: 11 },
      { name: "parent_region_id", value: 1 },
      { name: "parent_category_id", value: 10 },
      { name: "name", value: "fruit" },
    ]);
    const via = repo(RegionCategory, connection);
    const regions = repo(RegionCategory, connection);
    regions.addEnriched(
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
    const loaded = await regions.readSingle(new Where(new Operator.Equals("CategoryId", 10)));
    expect(loaded?.Children[0]?.Name).toBe("fruit");
    expect(loaded?.Children[0]?.Children).toEqual([]);
  });
});

describe("GraphReadonlyEnrichmentStrategy", () => {
  it("fills edges, copies a diamond, and drops a self-loop", async () => {
    const a: Seed = { value: "a" };
    const b: Seed = { value: "b" };
    const c: Seed = { value: "c" };
    const d: Seed = { value: "d" };
    const nodes = await setup([a, b, c, d]);
    const edges = new MemoryDriver().connection();
    await edges.open();
    for (const pair of [
      [a.id, b.id],
      [a.id, c.id],
      [b.id, d.id],
      [c.id, d.id],
      [d.id, a.id],
    ] as const) {
      await edges.createCommand().execute("insert", [
        { name: "from_node_id", value: pair[0] },
        { name: "to_node_id", value: pair[1] },
      ]);
    }
    const via = repo(Node, nodes);
    const link = repo(NodeEdge, edges);
    const graph = repo(Node, nodes);
    graph.addEnriched(
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
    const loaded = await graph.readSingle(new Where(new Operator.Equals("Id", a.id)));
    expect(loaded?.Edges.map((node) => node.Value).sort()).toEqual(["b", "c"]);
    const throughB = loaded?.Edges.find((node) => node.Value === "b")?.Edges ?? [];
    const throughC = loaded?.Edges.find((node) => node.Value === "c")?.Edges ?? [];
    expect(throughB[0]?.Value).toBe("d");
    expect(throughC[0]?.Value).toBe("d");
    expect(throughB[0]?.Edges.length === 0 || throughC[0]?.Edges.length === 0).toBe(true);
    const loop: Seed = { value: "loop" };
    const loopNodes = await setup([loop]);
    const loopEdges = new MemoryDriver().connection();
    await loopEdges.open();
    await loopEdges.createCommand().execute("insert", [
      { name: "from_node_id", value: loop.id },
      { name: "to_node_id", value: loop.id },
    ]);
    const loopRepo = repo(Node, loopNodes);
    loopRepo.addEnriched(
      {
        destinationProperty: "Edges",
        sourceProperty: "Id",
        relatedKey: "Id",
        type: "list",
        association: { parent: "FromNodeId", related: "ToNodeId" },
      },
      repo(Node, loopNodes),
      new GraphReadonlyEnrichmentStrategy(repo(NodeEdge, loopEdges)),
    );
    const items = await collectAsync(loopRepo.read());
    expect(items[0]?.Edges).toEqual([]);
  });
});
