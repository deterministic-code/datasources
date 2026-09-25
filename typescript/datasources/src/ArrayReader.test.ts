import { describe, expect, it } from "vitest";
import { ArrayReader } from "./index.ts";

describe("ArrayReader", () => {
  it("walks an array including the empty case", async () => {
    const empty = new ArrayReader([]);
    expect(empty.current).toBeUndefined();
    expect(await empty.next()).toBe(false);
    expect(empty.current).toBeUndefined();

    const reader = new ArrayReader([{ id: 1 }, { id: 2 }]);
    expect(await reader.next()).toBe(true);
    expect(reader.current).toEqual({ id: 1 });
    expect(await reader.next()).toBe(true);
    expect(reader.current).toEqual({ id: 2 });
    expect(await reader.next()).toBe(false);
    expect(reader.current).toBeUndefined();
  });
});
