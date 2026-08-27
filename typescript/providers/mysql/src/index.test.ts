import { afterEach, describe, expect, it } from "vitest";
import { Datasource } from "@datasources/core";
import { MysqlDatasource } from "./index.ts";

afterEach(() => Datasource.reset());

describe("mysql", () => {
  it("creates", () => {
    Datasource.use(MysqlDatasource.factory);
    expect(Datasource.create("mysql").provider).toBe("mysql");
  });
});
