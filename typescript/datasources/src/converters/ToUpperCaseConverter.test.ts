import { describe, expect, it } from "vitest";
import { ToUpperCaseConverter } from "./ToUpperCaseConverter.ts";

describe("ToUpperCaseConverter", () => {
  it("uppers", () => {
    expect(new ToUpperCaseConverter().convert("ada")).toBe("ADA");
    expect(new ToUpperCaseConverter().reverseConvert("ada")).toBe("ada");
  });
});
