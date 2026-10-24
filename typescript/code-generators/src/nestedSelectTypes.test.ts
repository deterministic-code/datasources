import { describe, expect, it } from "vitest";
import type { Type } from "@deterministic-code/deterministic-generators-common/parser";
import { nestedSelectFromSpec } from "./nestedSelectTypes.ts";

const typed = (name: string, fields: Type["fields"]): Type =>
  ({
    name,
    tags: ["datasource_type"],
    kind: "shaped",
    fields,
  }) as Type;

describe("nestedSelectFromSpec", () => {
  it("builds a contacts graph from spec types", () => {
    const graph = nestedSelectFromSpec(
      [
        typed("contacts_base", [
          { name: "id", type: "integer", isId: true },
          { name: "first_name", type: "string" },
          { name: "contact_source_id", type: "integer", references: "contact_source.id" },
        ]),
        typed("contact_source", [
          { name: "id", type: "integer", isId: true },
          { name: "name", type: "string" },
        ]),
        typed("address", [
          { name: "id", type: "integer", isId: true },
          { name: "contact_id", type: "integer", references: "contacts_base.id" },
          { name: "line1", type: "string" },
        ]),
      ],
      "contacts_base",
    );
    expect(graph.name).toBe("ContactsBase");
    expect(graph.key).toBe("Id");
    expect(graph.properties).toContain("FirstName");
    expect(graph.relations.some((item) => item.name === "ContactSource" && item.kind === "object")).toBe(true);
    expect(graph.relations.some((item) => item.name === "Addresses" && item.kind === "list")).toBe(true);
  });
});
