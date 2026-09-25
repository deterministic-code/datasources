import { describe, expect, it } from "vitest";
import { MockQuery } from "../index.ts";

describe("MockQuery", () => {
  it("counts text access", () => {
    const query = new MockQuery("q");
    expect(query.getText()).toBe("q");
    expect(query.getTextCalls).toBe(1);
  });
});
