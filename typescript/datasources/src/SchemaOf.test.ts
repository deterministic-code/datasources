import { describe, expect, it } from "vitest";
import { SchemaOf } from "./SchemaOf.ts";

class User {
  id = 0;
  username = "";
  isEnabled = false;
  created = new Date(0);
  huge = 0n;
  blob = new Uint8Array();
}

describe("SchemaOf", () => {
  it("from maps language fields to canonical types", () => {
    const schema = SchemaOf.from(User);
    expect(schema.types()).toEqual({
      id: "integer",
      isEnabled: "boolean",
      created: "datetime",
      huge: "biginteger",
      blob: "binary",
    });
  });

  it("from Object is empty", () => {
    expect(SchemaOf.from(Object).types()).toEqual({});
  });
});
