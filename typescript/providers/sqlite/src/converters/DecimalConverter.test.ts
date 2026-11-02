import { describe, expect, it } from "vitest";
import { DecimalConverter } from "./DecimalConverter.ts";

describe("DecimalConverter", () => {
  it("maps decimal", () => {
    const converter = new DecimalConverter();
    expect(converter.convert(10.5)).toBe("10.5");
    expect(converter.convert(7)).toBe("7");
    expect(converter.convert(0)).toBe("0");
    expect(converter.convert("10.5")).toBe("10.5");
    expect(converter.convert("10.50")).toBe("10.5");
    expect(converter.convert(-1.5)).toBe("-1.5");
    expect(converter.convert("-10.50")).toBe("-10.5");
    expect(converter.convert(1e21)).toBe("1e+21");
    expect(converter.convert(Number.NaN)).toBe(Number.NaN);
    expect(converter.convert(1n)).toBe("1");
    expect(converter.convert("0.00")).toBe("0");
    expect(converter.convert("nope")).toBe("nope");
    expect(converter.convert(true)).toBe(true);
    expect(converter.reverseConvert(10.5)).toBe("10.5");
    expect(converter.reverseConvert("10.5")).toBe("10.5");
  });
});
