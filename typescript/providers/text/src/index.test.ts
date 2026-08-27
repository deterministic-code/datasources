import { afterEach, describe, expect, it } from "vitest";
import { Datasource } from "@datasources/core";
import { TextDatasource } from "./index.ts";

afterEach(() => Datasource.reset());

describe("text", () => {
  it("creates", () => {
    Datasource.use(TextDatasource.factory);
    expect(Datasource.create("text").provider).toBe("text");
  });
});
