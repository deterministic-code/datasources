import { describe, expect, it } from "vitest";
import { RegexConverter } from "./RegexConverter.ts";

describe("RegexConverter", () => {
  it("replaces with regex", () => {
    expect(new RegexConverter(/\d+/g, "").convert("Ada 1 Lovelace")).toBe("Ada  Lovelace");
    expect(new RegexConverter("\\d+", "").convert("Ada 1 Lovelace")).toBe("Ada  Lovelace");
    expect(new RegexConverter(/id_(\d+)/, "id-$1").convert("id_7")).toBe("id-7");
    expect(new RegexConverter(/id_(\d+)/, "id-$1").reverseConvert("id_7")).toBe("id_7");
  });
});
