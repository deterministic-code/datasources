import { describe, expect, it } from "vitest";
import { PostgresProvider } from "../PostgresProvider.ts";
import { PostgresRowTypeConverters } from "./PostgresRowTypeConverters.ts";

describe("PostgresRowTypeConverters", () => {
  it("registers default types", () => {
    expect(Object.keys(new PostgresProvider().rowTypeConverters)).toEqual(Object.keys(PostgresRowTypeConverters));
  });
});
