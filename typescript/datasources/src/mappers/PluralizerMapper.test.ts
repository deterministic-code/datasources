import { describe, expect, it } from "vitest";
import { PluralizerMapper } from "./PluralizerMapper.ts";

describe("PluralizerMapper", () => {
  it("pluralizes and lowercases the table name", () => {
    const mapper = new PluralizerMapper();
    expect(mapper.map("User")).toBe("users");
    expect(mapper.reverseMap("users")).toBe("user");
  });
});
