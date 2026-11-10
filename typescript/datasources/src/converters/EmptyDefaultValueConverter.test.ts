import { describe, expect, it } from "vitest";
import { emptyDefaultValueConverter } from "./EmptyDefaultValueConverter.ts";

describe("EmptyDefaultValueConverter", () => {
  it("convert returns null", () => {
    expect(emptyDefaultValueConverter.convert("string", "hi")).toBeNull();
  });

  it("reverseConvert throws", () => {
    expect(() => emptyDefaultValueConverter.reverseConvert("string", "hi")).toThrow(
      "Native default 'hi' is not a valid string default",
    );
  });
});
