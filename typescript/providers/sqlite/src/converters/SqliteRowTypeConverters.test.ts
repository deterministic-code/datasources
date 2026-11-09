import { describe, expect, it } from "vitest";
import { RowTypeConverter, Schema } from "@deterministic-code/datasources";
import { SqliteDriver } from "../SqliteDriver.ts";
import { reverse, SqliteRowTypeConverters } from "./SqliteRowTypeConverters.ts";

const floorSchema = () => new Schema({ floor: { type: "unsignedinteger", nullable: true, hasDefault: false } });

describe("SqliteRowTypeConverters", () => {
  it("registers default types", () => {
    expect(Object.keys(new SqliteDriver().rowTypeConverters)).toEqual(Object.keys(SqliteRowTypeConverters));
  });

  it("converts BIGINT text floor through unsignedinteger", () => {
    const types = floorSchema().types();
    expect(RowTypeConverter.from({ integer: SqliteRowTypeConverters.integer }, types).convert({ floor: "2" })).toEqual({
      floor: "2",
    });
    expect(RowTypeConverter.from(SqliteRowTypeConverters, types).convert({ floor: "2" })).toEqual({ floor: 2 });
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
