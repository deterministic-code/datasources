import { afterEach, describe, expect, it } from "vitest";
import { Datasource } from "@datasources/core";
import { PostgresDatasource } from "./index.ts";

afterEach(() => Datasource.reset());

describe("postgres", () => {
  it("creates", () => {
    Datasource.use(PostgresDatasource.factory);
    expect(Datasource.create("postgres").provider).toBe("postgres");
  });
});
