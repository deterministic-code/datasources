import { describe, expect, it } from "vitest";
import { MemoryDriver } from "@deterministic-code/datasources-memory";
import { collectAsync } from "./collectAsync.ts";
import { CRUDRepository } from "./CRUDRepository.ts";
import { NestedToObjectEnrichmentStrategy } from "./NestedToObjectEnrichmentStrategy.ts";
import { ChangeCasingMapper } from "./mappers/ChangeCasingMapper.ts";
import { DictionaryMapper } from "./mappers/DictionaryMapper.ts";
import type { IMapper } from "./mappers/IMapper.ts";
import { TransformMapper } from "./mappers/TransformMapper.ts";
import type { IConnection } from "./IConnection.ts";
import type { Scalar } from "./builders/Scalar.ts";
import { MockCommand } from "./mocks/mock-command.ts";
import { MockConnection } from "./mocks/mock-connection.ts";
import { MockReader } from "./mocks/mock-reader.ts";
import { Join, Operator, SelectStatement, Where } from "./statements/index.ts";

describe("readWithWhereClauseAndCustomFlatToNestedObjectMappingOptions", () => {
  it("read with where clause and custom flat to nested object mapping options", async () => {
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
      Label = "";
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

    const joinEntity = new DictionaryMapper([
      ["Parent", "parent"],
      ["Child", "child"],
    ]);

    const setup = async (): Promise<IConnection> => {
      const connection = new MemoryDriver().connection();
      await connection.open();
      return connection;
    };

    const typesRepo = (connection: IConnection) =>
      new CRUDRepository(connection, { type: UserType, ids: ["Id"], propertyMappings: columns });

    const parentsRepo = (connection: IConnection) =>
      new CRUDRepository(connection, { type: Parent, ids: ["Id"], propertyMappings: columns });

    const connection = await setup();
    const types = typesRepo(connection);
    const type = await types.create(Object.assign(new UserType(), { Name: "person" }));
    const parents = parentsRepo(connection);
    parents.addEnriched(
      { destinationProperty: "TypeName", sourceProperty: "Type", relatedKey: "Id", relatedProperty: "Name" },
      types,
    );
    await parents.create(Object.assign(new Parent(), { Type: Object.assign(new UserType(), { Id: type.Id }) }));
    const enriched = await parents.readSingle();

    const row1: Seed = { Id: 1, TypeName: "person", ChildId: 10, Label: "a" };
    const row2: Seed = { Id: 1, TypeName: "person", ChildId: 11, Label: "b" };
    const mockConnection = new MockConnection({ provider: "memory" });
    const command = new MockCommand(mockConnection);
    mockConnection.command = command;
    const joinParents = parentsRepo(mockConnection);
    joinParents.addEnriched(
      { destinationProperty: "TypeName", sourceProperty: "Type", relatedKey: "Id", relatedProperty: "Name" },
      types,
    );
    joinParents.addEnriched(
      { destinationProperty: "Children", sourceProperty: "Id", relatedKey: "ParentId", type: "list" },
      joinParents,
      new NestedToObjectEnrichmentStrategy<Parent>({
        key: "Id",
        fields: {
          Id: "Id",
          TypeName: "TypeName",
          Children: {
            key: "ChildId",
            fields: { Id: "ChildId", Label: "Label" },
          },
        },
      }),
    );
    const join = new SelectStatement({
      table: "Parent",
      fields: [
        "Parent.Id AS Id",
        "Parent.TypeName AS TypeName",
        "Child.Id AS ChildId",
        "Child.Label AS Label",
      ],
      joins: [Join.left("Child", "Parent.Id", "Child.ParentId")],
      where: new Where(new Operator.Equals("Parent.Id", 1)),
    });
    command.reader = new MockReader([row1, row2]);
    const items = await collectAsync(
      joinParents.read(join, undefined, { entityMapper: joinEntity }),
    );
    expect(enriched?.TypeName).toBe("person");
    expect(items).toHaveLength(1);
    expect(items[0]?.Id).toBe(1);
    expect(items[0]?.TypeName).toBe("person");
    expect(items[0]?.Children.map((child) => child.Label)).toEqual(["a", "b"]);
  });
});
