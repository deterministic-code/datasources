import { afterEach, describe, expect, it } from "vitest";
import { Datasource } from "@datasources/core";
import { MongoDbDatasource } from "./index.ts";

afterEach(() => Datasource.reset());

describe("mongodb", () => {
  it("creates", () => {
    Datasource.use(MongoDbDatasource.factory);
    expect(Datasource.create("mongodb").provider).toBe("mongodb");
  });
});
