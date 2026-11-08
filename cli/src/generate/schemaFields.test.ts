import { createCasingStrategy } from "@deterministic-code/generators-common/casing-strategy";
import { memoryReader } from "@deterministic-code/generators-common/deterministic-reader";
import { DeterministicParser, type Type } from "@deterministic-code/deterministic-generators-common/parser";
import { describe, expect, it } from "vitest";
import { inferredSpec, schemaFields } from "./schemaFields.ts";
import { planRepositories } from "./RepositoryPlan.ts";

const files = {
  "types.yaml": `version: 1.0.0
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
      union:
        - gender
      extract:
        - gender.name
      mapping:
        gender.name: gender_type_name
      remove_fields:
        - gender_type_id
`,
  "datasource.yaml": `version: 1.0.0
includes:
  - types:
      filter: tag == "datasource_type"
types: []
`,
};

describe("schemaFields", () => {
  it("uses the expanded view: inferred remaining keys, no remove_fields", async () => {
    const spec = inferredSpec(await DeterministicParser(memoryReader(files)).parse({}));
    const expanded = spec.expandedTypes.find((type) => type.name === "contact");
    const property = (name: string) => createCasingStrategy("rust", {}).convertFields(name);
    const pairs = schemaFields(expanded, property);
    expect(pairs).toContainEqual({ from: "id", type: "integer", nullable: false, hasDefault: false, isId: true });
    expect(pairs).toContainEqual({ from: "name", type: "string", nullable: false, hasDefault: false, isId: false });
    expect(pairs.some((column) => column.from === "gender_type_id")).toBe(false);
    expect(schemaFields(undefined, property)).toEqual([]);
  });

  it("omits fields with an empty type", () => {
    const type = {
      name: "blank",
      fields: [{ name: "ghost", type: "", kind: "primitive" as const, base: "", isArray: false, isNullable: false }],
      tags: [],
    };
    expect(schemaFields(type as Type, (name) => name)).toEqual([]);
  });

  it("plans the view schema from the same expanded object", async () => {
    const spec = await DeterministicParser(memoryReader(files)).parse({});
    const contact = planRepositories(spec, "rust", {}).find((model) => model.specName === "contact");
    expect(contact?.schema.some((column) => column.from === "gender_type_id")).toBe(false);
    expect(contact?.schema.some((column) => column.from === "id" && column.type === "integer")).toBe(true);
    expect(contact?.propertyMappings).toEqual([]);
    const csharp = planRepositories(spec, "csharp", {}).find((model) => model.specName === "contact");
    expect(csharp?.propertyMappings).toContainEqual({ from: "GenderTypeId", to: "gender_type_id" });
    expect(csharp?.readProperties).toContain("GenderTypeId");
    expect(csharp?.createProperties).not.toContain("GenderTypeId");
    expect(csharp?.schema.some((column) => column.from === "GenderTypeId")).toBe(false);
  });
});
