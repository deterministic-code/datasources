import { describe, expect, it } from "vitest";
import { DecimalConverter } from "./DecimalConverter.ts";

describe("DecimalConverter", () => {
  it("maps decimal", () => {
    const converter = new DecimalConverter();
    expect(converter.convert("10.5")).toBe("10.5");
    expect(converter.reverseConvert(10.5)).toBe("10.5");
    expect(converter.reverseConvert("10.5")).toBe("10.5");
  });
});
