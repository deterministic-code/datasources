import { describe, expect, it } from "vitest";
import { PostgresDriver } from "../PostgresDriver.ts";
import { reverse, PostgresRowTypeConverters } from "./PostgresRowTypeConverters.ts";

describe("PostgresRowTypeConverters", () => {
  it("registers default types", () => {
    expect(Object.keys(new PostgresDriver().rowTypeConverters)).toEqual(Object.keys(PostgresRowTypeConverters));
  });

  it("reverses booleans, dates, and bytes", () => {
    const when = new Date("2024-01-02T00:00:00.000Z");
    expect(reverse(true)).toBe(true);
    expect(reverse(false)).toBe(false);
    expect(reverse(when)).toBe(when);
    expect(reverse(new Uint8Array([1, 2]))).toEqual(new Uint8Array([1, 2]));
    expect(reverse("a")).toBe("a");
  });
});
