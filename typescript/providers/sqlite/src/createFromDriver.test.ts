import { CRUDRepository, Datasource } from "@deterministic-code/datasources";
import { expect, it } from "vitest";
import { SqliteDriver } from "./SqliteDriver.ts";

class Item {
  id = 0;
  name = "";
}

it("create from a driver", () => {
  const driver = new SqliteDriver();
  const ds = new Datasource("sqlite", driver, { DATASOURCE_URL: "sqlite:///:memory:" });
  const repo = ds.createRepository(Item);
  expect(repo instanceof CRUDRepository).toBe(true);
});
