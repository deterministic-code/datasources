import { describe, expect, it } from "vitest";
import { PostgresDriver } from "../PostgresDriver.ts";
import { PostgresRowTypeConverters } from "./PostgresRowTypeConverters.ts";

describe("PostgresRowTypeConverters", () => {
  it("registers default types", () => {
    expect(Object.keys(new PostgresDriver().rowTypeConverters)).toEqual(Object.keys(PostgresRowTypeConverters));
  });
});
