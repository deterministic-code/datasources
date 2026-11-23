import { describe, expect, it } from "vitest";
import { BinaryConverter, isBase64 } from "./BinaryConverter.ts";

describe("BinaryConverter", () => {
  it("maps binary", () => {
    const converter = new BinaryConverter();
    const raw = new Uint8Array([1, 2, 3, 4]);
    expect(converter.convert(raw)).toBe(raw);
    expect(converter.convert("AQIDBA==")).toEqual(raw);
    expect(converter.convert(Buffer.from(raw).toString("latin1"))).toEqual(raw);
    expect(converter.convert("\\x01020304")).toEqual(raw);
    expect(converter.reverseConvert(raw)).toBe(raw);
    expect(converter.reverseConvert("AQIDBA==")).toEqual(raw);
    expect(isBase64("AA==")).toBe(true);
    expect(isBase64("ada")).toBe(false);
  });
});
