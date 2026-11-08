import { describe, expect, it } from "vitest";
import type { Type } from "@deterministic-code/deterministic-generators-common/parser";
import { inferReferenceFieldTypes } from "./infer-reference-types.ts";

const typeOf = (name: string, fields: Type["fields"]): Type => ({
  name,
  tags: [],
  kind: "shaped",
  fields,
});

const field = (
  name: string,
  extra: Partial<Type["fields"][0]> & Pick<Type["fields"][0], "type">,
): Type["fields"][0] => ({
  name,
  kind: "primitive",
  base: extra.type,
  isArray: false,
  isNullable: false,
  ...extra,
});

describe("inferReferenceFieldTypes", () => {
  it("copies the parent column type onto an untyped reference", () => {
    const [integerChild] = inferReferenceFieldTypes([
      typeOf("contacts_base", [field("id", { type: "integer", isId: true })]),
      typeOf("address", [field("contact_id", { type: "string", references: "contacts_base.id" })]),
    ]).filter((type) => type.name === "address");
    expect(integerChild?.fields[0]?.type).toBe("integer");
    expect(integerChild?.fields[0]?.base).toBe("integer");
    const [bigintChild] = inferReferenceFieldTypes([
      typeOf("contacts_base", [field("id", { type: "biginteger", isId: true })]),
      typeOf("address", [field("contact_id", { type: "string", references: "contacts_base.id" })]),
    ]).filter((type) => type.name === "address");
    expect(bigintChild?.fields[0]?.type).toBe("biginteger");
    expect(bigintChild?.fields[0]?.base).toBe("biginteger");
    const [uuidChild] = inferReferenceFieldTypes([
      typeOf("contacts_base", [field("id", { type: "uuid", isId: true })]),
      typeOf("address", [field("contact_id", { type: "string", references: "contacts_base.id" })]),
    ]).filter((type) => type.name === "address");
    expect(uuidChild?.fields[0]?.type).toBe("uuid");
    expect(uuidChild?.fields[0]?.base).toBe("uuid");
  });
});
