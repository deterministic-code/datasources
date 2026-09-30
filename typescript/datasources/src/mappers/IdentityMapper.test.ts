import { describe, expect, it } from "vitest";
import { IdentityMapper } from "./IdentityMapper.ts";

describe("IdentityMapper", () => {
  it("maps and reverses the same value", () => {
    const mapper = new IdentityMapper<string>();
    const mapped = mapper.map("contact");
    const reversed = mapper.reverseMap("contact");
    expect(mapped).toBe("contact");
    expect(reversed).toBe("contact");
  });
});
