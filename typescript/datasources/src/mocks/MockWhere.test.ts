import { describe, expect, it } from "vitest";
import { MockWhere } from "../index.ts";

describe("MockWhere", () => {
  it("counts text access", () => {
    const where = new MockWhere("w");
    expect(where.getText()).toBe("w");
    expect(where.getTextCalls).toBe(1);
  });
});
