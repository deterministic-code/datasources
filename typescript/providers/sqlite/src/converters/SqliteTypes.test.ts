import { describe, expect, it } from "vitest";
import { SqliteProvider } from "../SqliteProvider.ts";
import { SqliteTypes } from "./SqliteTypes.ts";

describe("SqliteTypes", () => {
  it("registers default types", () => {
    expect(Object.keys(new SqliteProvider().types)).toEqual(Object.keys(SqliteTypes));
  });
});
