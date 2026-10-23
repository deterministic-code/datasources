import { describe, expect, it } from "vitest";
import type { RepositoryModel } from "./RepositoryModel.ts";
import { writeList } from "./writeLists.ts";
import { memoryReader } from "@deterministic-code/generators-common/deterministic-reader";
import { datasourceEnginesOf } from "./datasourceYaml.ts";

const model = (writesByProvider?: RepositoryModel["writesByProvider"]): RepositoryModel => ({
  specName: "item",
  typeName: "Item",
  typeHome: "datasource",
  entity: "Item",
  table: "items",
  propertyMappings: [],
  ids: ["Id"],
  readProperties: ["Id", "Name"],
  createProperties: ["Name"],
  updateProperties: ["Name"],
  writesByProvider,
  enrichments: [],
});

describe("writeList", () => {
  it("emits rust match arms for engines other than mysql and postgres", () => {
    const rust = writeList(
      "rust",
      model({
        sqlserver: { create: ["Name"], update: ["Name"] },
        mysql: { create: [], update: ["Name"] },
      }),
      "create",
    );
    expect(rust).toContain('"sqlserver" => vec!["Name".into()]');
    expect(writeList("rust", model(), "update")).toContain('vec!["Name".into()]');
  });

  it("falls back to the model list when sqlite is absent", () => {
    expect(
      writeList("csharp", model({ mysql: { create: [], update: ["Name"] } }), "create"),
    ).toContain('_ => ["Name"]');
  });
});

describe("datasourceEnginesOf", () => {
  it("skips an empty backend.datasources setting", async () => {
    const reader = memoryReader({});
    expect(await datasourceEnginesOf(reader, {})).toEqual([]);
    expect(await datasourceEnginesOf(reader, { "backend.datasources": " , " })).toEqual([]);
  });
});
