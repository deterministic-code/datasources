import { describe, expect, it } from "vitest";
import { UuidConverter } from "./UuidConverter.ts";

describe("UuidConverter", () => {
  it("maps uuid", () => {
    const converter = new UuidConverter();
    const id = "550e8400-e29b-41d4-a716-446655440000";
    const bytes = Uint8Array.from([
      0x55, 0x0e, 0x84, 0x00, 0xe2, 0x9b, 0x41, 0xd4, 0xa7, 0x16, 0x44, 0x66, 0x55, 0x44, 0x00, 0x00,
    ]);
    expect(converter.convert(id)).toBe(id);
    expect(converter.convert(id.toUpperCase())).toBe(id);
    expect(converter.convert(bytes)).toBe(id);
    expect(converter.reverseConvert(id)).toBe(id);
    expect(converter.convert("nope")).toBe("nope");
    expect(converter.convert(1)).toBe(1);
    expect(converter.convert(true)).toBe(true);
    expect(converter.reverseConvert(true)).toBe(true);
  });
});
