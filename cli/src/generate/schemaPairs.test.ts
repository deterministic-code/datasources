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
  createZeros: [],
  updateProperties: [],
  enrichments: [],
});

describe("schemaPairs", () => {
  it("puts inferred integer foreign keys on the table schema for a view", async () => {
    const spec = await DeterministicParser(memoryReader({ "types.yaml": yaml })).parse({});
    const pairs = schemaPairs(model("contact"), spec, createCasingStrategy("rust", {}));
    expect(pairs).toContainEqual({ from: "id", to: "integer" });
    expect(pairs).toContainEqual({ from: "gender_type_id", to: "integer" });
    expect(pairs).not.toContainEqual({ from: "name", to: "string" });
  });
});
