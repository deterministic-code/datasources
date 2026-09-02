import { describe, expect, it } from "vitest";
import { DefaultTypeConverters } from "@datasources/core";
import { MysqlTypes } from "./MysqlTypes.ts";

describe("MysqlTypes", () => {
  it("registers default types", () => {
    expect(Object.keys(DefaultTypeConverters.for("mysql"))).toEqual(
      Object.keys(MysqlTypes),
    );
  });
});
