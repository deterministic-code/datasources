import { describe, expect, it } from "vitest";
import { MemoryDriver } from "@datasources/memory";
import { MockConnection } from "./mocks/mock-connection.ts";
import { CRUDRepository } from "./CRUDRepository.ts";
import { ReadonlyRepository } from "./ReadonlyRepository.ts";

class Item {
  id = 0;
}

describe("repository type", () => {
  it("reports constructor from typed construction", () => {
    const repo = new CRUDRepository(Item, new MockConnection());
    expect(repo.type).toBe(Item);
    expect(new ReadonlyRepository(Item, new MockConnection()).type).toBe(Item);
  });

  it("crud repository reports item type", () => {
    const repo = new CRUDRepository(Item, new MemoryDriver().connection());
    expect(repo.type).toBe(Item);
  });

  it("untyped options report Object", () => {
    const repo = new CRUDRepository(new MockConnection(), {
      entity: "items",
    });
    expect(repo.type).toBe(Object);
  });
});
