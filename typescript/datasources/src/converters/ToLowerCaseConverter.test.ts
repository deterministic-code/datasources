import { describe, expect, it } from "vitest";
import { ToLowerCaseConverter } from "./ToLowerCaseConverter.ts";

describe("ToLowerCaseConverter", () => {
  it("lowers", () => {
    expect(new ToLowerCaseConverter().convert("ADA")).toBe("ada");
    expect(new ToLowerCaseConverter().reverseConvert("ADA")).toBe("ADA");
  });
});
