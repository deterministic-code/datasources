import { describe, expect, it } from "vitest";
import { DefaultStatementTextRenderer, MockSelectStatement } from "../index.ts";

describe("MockSelectStatement", () => {
  it("tracks accept calls", () => {
    const query = new MockSelectStatement("q");
    expect(query.accept({ visit: () => "" })).toBe("q");
    expect(query.acceptCalls).toBe(1);
    expect(DefaultStatementTextRenderer.instance.render(query)).toBe("q");
  });
});
