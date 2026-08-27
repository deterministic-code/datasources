import { afterEach, describe, expect, it } from "vitest";
import { Datasource } from "@datasources/core";
import { XmlDatasource } from "./index.ts";

afterEach(() => Datasource.reset());

describe("xml", () => {
  it("creates", () => {
    Datasource.use(XmlDatasource.factory);
    expect(Datasource.create("xml").provider).toBe("xml");
  });
});
