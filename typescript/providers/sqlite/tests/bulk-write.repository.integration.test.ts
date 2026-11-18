import { describe, expect, it } from "vitest";
import {
  BatchCommandIBulkWriteStrategy,
  BulkCRUDRepository,
  ChangeCasingMapper,
  Datasource,
  Native,
  Schema,
  createCRUDRepositoryOptions,
  type IConnection,
  type SchemaColumn,
} from "@deterministic-code/datasources";
import { collectAsync } from "../../../datasources/src/collectAsync.ts";
import { SqliteDriver } from "../src/SqliteDriver.ts";

class Parent {
  Id = 0;
  Label = "";
  Children: Child[] = [];
}

class Child {
  Id = 0;
  ParentId = 0;
  Label = "";
}

const column = (type: string, isId = false): SchemaColumn => ({
  type,
  nullable: false,
  hasDefault: false,
  isId,
});

const open = async (): Promise<IConnection> => {
  const ds = new Datasource("sqlite", new SqliteDriver(), { DATASOURCE_URL: "sqlite:///:memory:" });
  const connection = await ds.createConnection();
  await connection.open();
  return connection;
};

describe("bulk write enrichment", () => {
  it("stamps a generated parent id onto children", async () => {
    const connection = await open();
    const command = connection.createCommand();
    await command.executeNative(
      new Native("CREATE TABLE parents (id INTEGER PRIMARY KEY AUTOINCREMENT, label TEXT NOT NULL)"),
    );
    await command.executeNative(
      new Native(
        "CREATE TABLE children (id INTEGER PRIMARY KEY AUTOINCREMENT, parent_id INTEGER NOT NULL, label TEXT NOT NULL)",
      ),
    );
    const casing = new ChangeCasingMapper("pascalCase", "snakeCase");
    const childrenRepo = new BulkCRUDRepository(
      connection,
      {
        ...createCRUDRepositoryOptions(Child),
        entity: "children",
        propertyMappings: casing,
        ids: ["Id"],
        schema: new Schema({
          Id: column("integer", true),
          ParentId: column("integer"),
          Label: column("string"),
        }),
      },
      new BatchCommandIBulkWriteStrategy<Child>(),
    );
    const parentsRepo = new BulkCRUDRepository(
      connection,
      {
        ...createCRUDRepositoryOptions(Parent),
        entity: "parents",
        propertyMappings: casing,
        ids: ["Id"],
        schema: new Schema({
          Id: column("integer", true),
          Label: column("string"),
        }),
      },
      new BatchCommandIBulkWriteStrategy<Parent>(),
    );
    parentsRepo.addEnriched(
      { destinationProperty: "Children", sourceProperty: "Id", relatedKey: "ParentId", type: "list" },
      childrenRepo,
    );
    const created = await parentsRepo.createBulk([
      Object.assign(new Parent(), { Label: "one", Children: [Object.assign(new Child(), { Label: "a" })] }),
      Object.assign(new Parent(), {
        Label: "two",
        Children: [Object.assign(new Child(), { Label: "b" }), Object.assign(new Child(), { Label: "c" })],
      }),
    ]);
    expect(created).toHaveLength(2);
    expect(created.every((parent) => parent.Id > 0)).toBe(true);
    expect(new Set(created.map((parent) => parent.Id)).size).toBe(2);
    const children = await collectAsync(childrenRepo.read());
    expect(children).toHaveLength(3);
    expect(children.find((child) => child.ParentId === created[0]!.Id)?.Label).toBe("a");
    expect(
      children
        .filter((child) => child.ParentId === created[1]!.Id)
        .map((child) => child.Label)
        .sort(),
    ).toEqual(["b", "c"]);
  });
});
