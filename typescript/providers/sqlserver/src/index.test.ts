import { afterEach, describe, expect, it } from "vitest";
import { Datasource } from "@datasources/core";
import { SqlServerDatasource } from "./index.ts";

afterEach(() => Datasource.reset());

describe("sqlserver", () => {
  it("creates", () => {
    Datasource.use(SqlServerDatasource.factory);
    expect(Datasource.create("sqlserver").provider).toBe("sqlserver");
  });
});
