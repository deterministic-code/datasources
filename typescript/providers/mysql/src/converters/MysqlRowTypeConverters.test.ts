import { describe, expect, it } from "vitest";
import { MysqlProvider } from "../MysqlProvider.ts";
import { MysqlRowTypeConverters } from "./MysqlRowTypeConverters.ts";

describe("MysqlRowTypeConverters", () => {
  it("registers default types", () => {
    expect(Object.keys(new MysqlProvider().rowTypeConverters)).toEqual(Object.keys(MysqlRowTypeConverters));
  });
});
