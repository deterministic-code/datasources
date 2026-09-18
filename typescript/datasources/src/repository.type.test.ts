import { describe, expect, it } from "vitest";
import { DeepCRUDRepository } from "./DeepCRUDRepository.ts";
import { MockConnection } from "./mocks/mock-connection.ts";
import { Repository } from "./Repository.ts";
import { ListCRUDRepository } from "./tests/listCRUDRepository.ts";

class Item {
  id = 0;
}

describe("repository type", () => {
  it("reports constructor from typed construction", () => {
    const repo = new Repository(Item, new MockConnection());
    expect(repo.type).toBe(Item);
  });

  it("wrappers expose inner type", () => {
    const inner = new ListCRUDRepository<Item>([], Item);
    const repo = new DeepCRUDRepository(inner);
    expect(inner.type).toBe(Item);
    expect(repo.type).toBe(Item);
  });

  it("untyped options report Object", () => {
    const repo = new Repository({
      connection: new MockConnection(),
      entity: "items",
    });
    expect(repo.type).toBe(Object);
  });
});
