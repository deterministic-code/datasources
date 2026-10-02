import { describe, expect, it } from "vitest";
import { DefaultStatementTextRenderer, MockWhere } from "../index.ts";

describe("MockWhere", () => {
  it("tracks accept calls", () => {
    const where = new MockWhere("w");
    expect(where.accept({ visit: () => "" })).toBe("w");
    expect(where.acceptCalls).toBe(1);
    expect(DefaultStatementTextRenderer.instance.render(where)).toBe("w");
  });
});
