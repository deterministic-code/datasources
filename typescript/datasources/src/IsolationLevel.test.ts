import { describe, expect, it } from "vitest";
import { IsolationLevel } from "./index.ts";

describe("IsolationLevel", () => {
  it("names the ANSI levels", () => {
    expect(IsolationLevel).toEqual({
      ReadUncommitted: "readUncommitted",
      ReadCommitted: "readCommitted",
      RepeatableRead: "repeatableRead",
      Serializable: "serializable",
    });
  });
});
