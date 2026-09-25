import { describe, expect, it } from "vitest";
import { MockSelectStatement } from "../index.ts";

describe("MockSelectStatement", () => {
  it("counts text access", () => {
    const query = new MockSelectStatement("q");
    expect(query.getText()).toBe("q");
    expect(query.getTextCalls).toBe(1);
  });
});
