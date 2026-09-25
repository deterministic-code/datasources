import { describe, expect, it } from "vitest";
import { identityFilter } from "./index.ts";

describe("identityFilter", () => {
  it("filters by id when present", () => {
    expect(identityFilter({ id: 9, name: "a" })).toEqual({ id: 9 });
  });

  it("copies the whole object when id is missing", () => {
    expect(identityFilter({ name: "a" })).toEqual({ name: "a" });
  });
});
