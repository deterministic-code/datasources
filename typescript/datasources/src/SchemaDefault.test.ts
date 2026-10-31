import { describe, expect, it } from "vitest";
import { SchemaDefault } from "./SchemaDefault.ts";

describe("SchemaDefault", () => {
  it("maps a nullable integer to null", () => {
    expect(SchemaDefault.of({ type: "integer", nullable: true, hasDefault: false })).toBeNull();
  });

  it("maps a required integer to 0", () => {
    expect(SchemaDefault.of({ type: "integer", nullable: false, hasDefault: false })).toBe(0);
  });

  it("maps a required string to empty", () => {
    expect(SchemaDefault.of({ type: "string", nullable: false, hasDefault: false })).toBe("");
  });

  it("maps a required boolean to false", () => {
    expect(SchemaDefault.of({ type: "boolean", nullable: false, hasDefault: false })).toBe(false);
  });

  it("maps an unknown required type to null", () => {
    expect(SchemaDefault.of({ type: "uuid", nullable: false, hasDefault: false })).toBeNull();
  });
});
