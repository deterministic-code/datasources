import { describe, expect, it } from "vitest";
import { PostgresProvider } from "../PostgresProvider.ts";
import { PostgresTypes } from "./PostgresTypes.ts";

describe("PostgresTypes", () => {
  it("registers default types", () => {
    expect(Object.keys(new PostgresProvider().types)).toEqual(Object.keys(PostgresTypes));
  });
});
