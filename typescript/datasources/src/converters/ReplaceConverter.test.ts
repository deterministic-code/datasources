import { describe, expect, it } from "vitest";
import { ReplaceConverter } from "./ReplaceConverter.ts";

describe("ReplaceConverter", () => {
  it("replaces literals", () => {
    expect(new ReplaceConverter("_", " ").convert("Ada_Lovelace")).toBe("Ada Lovelace");
    expect(new ReplaceConverter("_", " ").reverseConvert("Ada Lovelace")).toBe("Ada_Lovelace");
  });
});
