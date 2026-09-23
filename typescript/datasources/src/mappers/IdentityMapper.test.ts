import { describe, expect, it } from "vitest";
import { IdentityMapper } from "./IdentityMapper.ts";

describe("IdentityMapper", () => {
  it("maps and reverses the same value", () => {
    const mapper = new IdentityMapper<string>();
    expect(mapper.map("ada")).toBe("ada");
    expect(mapper.reverseMap("ada")).toBe("ada");
  });
});
