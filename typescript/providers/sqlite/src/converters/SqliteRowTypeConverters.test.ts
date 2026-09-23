import { describe, expect, it } from "vitest";
import { SqliteDriver } from "../SqliteDriver.ts";
import { SqliteRowTypeConverters } from "./SqliteRowTypeConverters.ts";

describe("SqliteRowTypeConverters", () => {
  it("registers default types", () => {
    expect(Object.keys(new SqliteDriver().rowTypeConverters)).toEqual(Object.keys(SqliteRowTypeConverters));
  });
});
