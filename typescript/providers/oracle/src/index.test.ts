import { afterEach, describe, expect, it } from "vitest";
import { Datasource } from "@datasources/core";
import { OracleDatasource } from "./index.ts";

afterEach(() => Datasource.reset());

describe("oracle", () => {
  it("creates", () => {
    Datasource.use(OracleDatasource.factory);
    expect(Datasource.create("oracle").provider).toBe("oracle");
  });
});
