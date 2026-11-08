import { memoryReader } from "@deterministic-code/generators-common/deterministic-reader";
import { DeterministicParser } from "@deterministic-code/deterministic-generators-common/parser";
import { describe, expect, it } from "vitest";
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
  it("uses the expanded view: remaining keys, no remove_fields", async () => {
    const spec = await DeterministicParser(memoryReader(files)).parse({});
    const contact = planRepositories(spec, "rust", {}).find((model) => model.specName === "contact");
    expect(contact?.schema).toContainEqual({ from: "id", type: "integer", nullable: false, hasDefault: false, isId: true });
    expect(contact?.schema).toContainEqual({ from: "name", type: "string", nullable: false, hasDefault: false, isId: false });
    expect(contact?.schema.some((column) => column.from === "gender_type_id")).toBe(false);
  });

  it("copies the parent id type onto an untyped reference", async () => {
    const spec = await DeterministicParser(memoryReader(files)).parse({});
    const genderId = spec.expandedTypes.find((type) => type.name === "gender")?.fields.find((item) => item.name === "id");
    const fk = spec.expandedTypes
      .find((type) => type.name === "contact_base")
      ?.fields.find((item) => item.name === "gender_type_id");
    expect(fk?.type).toBe(genderId?.type);
    expect(fk?.kind).toBe(genderId?.kind);
    expect(fk?.base).toBe(genderId?.base);
  });

  it("omits empty-type columns and ignores a dotted empty reference", async () => {
    const spec = await DeterministicParser(memoryReader(files)).parse({});
    const ghost = {
      name: "ghost",
      type: "",
      kind: "primitive" as const,
      base: "",
      isArray: false,
      isNullable: false,
      references: ".",
    };
    spec.types.find((type) => type.name === "gender")?.fields.push(ghost);
    spec.expandedTypes.find((type) => type.name === "gender")?.fields.push(ghost);
    const model = planRepositories(spec, "rust", {}).find((item) => item.specName === "gender");
    expect(model?.schema.some((column) => column.from === "ghost")).toBe(false);
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
