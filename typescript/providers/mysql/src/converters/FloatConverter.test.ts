import { describe, expect, it } from "vitest";
import { FloatConverter } from "./FloatConverter.ts";

describe("FloatConverter", () => {
  it("maps float", () => {
    const converter = new FloatConverter();
    expect(converter.convert(10.9)).toBe(10.9);
    expect(converter.convert(10n)).toBe(10);
    expect(converter.convert("1.2345678901234567")).toBe(Number("1.2345678901234567"));
    expect(converter.convert("")).toBe("");
    expect(converter.convert("nope")).toBe("nope");
    expect(converter.convert(true)).toBe(true);
    expect(converter.reverseConvert(10.9)).toBe(10.9);
  });
});
