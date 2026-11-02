import { describe, expect, it } from "vitest";
import { defaultId } from "./createCRUDRepositoryOptions.ts";
import { identityKey, idsFromSchema } from "./identity.ts";
import { Schema } from "./Schema.ts";

describe("identity", () => {
  it("reads is_id names from schema", () => {
    const schema = new Schema({
      Key: { type: "string", nullable: false, hasDefault: false, isId: true },
      Name: { type: "string", nullable: false, hasDefault: false, isId: false },
    });
    expect(idsFromSchema(schema)).toEqual(["Key"]);
    expect(defaultId(schema)).toEqual(["Key"]);
    expect(defaultId(undefined, ["Id"])).toEqual(["Id"]);
    expect(defaultId(undefined, ["name"])).toEqual([]);
    expect(identityKey({ Key: "home", Name: "house" }, "Key")).toBe("home");
  });
});
