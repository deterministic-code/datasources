import { describe, expect, it } from "vitest";
import type { IDeterministic, Type, TypeField } from "@deterministic-code/deterministic-generators-common/parser";
import { requireKnownReferenceTypes } from "./requireKnownReferenceTypes.ts";

const field = (references?: TypeField["references"]): TypeField => ({
  name: "user_id",
  type: "integer",
  kind: "primitive",
  base: "integer",
  isArray: false,
  isNullable: false,
  ...(references === undefined ? {} : { references }),
});

const named = (name: string, fields: TypeField[]): Type => ({
  name,
  tags: [],
  kind: "inherit",
  fields,
});

const specOf = (types: Type[]): IDeterministic => ({
  types,
  expandedTypes: types,
  datasource: [],
  datasourceSeeds: new Map(),
  services: { generics: [], customs: [] },
  routes: { candidates: [], customs: [], nested: [], childrenOnly: new Set(), datasource: [] },
});

describe("requireKnownReferenceTypes", () => {
  it("accepts a field with no references", () => {
    expect(() => requireKnownReferenceTypes(specOf([named("review", [field()])]))).not.toThrow();
  });

  it("accepts a tuple reference to a known type", () => {
    expect(() =>
      requireKnownReferenceTypes(specOf([named("user", [field()]), named("review", [field(["user", "id"])])])),
    ).not.toThrow();
  });

  it("ignores an empty string reference", () => {
    expect(() => requireKnownReferenceTypes(specOf([named("review", [field(".id")])]))).not.toThrow();
  });

  it("throws missing type for a string reference", () => {
    expect(() => requireKnownReferenceTypes(specOf([named("review", [field("user.id")])]))).toThrow(
      'missing type "user"',
    );
  });

  it("throws missing type for a tuple reference", () => {
    expect(() => requireKnownReferenceTypes(specOf([named("review", [field(["user", "id"])])]))).toThrow(
      'missing type "user"',
    );
  });
});
