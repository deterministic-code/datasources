import { describe, expect, it } from "vitest";
import { MysqlDriver } from "../MysqlDriver.ts";
import { MysqlRowTypeConverters } from "./MysqlRowTypeConverters.ts";

describe("MysqlRowTypeConverters", () => {
  it("registers default types", () => {
    expect(Object.keys(new MysqlDriver().rowTypeConverters)).toEqual(Object.keys(MysqlRowTypeConverters));
  });
});
