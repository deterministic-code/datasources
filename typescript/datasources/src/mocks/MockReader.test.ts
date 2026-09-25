import { describe, expect, it } from "vitest";
import { MockReader } from "../index.ts";

describe("MockReader", () => {
  it("walks mock reader rows and errors", async () => {
    const reader = new MockReader(["x"]);
    expect(await reader.next()).toBe(true);
    expect(reader.current).toBe("x");
    expect(await reader.next()).toBe(false);
    reader.nextError = new Error("next");
    await expect(reader.next()).rejects.toThrow("next");
  });
});
