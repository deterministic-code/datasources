import { describe, expect, it } from "vitest";
import { UuidConverter } from "./UuidConverter.ts";

describe("UuidConverter", () => {
  it("maps uuid", () => {
    const converter = new UuidConverter();
    const id = "550e8400-e29b-41d4-a716-446655440000";
    expect(converter.convert(id)).toBe(id);
    expect(converter.reverseConvert(id)).toBe(id);
  });
});
