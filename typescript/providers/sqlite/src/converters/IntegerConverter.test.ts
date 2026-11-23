import { describe, expect, it } from "vitest";
import { DatasourceError } from "@deterministic-code/datasources";
import { IntegerConverter } from "./IntegerConverter.ts";

describe("IntegerConverter", () => {
  it("maps integer", () => {
    const converter = new IntegerConverter();
    expect(converter.convert("10")).toBe(10);
    expect(converter.convert("0")).toBe(0);
    expect(() => converter.convert("")).toThrow(DatasourceError);
    expect(converter.convert("1.0")).toBe(1);
    expect(converter.convert("10.0")).toBe(10);
    expect(converter.convert("10.00000000000000")).toBe(10);
    expect(() => converter.convert("10.5")).toThrow(DatasourceError);
    expect(() => converter.convert("10.50000000000000")).toThrow(DatasourceError);
    expect(converter.convert(10.0)).toBe(10);
    expect(converter.convert(7)).toBe(7);
    expect(converter.convert(10.0)).toBe(10);
    expect(converter.convert(10.9)).toBe(10.9);
    expect(converter.reverseConvert(7)).toBe(7n);
    expect(converter.reverseConvert(10)).toBe(10n);
    expect(converter.reverseConvert(10.9)).toBe(10.9);
  });
});
