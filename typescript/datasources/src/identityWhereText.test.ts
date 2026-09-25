import { describe, expect, it } from "vitest";
import { identityWhereText } from "./index.ts";

describe("identityWhereText", () => {
  it("filters by id when present", () => {
    expect(identityWhereText({ id: 9, name: "a" })).toBe("id = $id");
  });

  it("copies the whole object when id is missing", () => {
    expect(identityWhereText({ name: "a", city: "x" })).toBe(
      "name = $name AND city = $city",
    );
  });

  it("matches everything when the object has no keys", () => {
    expect(identityWhereText({})).toBe("1 = 1");
  });
});
