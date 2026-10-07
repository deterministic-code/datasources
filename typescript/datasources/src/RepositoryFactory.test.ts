import { describe, expect, it } from "vitest";
import type { IReadonlyRepository } from "./IReadonlyRepository.ts";
import { MockConnection } from "./mocks/mock-connection.ts";
import { MockRepository } from "./mocks/mock-repository.ts";
import { RepositoryFactory } from "./RepositoryFactory.ts";
import { RepositoryNotRegisteredError } from "./RepositoryNotRegisteredError.ts";

class Item {}

const emptyReadonly = (): IReadonlyRepository<Item> => ({
  type: Item,
  readSingle: async () => undefined,
  async *read() {},
});

describe("RepositoryFactory", () => {
  it("registers and creates", () => {
    const factory = new RepositoryFactory();
    const repo = new MockRepository<Item>(new MockConnection(), Item);
    factory.register(repo);
    const created = factory.create(Item);

    expect(created).toBe(repo);
    expect(factory.asReadonly.create(Item)).toBe(repo);
  });

  it("readonly registration is not crud", () => {
    const factory = new RepositoryFactory();
    const repo = emptyReadonly();
    factory.asReadonly.register(repo);

    expect(factory.asReadonly.create(Item)).toBe(repo);
    expect(() => factory.create(Item)).toThrow(RepositoryNotRegisteredError);
    try {
      factory.create(Item);
    } catch (error) {
      expect(error).toBeInstanceOf(RepositoryNotRegisteredError);
      expect((error as RepositoryNotRegisteredError).entityTypeName).toBe("Item");
      expect((error as Error).message).toContain("Item");
    }
  });

  it("registers and creates by name", () => {
    const factory = new RepositoryFactory();
    const connection = new MockConnection();
    const crud = new MockRepository<Item>(connection, Item);
    const read = emptyReadonly();
    factory.register("Contact", "contacts", () => crud, () => read);

    expect(factory.create("Contact", connection)).toBe(crud);
    expect(factory.createReadonly("contacts", connection)).toBe(read);
    expect(factory.access("contacts", connection).create()).toBe(crud);
    expect(factory.access("Contact", connection).createReadonly()).toBe(read);
  });

  it("throws when missing", () => {
    const factory = new RepositoryFactory();
    expect(() => factory.create(Item)).toThrow(RepositoryNotRegisteredError);
    expect(() => factory.create("Contacts", new MockConnection())).toThrow(RepositoryNotRegisteredError);
    expect(() => factory.createReadonly("Contacts", new MockConnection())).toThrow(RepositoryNotRegisteredError);
  });
});
