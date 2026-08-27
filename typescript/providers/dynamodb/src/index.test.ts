import { afterEach, describe, expect, it } from "vitest";
import { Datasource } from "@datasources/core";
import { DynamoDbDatasource } from "./index.ts";

afterEach(() => Datasource.reset());

describe("dynamodb", () => {
  it("creates", () => {
    Datasource.use(DynamoDbDatasource.factory);
    expect(Datasource.create("dynamodb").provider).toBe("dynamodb");
  });
});
