import { describe, expect, it } from "vitest";
import { DefaultTypeConverters } from "@datasources/core";
import { SqliteTypes } from "./SqliteTypes.ts";

describe("SqliteTypes", () => {
  it("registers default types", () => {
    expect(Object.keys(DefaultTypeConverters.for("sqlite"))).toEqual(
      Object.keys(SqliteTypes),
    );
    expect(DefaultTypeConverters.for("postgres")).toEqual({});
  });
});
