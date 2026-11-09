import { describe, expect, it } from "vitest";
import { RowTypeConverter, Schema } from "@deterministic-code/datasources";
import { PostgresDriver } from "../PostgresDriver.ts";
import { reverse, PostgresRowTypeConverters } from "./PostgresRowTypeConverters.ts";

const floorSchema = () => new Schema({ floor: { type: "unsignedinteger", nullable: true, hasDefault: false } });

describe("PostgresRowTypeConverters", () => {
  it("registers default types", () => {
    expect(Object.keys(new PostgresDriver().rowTypeConverters)).toEqual(Object.keys(PostgresRowTypeConverters));
  });

  it("converts BIGINT text floor through unsignedinteger", () => {
    const types = floorSchema().types();
    expect(RowTypeConverter.from({ integer: PostgresRowTypeConverters.integer }, types).convert({ floor: "2" })).toEqual({
      floor: "2",
    });
    expect(RowTypeConverter.from(PostgresRowTypeConverters, types).convert({ floor: "2" })).toEqual({ floor: 2 });
  });

  it("reverses booleans, dates, and bytes", () => {
    const when = new Date("2024-01-02T00:00:00.000Z");
    expect(reverse(true)).toBe(true);
    expect(reverse(false)).toBe(false);
    expect(reverse(when)).toBe("2024-01-02T00:00:00.000Z");
    expect(reverse(new Uint8Array([1, 2]))).toEqual(new Uint8Array([1, 2]));
    expect(reverse("a")).toBe("a");
  });
});
