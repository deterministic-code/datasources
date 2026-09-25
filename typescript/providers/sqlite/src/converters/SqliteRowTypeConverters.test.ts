import { describe, expect, it } from "vitest";
import { SqliteDriver } from "../SqliteDriver.ts";
import { reverse, SqliteRowTypeConverters } from "./SqliteRowTypeConverters.ts";

describe("SqliteRowTypeConverters", () => {
  it("registers default types", () => {
    expect(Object.keys(new SqliteDriver().rowTypeConverters)).toEqual(Object.keys(SqliteRowTypeConverters));
  });

  it("reverses booleans, dates, and bytes", () => {
    const when = new Date("2024-01-02T00:00:00.000Z");
    expect(reverse(true)).toBe(1);
    expect(reverse(false)).toBe(0);
    expect(reverse(when)).toBe(when.toISOString());
    expect(reverse(new Uint8Array([1, 2]))).toEqual(new Uint8Array([1, 2]));
    expect(reverse("a")).toBe("a");
  });
});
