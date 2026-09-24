import { describe, expect, it } from "vitest";
import { MockConnection } from "./mocks/mock-connection.ts";
import { CRUDRepository } from "./CRUDRepository.ts";
import { ReadonlyRepository } from "./ReadonlyRepository.ts";
import { ListCRUDRepository } from "./tests/listCRUDRepository.ts";

class Item {
  id = 0;
}

describe("repository type", () => {
  it("reports constructor from typed construction", () => {
    const repo = new CRUDRepository(Item, new MockConnection());
    expect(repo.type).toBe(Item);
    expect(new ReadonlyRepository(Item, new MockConnection()).type).toBe(Item);
  });

  it("wrappers expose inner type", () => {
    const inner = new ListCRUDRepository<Item>([], Item);
    const repo = new CRUDRepository(inner);
    expect(inner.type).toBe(Item);
    expect(repo.type).toBe(Item);
  });

  it("untyped options report Object", () => {
    const repo = new CRUDRepository(new MockConnection(), {
      entity: "items",
    });
    expect(repo.type).toBe(Object);
  });
});
