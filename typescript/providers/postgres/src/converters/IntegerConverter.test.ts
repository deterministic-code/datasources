import { describe, expect, it } from "vitest";
import { IntegerConverter } from "./IntegerConverter.ts";

describe("IntegerConverter", () => {
  it("maps integer", () => {
    const converter = new IntegerConverter();
    expect(converter.convert("10")).toBe(10);
    expect(converter.convert("")).toBe(0);
    expect(converter.convert(7)).toBe(7);
    expect(converter.reverseConvert(7)).toBe(7);
  });
});
