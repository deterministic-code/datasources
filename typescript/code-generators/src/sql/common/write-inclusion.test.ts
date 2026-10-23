import { describe, expect, it } from "vitest";
import { resolveWriteInclusion, writeInclusionOf } from "./write-inclusion.ts";
import type { TypeField } from "@deterministic-code/deterministic-generators-common/parser";

const field = (name: string): TypeField => ({
  name,
  type: name === "id" ? "integer" : "string",
  kind: "primitive",
  base: name === "id" ? "integer" : "string",
  isArray: false,
  isNullable: false,
});

describe("write inclusion", () => {
  it("defaults omit a single generated id", () => {
    expect(resolveWriteInclusion(undefined, undefined, true, false, true)).toEqual({
      includeInCreate: false,
      includeInUpdate: false,
    });
  });

  it("defaults include a fixed id on create", () => {
    expect(resolveWriteInclusion(undefined, undefined, true, true, false)).toEqual({
      includeInCreate: true,
      includeInUpdate: false,
    });
  });

  it("reads authored flags from the overlay", () => {
    expect(
      writeInclusionOf(field("created"), ["id"], { name: "created", includeInCreate: false, includeInUpdate: false })
        .includeInCreate,
    ).toBe(false);
  });
});
