import { describe, expect, it } from "vitest";
import { EnrichedCRUDRepository } from "./EnrichedCRUDRepository.ts";
import { MockConnection } from "./mocks/mock-connection.ts";
import { CRUDRepository } from "./CRUDRepository.ts";
import { ListCRUDRepository } from "./tests/listCRUDRepository.ts";

class Item {
  id = 0;
}

describe("repository type", () => {
  it("reports constructor from typed construction", () => {
    const repo = new CRUDRepository(Item, new MockConnection());
    expect(repo.type).toBe(Item);
  });

  it("wrappers expose inner type", () => {
    const inner = new ListCRUDRepository<Item>([], Item);
    const repo = new EnrichedCRUDRepository(inner);
    expect(inner.type).toBe(Item);
    expect(repo.type).toBe(Item);
  });

  it("untyped options report Object", () => {
    const repo = new CRUDRepository({
      connection: new MockConnection(),
      entity: "items",
    });
    expect(repo.type).toBe(Object);
  });
});
