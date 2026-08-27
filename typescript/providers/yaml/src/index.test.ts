import { afterEach, describe, expect, it } from "vitest";
import { Datasource } from "@datasources/core";
import { YamlDatasource } from "./index.ts";

afterEach(() => Datasource.reset());

describe("yaml", () => {
  it("creates", () => {
    Datasource.use(YamlDatasource.factory);
    expect(Datasource.create("yaml").provider).toBe("yaml");
  });
});
