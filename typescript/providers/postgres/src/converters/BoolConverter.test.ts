import { describe, expect, it } from "vitest";
import { BoolConverter } from "./BoolConverter.ts";

describe("BoolConverter", () => {
  it("maps bool", () => {
    const converter = new BoolConverter();
    expect(converter.convert(1)).toBe(true);
    expect(converter.convert(0)).toBe(false);
    expect(converter.convert(true)).toBe(true);
    expect(converter.reverseConvert(true)).toBe(true);
    expect(converter.reverseConvert(false)).toBe(false);
    expect(converter.reverseConvert("x")).toBe("x");
  });
});
