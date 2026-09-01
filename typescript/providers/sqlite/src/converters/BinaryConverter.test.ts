import { describe, expect, it } from "vitest";
import { BinaryConverter } from "./BinaryConverter.ts";

describe("BinaryConverter", () => {
  it("maps binary", () => {
    const converter = new BinaryConverter();
    const bytes = new Uint8Array([1, 2]);
    expect(converter.convert(bytes)).toBe(bytes);
    expect(converter.reverseConvert(bytes)).toBe(bytes);
  });
});
