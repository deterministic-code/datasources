import { createCasingStrategy } from "@deterministic-code/generators-common/casing-strategy";
import { memoryReader } from "@deterministic-code/generators-common/deterministic-reader";
import { DeterministicParser } from "@deterministic-code/deterministic-generators-common/parser";
import { describe, expect, it } from "vitest";
import { schemaPairs } from "./schemaPairs.ts";
import type { RepositoryModel } from "./RepositoryModel.ts";

const yaml = `version: 1.0.0
types:
  - gender:
      tags: [datasource_type]
      fields:
        - id:
            type: integer
            is_id: true
        - name:
            type: string
  - contact_base:
      tags: [datasource_type]
      fields:
        - id:
            type: integer
            is_id: true
        - name:
            type: string
        - gender_type_id:
            references: gender.id
  - contact:
      tags: [view_type]
      inherits: contact_base
      remove_fields:
        - gender_type_id
`;

const model = (specName: string): RepositoryModel => ({
  specName,
  typeName: "Contact",
  typeHome: "view",
  entity: "Contact",
  table: "contacts",
  propertyMappings: [],
  ids: ["id"],
  readProperties: [],
  createProperties: [],
  updateProperties: [],
  enrichments: [],
});

describe("schemaPairs", () => {
  it("puts inferred integer foreign keys on the table schema for a view", async () => {
    const spec = await DeterministicParser(memoryReader({ "types.yaml": yaml })).parse({});
    const pairs = schemaPairs(model("contact"), spec, createCasingStrategy("rust", {}));
    expect(pairs).toContainEqual({ from: "id", type: "integer", nullable: false, hasDefault: false });
    expect(pairs).toContainEqual({ from: "gender_type_id", type: "integer", nullable: false, hasDefault: false });
    expect(pairs).toContainEqual({ from: "name", type: "string", nullable: false, hasDefault: false });
    expect(schemaPairs(model("missing"), undefined, createCasingStrategy("rust", {}))).toEqual([]);
  });

  it("omits fields with an empty type", () => {
    const spec = {
      types: [{ name: "blank", fields: [{ name: "ghost", type: "" }], tags: [] }],
      expandedTypes: [{ name: "blank", fields: [{ name: "ghost", type: "" }], tags: [] }],
    };
    expect(schemaPairs(model("blank"), spec as never, createCasingStrategy("rust", {}))).toEqual([]);
  });
});
