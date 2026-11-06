import { describe, expect, it } from "vitest";
import { BinaryConverter } from "./BinaryConverter.ts";

describe("BinaryConverter", () => {
  it("maps binary", () => {
    const converter = new BinaryConverter();
    const raw = new Uint8Array([1, 2]);
    expect(converter.convert(raw)).toBe("AQI=");
    expect(converter.convert("AQI=")).toBe("AQI=");
    expect(converter.reverseConvert("AQI=")).toEqual(raw);
    expect(converter.reverseConvert(raw)).toBe(raw);
  });
});
