import { describe, expect, it } from "vitest";
import { SqliteProvider } from "../SqliteProvider.ts";
import { SqliteRowTypeConverters } from "./SqliteRowTypeConverters.ts";

describe("SqliteRowTypeConverters", () => {
  it("registers default types", () => {
    expect(Object.keys(new SqliteProvider().rowTypeConverters)).toEqual(Object.keys(SqliteRowTypeConverters));
  });
});
