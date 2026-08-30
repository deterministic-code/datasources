import { describe, expect, it } from "vitest";
import { TypeConverter } from "./TypeConverter.ts";
import { RowConverter } from "./RowConverter.ts";
import type { Cells } from "./Cell.ts";
import type { Constructable } from "./Constructable.ts";

class User {
  id = 0;
  username = "";
  isEnabled = false;
}

class Empty {}

class SkipsUndefined {
  id = 1;
  ghost = undefined;
}

class Mixed {
  label = "";
  count = 1n;
}

const asType = <T>(type: new () => T) => type as Constructable<Cells>;

describe("TypeConverter", () => {
  it("round-trips a row with the same keys", () => {
    const converter = new TypeConverter("User", asType(User));
    const user = converter.convert({
      id: 7,
      username: "ada",
      isEnabled: true,
    });
    expect(user).toBeInstanceOf(User);
    expect(user.id).toBe(7);
    expect(user.username).toBe("ada");
    expect(user.isEnabled).toBe(true);
    expect(converter.reverseConvert(user)).toEqual({
      id: 7,
      username: "ada",
      isEnabled: true,
    });
  });

  it("maps without a type using identity", () => {
    const converter = new TypeConverter("items");
    expect(converter.convert({ display_name: "a" })).toEqual({ display_name: "a" });
    expect(converter.reverseConvert({ displayName: "a" })).toEqual({ displayName: "a" });
  });

  it("lists constructable keys", () => {
    expect(RowConverter.keys()).toEqual([]);
    expect(RowConverter.keys(asType(Empty))).toEqual([]);
    expect(RowConverter.keys(asType(SkipsUndefined))).toEqual(["id"]);
    expect(new TypeConverter("Mixed", asType(Mixed)).convert({ count: 1n, label: "x" })).toMatchObject({
      count: 1n,
      label: "x",
    });
  });
});
