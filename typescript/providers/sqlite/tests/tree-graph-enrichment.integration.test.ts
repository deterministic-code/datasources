import { describe, expect, it } from "vitest";
import {
  ChangeCasingMapper,
  Datasource,
  Native,
  Operator,
  ReadonlyRepository,
  TreeReadonlyEnrichmentStrategy,
  Where,
} from "@deterministic-code/datasources";
import { SqliteDriver } from "../src/SqliteDriver.ts";

class Category {
  Id = 0;
  ParentCategoryId: number | null = null;
  Name = "";
  Children: Category[] = [];
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
});
