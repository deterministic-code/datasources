import { describe, expect, it } from "vitest";
import { dialectsOf, type RepositoryModel } from "./RepositoryModel.ts";
import { memoryReader } from "@deterministic-code/generators-common/deterministic-reader";
import { datasourceEnginesOf } from "./datasourceYaml.ts";

const model = (optionsByProvider?: RepositoryModel["optionsByProvider"]): RepositoryModel => ({
  specName: "item",
  typeName: "Item",
  typeHome: "datasource",
  entity: "Item",
  table: "items",
  propertyMappings: [],
  ids: ["Id"],
  readProperties: ["Id", "Name"],
  createProperties: ["Name"],
  createZeros: [],
  updateProperties: ["Name"],
  optionsByProvider,
  enrichments: [],
});

const dialect = (create: string[]): RepositoryModel => ({
  ...model(),
  createProperties: create,
});

describe("dialectsOf", () => {
  it("keeps engines whose options differ from the default", () => {
    const dialects = dialectsOf(
      model({
        sqlite: model(),
        mysql: dialect([]),
        postgres: dialect(["Id", "Name"]),
      }),
    );
    expect(dialects.map(([engine]) => engine)).toEqual(["mysql", "postgres"]);
  });
});

describe("datasourceEnginesOf", () => {
  it("skips an empty backend.datasources setting", async () => {
    const reader = memoryReader({});
    expect(await datasourceEnginesOf(reader, {})).toEqual([]);
    expect(await datasourceEnginesOf(reader, { "backend.datasources": " , " })).toEqual([]);
  });
});
