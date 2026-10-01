import { describe, expect, it } from "vitest";
import { MemoryDriver } from "@datasources/memory";
import { collectAsync } from "./collectAsync.ts";
import { BulkCRUDRepository } from "./BulkCRUDRepository.ts";
import { DefaultWriteEnrichmentStrategy } from "./DefaultWriteEnrichmentStrategy.ts";
import { DatasourceError } from "./DatasourceError.ts";
import type { IConnection } from "./IConnection.ts";

class Parent {
  id = 0;
  label = "";
  children: Child[] = [];
}

class Child {
  id = 0;
  parent_id = 0;
  label = "";
}

const setup = async (): Promise<IConnection> => {
  const connection = new MemoryDriver().connection();
  await connection.open();
  return connection;
};

const relate = (parentsRepo: BulkCRUDRepository<Parent>, childrenRepo: BulkCRUDRepository<Child>) => {
  parentsRepo.addEnriched({ destinationProperty: "children", sourceProperty: "id", relatedKey: "parent_id", type: "list" }, childrenRepo);
  return parentsRepo;
};

describe("BulkCRUDRepository", () => {
  it("creates parents and children", async () => {
    const connection = await setup();
    const parentsRepo = new BulkCRUDRepository(Parent, connection);
    const childrenRepo = new BulkCRUDRepository(Child, connection);
    const response = await relate(parentsRepo, childrenRepo).createBulk([
      Object.assign(new Parent(), { children: [Object.assign(new Child(), { label: "a" })] }),
      Object.assign(new Parent(), {
        children: [Object.assign(new Child(), { label: "b" }), Object.assign(new Child(), { label: "c" })],
      }),
    ]);
    const parents = await collectAsync(parentsRepo.read());
    const children = await collectAsync(childrenRepo.read());
    expect(parents).toHaveLength(2);
    expect(parents.every((parent) => parent.id > 0)).toBe(true);
    expect(children).toHaveLength(3);
    expect(response.rowsAffected).toBe(2);
    expect(response.result).toBe(Math.max(...parents.map((parent) => parent.id)));
  });

  it("createBulk writes each parent", async () => {
    const parentsRepo = new BulkCRUDRepository(Parent, await setup());
    const response = await parentsRepo.createBulk([
      Object.assign(new Parent(), { label: "a" }),
      Object.assign(new Parent(), { label: "b" }),
    ]);
    const parents = await collectAsync(parentsRepo.read());
    expect(parents).toHaveLength(2);
    expect(response.rowsAffected).toBe(2);
    expect(response.result).toBe(Math.max(...parents.map((parent) => parent.id)));
  });

  it("keeps the first write when a later bulk update throws", async () => {
    const parentsRepo = new BulkCRUDRepository(Parent, await setup());
    const first = await parentsRepo.create(Object.assign(new Parent(), { label: "a" }));
    expect(first.id).toBeGreaterThan(0);
    const second = await parentsRepo.create(Object.assign(new Parent(), { label: "b" }));
    expect(second.id).toBeGreaterThan(0);
    let seen = 0;
    parentsRepo.addMiddleware({
      beforeCommand() {
        seen += 1;
        if (seen === 2) throw new DatasourceError("stop");
      },
      afterCommand() {},
    });
    await expect(parentsRepo.updateBulk([
      Object.assign(new Parent(), { id: first.id, label: "changed" }),
      Object.assign(new Parent(), { id: first.id, label: "later" }),
    ])).rejects.toThrow(/stop/);
    const stored = await collectAsync(parentsRepo.read());
    expect(stored.find((parent) => parent.id === first.id)?.label).toBe("changed");
  });

  it("rolls back a bulk update when useTransaction is on", async () => {
    const parentsRepo = new BulkCRUDRepository<Parent>(await setup(), {
      type: Parent,
      write: new DefaultWriteEnrichmentStrategy(undefined, undefined, undefined, true),
    });
    const first = await parentsRepo.create(Object.assign(new Parent(), { label: "a" }));
    expect(first.id).toBeGreaterThan(0);
    const second = await parentsRepo.create(Object.assign(new Parent(), { label: "b" }));
    expect(second.id).toBeGreaterThan(0);
    let seen = 0;
    parentsRepo.addMiddleware({
      beforeCommand() {
        seen += 1;
        if (seen === 2) throw new DatasourceError("stop");
      },
      afterCommand() {},
    });
    await expect(parentsRepo.updateBulk([
      Object.assign(new Parent(), { id: first.id, label: "changed" }),
      Object.assign(new Parent(), { id: first.id, label: "later" }),
    ])).rejects.toThrow(/stop/);
    const stored = await collectAsync(parentsRepo.read());
    expect(stored.find((parent) => parent.id === first.id)?.label).toBe("a");
  });

  it("update replace-syncs each parent", async () => {
    const connection = await setup();
    const parentsRepo = new BulkCRUDRepository(Parent, connection);
    const childrenRepo = new BulkCRUDRepository(Child, connection);
    const first = await parentsRepo.create(Object.assign(new Parent(), { label: "one" }));
    expect(first.id).toBeGreaterThan(0);
    const second = await parentsRepo.create(Object.assign(new Parent(), { label: "two" }));
    expect(second.id).toBeGreaterThan(0);
    const kept = await childrenRepo.create(Object.assign(new Child(), { parent_id: first.id, label: "a" }));
    expect(kept.id).toBeGreaterThan(0);
    const dropped = await childrenRepo.create(Object.assign(new Child(), { parent_id: second.id, label: "b" }));
    expect(dropped.id).toBeGreaterThan(0);
    const response = await relate(parentsRepo, childrenRepo).updateBulk([
      Object.assign(new Parent(), {
        id: first.id,
        label: "one",
        children: [Object.assign(new Child(), { id: kept.id, label: "aa" })],
      }),
      Object.assign(new Parent(), {
        id: second.id,
        label: "two",
        children: [Object.assign(new Child(), { label: "c" })],
      }),
    ]);
    expect(response.rowsAffected).toBe(2);
    const children = await collectAsync(childrenRepo.read());
    expect(children).toHaveLength(2);
    expect(children.find((child) => child.parent_id === first.id)?.label).toBe("aa");
    expect(children.find((child) => child.parent_id === second.id)?.label).toBe("c");
    expect(children.some((child) => child.label === "b")).toBe(false);
  });

  it("delete cascades each parent", async () => {
    const connection = await setup();
    const parentsRepo = new BulkCRUDRepository(Parent, connection);
    const childrenRepo = new BulkCRUDRepository(Child, connection);
    const first = await parentsRepo.create(Object.assign(new Parent(), { label: "one" }));
    expect(first.id).toBeGreaterThan(0);
    const second = await parentsRepo.create(Object.assign(new Parent(), { label: "two" }));
    expect(second.id).toBeGreaterThan(0);
    const firstChild = await childrenRepo.create(Object.assign(new Child(), { parent_id: first.id, label: "a" }));
    expect(firstChild.id).toBeGreaterThan(0);
    const secondChild = await childrenRepo.create(Object.assign(new Child(), { parent_id: second.id, label: "b" }));
    expect(secondChild.id).toBeGreaterThan(0);
    const kept = await childrenRepo.create(Object.assign(new Child(), { label: "keep" }));
    expect(kept.id).toBeGreaterThan(0);
    const response = await relate(parentsRepo, childrenRepo).deleteBulk([
      Object.assign(new Parent(), { id: first.id }),
      Object.assign(new Parent(), { id: second.id }),
    ]);
    expect(response.rowsAffected).toBe(2);
    expect(await collectAsync(parentsRepo.read())).toEqual([]);
    expect((await collectAsync(childrenRepo.read())).map((child) => child.label)).toEqual(["keep"]);
  });
});
