import { describe, expect, it } from "vitest";
import { DefaultTypeConverters } from "@datasources/core";
import { PostgresTypes } from "./PostgresTypes.ts";

describe("PostgresTypes", () => {
  it("registers default types", () => {
    expect(Object.keys(DefaultTypeConverters.for("postgres"))).toEqual(
      Object.keys(PostgresTypes),
    );
  });
});
