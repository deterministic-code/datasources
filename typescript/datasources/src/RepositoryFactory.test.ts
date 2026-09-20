import { describe, expect, it } from "vitest";
import type { IReadonlyRepository } from "./IReadonlyRepository.ts";
import { MockConnection } from "./mocks/mock-connection.ts";
import { MockRepository } from "./mocks/mock-repository.ts";
import { RepositoryFactory } from "./RepositoryFactory.ts";
import { RepositoryNotRegisteredError } from "./RepositoryNotRegisteredError.ts";

class Item {}

const emptyReadonly = (): IReadonlyRepository => ({
  type: Object,
  readSingle: async () => undefined,
  async *read() {},
});

describe("RepositoryFactory", () => {
  it("registers and creates by type and name", () => {
    const factory = new RepositoryFactory();
    const repo = new MockRepository<Item>(new MockConnection());
    factory.register(Item, () => repo);

    expect(factory.create(Item)).toBe(repo);
    expect(factory.create("Item")).toBe(repo);
    expect(factory.createReadonly(Item)).toBe(repo);
    expect(factory.asReadonly.create(Item)).toBe(repo);
  });

  it("registers by name and type", () => {
    const factory = new RepositoryFactory();
    const named = new MockRepository<Item>(new MockConnection());
    const typed = new MockRepository<Item>(new MockConnection());
    factory.register("people", () => named);
    factory.register(Item, () => typed);

    expect(factory.create("people")).toBe(named);
    expect(factory.create(Item)).toBe(typed);
  });

  it("forwards connection", () => {
    const factory = new RepositoryFactory();
    const connection = new MockConnection();
    let seen: MockConnection | undefined;
    factory.register(Item, (conn) => {
      seen = conn as MockConnection;
      return new MockRepository<Item>(conn!);
    });

    factory.create(Item, connection);
    expect(seen).toBe(connection);
    factory.create("Item", connection);
    expect(seen).toBe(connection);
  });

  it("readonly registration is not crud", () => {
    const factory = new RepositoryFactory();
    const repo = emptyReadonly();
    factory.registerReadonly(Item, () => repo);

    expect(factory.createReadonly(Item)).toBe(repo);
    expect(factory.createReadonly("Item")).toBe(repo);
    expect(() => factory.create(Item)).toThrow(RepositoryNotRegisteredError);
    try {
      factory.create("Item");
    } catch (error) {
      expect(error).toBeInstanceOf(RepositoryNotRegisteredError);
      expect((error as RepositoryNotRegisteredError).entityTypeName).toBe("Item");
      expect((error as Error).message).toContain("Item");
    }
  });

  it("throws when missing", () => {
    const factory = new RepositoryFactory();
    expect(() => factory.create(Item)).toThrow(RepositoryNotRegisteredError);
    expect(() => factory.create("missing")).toThrow(RepositoryNotRegisteredError);
  });
});
