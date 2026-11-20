import { describe, expect, it } from "vitest";
import {
  ChangeCasingMapper,
  CRUDRepository,
  Datasource,
  DefaultWriteEnrichmentStrategy,
  GraphCreateEnrichmentStrategy,
  GraphReadonlyEnrichmentStrategy,
  Native,
  Operator,
  ReadonlyRepository,
  TreeCreateEnrichmentStrategy,
  TreeReadonlyEnrichmentStrategy,
  Where,
} from "@deterministic-code/datasources";
import { collectAsync } from "../../../datasources/src/collectAsync.ts";
import { SqliteDriver } from "../src/SqliteDriver.ts";

const flatten = (node: Category): string[] => [node.Name, ...node.Children.flatMap(flatten)];

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
  Id = 0;
  FromNodeId = 0;
  ToNodeId = 0;
}

describe("tree enrichment", () => {
  it("stops on a cycle", async () => {
    const ds = new Datasource("sqlite", new SqliteDriver(), { DATASOURCE_URL: "sqlite:///:memory:" });
    const connection = await ds.createConnection();
    await connection.open();
    const command = connection.createCommand();
    await command.executeNative(
      new Native("CREATE TABLE categories (id INTEGER PRIMARY KEY AUTOINCREMENT, parent_category_id INTEGER, name TEXT NOT NULL)"),
    );
    await command.executeNative(new Native("INSERT INTO categories (id, parent_category_id, name) VALUES (1, 3, 'food')"));
    await command.executeNative(new Native("INSERT INTO categories (id, parent_category_id, name) VALUES (2, 1, 'fruit')"));
    await command.executeNative(new Native("INSERT INTO categories (id, parent_category_id, name) VALUES (3, 2, 'apple')"));
    const snake = new ChangeCasingMapper("pascalCase", "snakeCase");
    const via = new ReadonlyRepository(connection, { type: Category, entity: "categories", propertyMappings: snake });
    const repo = new ReadonlyRepository(connection, { type: Category, entity: "categories", propertyMappings: snake });
    repo.addEnriched(
      { destinationProperty: "Children", sourceProperty: "Id", relatedKey: "ParentCategoryId", type: "list" },
      via,
      new TreeReadonlyEnrichmentStrategy(),
    );
    const food = await repo.readSingle(new Where(new Operator.Equals("Id", 1)));
    expect(food?.Children[0]?.Name).toBe("fruit");
    expect(food?.Children[0]?.Children[0]?.Name).toBe("apple");
    expect(food?.Children[0]?.Children[0]?.Children).toEqual([]);
  });

  it("returns each row once under the roots", async () => {
    const ds = new Datasource("sqlite", new SqliteDriver(), { DATASOURCE_URL: "sqlite:///:memory:" });
    const connection = await ds.createConnection();
    await connection.open();
    const command = connection.createCommand();
    await command.executeNative(
      new Native("CREATE TABLE categories (id INTEGER PRIMARY KEY AUTOINCREMENT, parent_category_id INTEGER, name TEXT NOT NULL)"),
    );
    await command.executeNative(new Native("INSERT INTO categories (id, parent_category_id, name) VALUES (1, NULL, 'food')"));
    await command.executeNative(new Native("INSERT INTO categories (id, parent_category_id, name) VALUES (2, 1, 'fruit')"));
    await command.executeNative(new Native("INSERT INTO categories (id, parent_category_id, name) VALUES (3, 2, 'apple')"));
    const snake = new ChangeCasingMapper("pascalCase", "snakeCase");
    const via = new ReadonlyRepository(connection, { type: Category, entity: "categories", propertyMappings: snake });
    const repo = new ReadonlyRepository(connection, { type: Category, entity: "categories", propertyMappings: snake });
    repo.addEnriched(
      { destinationProperty: "Children", sourceProperty: "Id", relatedKey: "ParentCategoryId", type: "list" },
      via,
      new TreeReadonlyEnrichmentStrategy(),
    );
    const roots = await collectAsync(repo.read());
    expect(roots.map((row) => row.Name)).toEqual(["food"]);
    expect(flatten(roots[0]!)).toEqual(["food", "fruit", "apple"]);
    const fruit = await repo.readSingle(new Where(new Operator.Equals("Id", 2)));
    expect(fruit?.Name).toBe("fruit");
    expect(fruit?.Children[0]?.Name).toBe("apple");
  });

  it("returns the composite root", async () => {
    const ds = new Datasource("sqlite", new SqliteDriver(), { DATASOURCE_URL: "sqlite:///:memory:" });
    const connection = await ds.createConnection();
    await connection.open();
    const command = connection.createCommand();
    await command.executeNative(
      new Native(`CREATE TABLE region_categories (
                region_id INTEGER NOT NULL,
                category_id INTEGER NOT NULL,
                parent_region_id INTEGER,
                parent_category_id INTEGER,
                name TEXT NOT NULL,
                PRIMARY KEY (region_id, category_id)
            )`),
    );
    await command.executeNative(
      new Native("INSERT INTO region_categories (region_id, category_id, name) VALUES (1, 10, 'food')"),
    );
    await command.executeNative(
      new Native(
        "INSERT INTO region_categories (region_id, category_id, parent_region_id, parent_category_id, name) VALUES (1, 11, 1, 10, 'fruit')",
      ),
    );
    const snake = new ChangeCasingMapper("pascalCase", "snakeCase");
    const via = new ReadonlyRepository(connection, {
      type: RegionCategory,
      entity: "region_categories",
      propertyMappings: snake,
    });
    const repo = new ReadonlyRepository(connection, {
      type: RegionCategory,
      entity: "region_categories",
      propertyMappings: snake,
    });
    repo.addEnriched(
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
    const roots = await collectAsync(repo.read());
    expect(roots.map((row) => row.Name)).toEqual(["food"]);
    expect(roots[0]?.Children[0]?.Name).toBe("fruit");
  });

  it("create then read matches", async () => {
    const ds = new Datasource("sqlite", new SqliteDriver(), { DATASOURCE_URL: "sqlite:///:memory:" });
    const connection = await ds.createConnection();
    await connection.open();
    await connection.createCommand().executeNative(
      new Native("CREATE TABLE categories (id INTEGER PRIMARY KEY AUTOINCREMENT, parent_category_id INTEGER, name TEXT NOT NULL)"),
    );
    const snake = new ChangeCasingMapper("pascalCase", "snakeCase");
    const via = new CRUDRepository(connection, { type: Category, entity: "categories", ids: ["Id"], propertyMappings: snake });
    const repo = new CRUDRepository(connection, {
      type: Category,
      entity: "categories",
      ids: ["Id"],
      propertyMappings: snake,
      write: new DefaultWriteEnrichmentStrategy(undefined, undefined, new TreeCreateEnrichmentStrategy()),
    });
    repo.addEnriched(
      { destinationProperty: "Children", sourceProperty: "Id", relatedKey: "ParentCategoryId", type: "list" },
      via,
      new TreeReadonlyEnrichmentStrategy(),
    );
    await repo.create(
      Object.assign(new Category(), {
        Name: "food",
        Children: [Object.assign(new Category(), { Name: "fruit", Children: [Object.assign(new Category(), { Name: "apple" })] })],
      }),
    );
    const food = await repo.readSingle(new Where(new Operator.Equals("Name", "food")));
    expect(food?.Children[0]?.Name).toBe("fruit");
    expect(food?.Children[0]?.Children[0]?.Name).toBe("apple");
  });

  it("create throws on a cycle", async () => {
    const ds = new Datasource("sqlite", new SqliteDriver(), { DATASOURCE_URL: "sqlite:///:memory:" });
    const connection = await ds.createConnection();
    await connection.open();
    await connection.createCommand().executeNative(
      new Native("CREATE TABLE categories (id INTEGER PRIMARY KEY AUTOINCREMENT, parent_category_id INTEGER, name TEXT NOT NULL)"),
    );
    const snake = new ChangeCasingMapper("pascalCase", "snakeCase");
    const via = new CRUDRepository(connection, { type: Category, entity: "categories", ids: ["Id"], propertyMappings: snake });
    const repo = new CRUDRepository(connection, {
      type: Category,
      entity: "categories",
      ids: ["Id"],
      propertyMappings: snake,
      write: new DefaultWriteEnrichmentStrategy(undefined, undefined, new TreeCreateEnrichmentStrategy()),
    });
    repo.addEnriched(
      { destinationProperty: "Children", sourceProperty: "Id", relatedKey: "ParentCategoryId", type: "list" },
      via,
      new TreeReadonlyEnrichmentStrategy(),
    );
    const food = Object.assign(new Category(), { Name: "food" });
    const apple = Object.assign(new Category(), { Name: "apple", Children: [food] });
    food.Children = [Object.assign(new Category(), { Name: "fruit", Children: [apple] })];
    await expect(repo.create(food)).rejects.toThrow(/food/);
  });

  it("create stamps composite parents", async () => {
    const ds = new Datasource("sqlite", new SqliteDriver(), { DATASOURCE_URL: "sqlite:///:memory:" });
    const connection = await ds.createConnection();
    await connection.open();
    await connection.createCommand().executeNative(
      new Native(`CREATE TABLE region_categories (
                region_id INTEGER NOT NULL,
                category_id INTEGER NOT NULL,
                parent_region_id INTEGER,
                parent_category_id INTEGER,
                name TEXT NOT NULL,
                PRIMARY KEY (region_id, category_id),
                FOREIGN KEY (parent_region_id, parent_category_id) REFERENCES region_categories (region_id, category_id)
            )`),
    );
    const snake = new ChangeCasingMapper("pascalCase", "snakeCase");
    const via = new CRUDRepository(connection, {
      type: RegionCategory,
      entity: "region_categories",
      ids: ["RegionId", "CategoryId"],
      propertyMappings: snake,
    });
    const repo = new CRUDRepository(connection, {
      type: RegionCategory,
      entity: "region_categories",
      ids: ["RegionId", "CategoryId"],
      propertyMappings: snake,
      write: new DefaultWriteEnrichmentStrategy(undefined, undefined, new TreeCreateEnrichmentStrategy()),
    });
    repo.addEnriched(
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
    await repo.create(
      Object.assign(new RegionCategory(), {
        RegionId: 1,
        CategoryId: 1,
        Name: "west",
        Children: [Object.assign(new RegionCategory(), { RegionId: 1, CategoryId: 2, Name: "coast" })],
      }),
    );
    const west = await repo.readSingle(new Where(new Operator.Equals("Name", "west")));
    expect(west?.Children[0]?.Name).toBe("coast");
    expect(west?.Children[0]?.ParentRegionId).toBe(1);
    expect(west?.Children[0]?.ParentCategoryId).toBe(1);
  });
});

describe("graph enrichment", () => {
  it("create writes a loop then read", async () => {
    const ds = new Datasource("sqlite", new SqliteDriver(), { DATASOURCE_URL: "sqlite:///:memory:" });
    const connection = await ds.createConnection();
    await connection.open();
    const command = connection.createCommand();
    await command.executeNative(new Native("CREATE TABLE nodes (id INTEGER PRIMARY KEY AUTOINCREMENT, value TEXT NOT NULL)"));
    await command.executeNative(
      new Native("CREATE TABLE edges (id INTEGER PRIMARY KEY AUTOINCREMENT, from_node_id INTEGER NOT NULL, to_node_id INTEGER NOT NULL)"),
    );
    const snake = new ChangeCasingMapper("pascalCase", "snakeCase");
    const via = new CRUDRepository(connection, { type: Node, entity: "nodes", ids: ["Id"], propertyMappings: snake });
    const link = new CRUDRepository(connection, { type: NodeEdge, entity: "edges", ids: ["Id"], propertyMappings: snake });
    const repo = new CRUDRepository(connection, {
      type: Node,
      entity: "nodes",
      ids: ["Id"],
      propertyMappings: snake,
      write: new DefaultWriteEnrichmentStrategy(undefined, undefined, new GraphCreateEnrichmentStrategy(link)),
    });
    repo.addEnriched(
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
    await repo.create(a);
    const loadedA = await repo.readSingle(new Where(new Operator.Equals("Value", "a")));
    expect(loadedA?.Edges).toHaveLength(1);
    expect(loadedA?.Edges[0]?.Value).toBe("b");
    const loadedB = await repo.readSingle(new Where(new Operator.Equals("Value", "b")));
    expect(loadedB?.Edges).toHaveLength(1);
    expect(loadedB?.Edges[0]?.Value).toBe("a");
  });
});
