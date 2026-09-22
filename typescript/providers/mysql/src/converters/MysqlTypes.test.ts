import { describe, expect, it } from "vitest";
import { MysqlProvider } from "../MysqlProvider.ts";
import { MysqlTypes } from "./MysqlTypes.ts";

describe("MysqlTypes", () => {
  it("registers default types", () => {
    expect(Object.keys(new MysqlProvider().types)).toEqual(Object.keys(MysqlTypes));
  });
});
