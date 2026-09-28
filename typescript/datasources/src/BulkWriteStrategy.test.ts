import { describe, expect, it } from "vitest";
import { BatchCommandIBulkWriteStrategy } from "./BatchCommandIBulkWriteStrategy.ts";
import { BulkCRUDRepository } from "./BulkCRUDRepository.ts";
import { MockCommand } from "./mocks/mock-command.ts";
import { MockConnection } from "./mocks/mock-connection.ts";
import { SingleCommands } from "./SingleCommands.ts";
import { StoredProcedureBulkWriteStrategy } from "./StoredProcedureBulkWriteStrategy.ts";
import type { IBulkWriteStrategy } from "./IBulkWriteStrategy.ts";

class Parent {
  id: number | null = null;
  label = "";
}

class Scalar {
  amount = 0;
  blob: Uint8Array | null = null;
  count = 0;
  created: Date | null = null;
  flag = false;
  label: string | null = null;
  price = 0;
  token: string | null = null;
}

const open = <T extends object>(type: new () => T, strategy: IBulkWriteStrategy<T>) => {
  const connection = new MockConnection(undefined, "memory");
  const command = new MockCommand(connection);
  connection.command = command;
  const repo = new BulkCRUDRepository<T>(connection, { type, entity: "parents" }, strategy);
  return { repo, command };
};

describe("bulk write strategies", () => {
  it("single commands writes one statement per row", async () => {
    const { repo, command } = open(Parent, new SingleCommands<Parent>());
    command.response = { result: 9, rowsAffected: 1 };
    await repo.createBulk([
      Object.assign(new Parent(), { label: "a" }),
      Object.assign(new Parent(), { label: "b" }),
    ]);
    await repo.updateBulk([
      Object.assign(new Parent(), { id: 1, label: "a" }),
      Object.assign(new Parent(), { id: 2, label: "b" }),
    ]);
    await repo.deleteBulk([
      Object.assign(new Parent(), { id: 1, label: "a" }),
      Object.assign(new Parent(), { id: 2, label: "b" }),
    ]);
    expect(command.executeCalls).toHaveLength(6);
    expect(command.executeCalls[0]?.text).toBe("INSERT INTO parents (label) VALUES ($label)");
    expect(command.executeCalls[1]?.text).toBe("INSERT INTO parents (label) VALUES ($label)");
    expect(command.executeCalls[2]?.text).toBe("UPDATE parents SET label = $label WHERE id = $id");
    expect(command.executeCalls[3]?.text).toBe("UPDATE parents SET label = $label WHERE id = $id");
    expect(command.executeCalls[4]?.text).toBe("DELETE FROM parents WHERE id = $id");
    expect(command.executeCalls[5]?.text).toBe("DELETE FROM parents WHERE id = $id");
  });

  it("batch commands joins statements", async () => {
    const { repo, command } = open(Parent, new BatchCommandIBulkWriteStrategy<Parent>());
    await repo.createBulk([
      Object.assign(new Parent(), { label: "a" }),
      Object.assign(new Parent(), { label: "b" }),
    ]);
    expect(command.executeCalls).toHaveLength(1);
    expect(command.executeCalls[0]?.text).toBe(
      "INSERT INTO parents (label) VALUES ($label__0); INSERT INTO parents (label) VALUES ($label__1)",
    );
    expect(command.executeCalls[0]?.parameters?.[0]).toMatchObject({ name: "label__0", value: "a" });
    expect(command.executeCalls[0]?.parameters?.[1]).toMatchObject({ name: "label__1", value: "b" });

    await repo.updateBulk([
      Object.assign(new Parent(), { id: 1, label: "a" }),
      Object.assign(new Parent(), { id: 2, label: "b" }),
    ]);
    expect(command.executeCalls[1]?.text).toBe(
      "UPDATE parents SET label = $label__0 WHERE id = $id__0; UPDATE parents SET label = $label__1 WHERE id = $id__1",
    );
    expect(command.executeCalls[1]?.parameters?.map((parameter) => parameter.name)).toEqual([
      "label__0",
      "id__0",
      "label__1",
      "id__1",
    ]);

    await repo.deleteBulk([
      Object.assign(new Parent(), { id: 1 }),
      Object.assign(new Parent(), { id: 2 }),
    ]);
    expect(command.executeCalls[2]?.text).toBe(
      "DELETE FROM parents WHERE id = $id__0; DELETE FROM parents WHERE id = $id__1",
    );
    expect(command.executeCalls[2]?.parameters?.map((parameter) => parameter.name)).toEqual(["id__0", "id__1"]);
  });

  it("stored procedure commands sends one json call", async () => {
    const { repo, command } = open(
      Parent,
      new StoredProcedureBulkWriteStrategy<Parent>("bulk_create_parent", "bulk_update_parent", "bulk_delete_parent"),
    );
    await repo.createBulk([
      Object.assign(new Parent(), { label: "a" }),
      Object.assign(new Parent(), { label: "b" }),
    ]);
    expect(command.executeCalls).toHaveLength(1);
    expect(command.executeCalls[0]?.text).toBe("CALL bulk_create_parent($payload)");
    expect(command.executeCalls[0]?.parameters?.[0]).toMatchObject({
      name: "payload",
      value: '[{"id":null,"label":"a"},{"id":null,"label":"b"}]',
    });

    await repo.updateBulk([
      Object.assign(new Parent(), { id: 1, label: "a" }),
      Object.assign(new Parent(), { id: 2, label: "b" }),
    ]);
    expect(command.executeCalls[1]?.text).toBe("CALL bulk_update_parent($payload)");
    expect(command.executeCalls[1]?.parameters?.[0]?.value).toBe(
      '[{"id":1,"label":"a"},{"id":2,"label":"b"}]',
    );

    await repo.deleteBulk([
      Object.assign(new Parent(), { id: 1 }),
      Object.assign(new Parent(), { id: 2 }),
    ]);
    expect(command.executeCalls[2]?.text).toBe("CALL bulk_delete_parent($payload)");
    expect(command.executeCalls[2]?.parameters?.[0]?.value).toBe(
      '[{"id":1,"label":""},{"id":2,"label":""}]',
    );
  });

  it("payload encodes scalars", async () => {
    const { repo, command } = open(
      Scalar,
      new StoredProcedureBulkWriteStrategy<Scalar>("create_scalar", "update_scalar", "delete_scalar"),
    );
    const created = new Date("2024-06-01T00:00:00.000Z");
    await repo.createBulk([
      Object.assign(new Scalar(), {
        amount: 1.25,
        blob: new Uint8Array([1, 2, 3]),
        count: 2,
        created,
        flag: true,
        price: 10.5,
        token: "3f2504e0-4f89-11d3-9a0c-0305e82c3301",
      }),
    ]);
    expect(command.executeCalls[0]?.parameters?.[0]?.value).toBe(
      `[{"amount":1.25,"blob":"AQID","count":2,"created":"${created.toISOString().replace(/\.(\d{3})Z$/, ".$10000Z")}","flag":true,"label":null,"price":10.5,"token":"3f2504e0-4f89-11d3-9a0c-0305e82c3301"}]`,
    );
  });

  it("empty list does not execute", async () => {
    for (const strategy of [
      new SingleCommands<Parent>(),
      new BatchCommandIBulkWriteStrategy<Parent>(),
      new StoredProcedureBulkWriteStrategy<Parent>("create_parent", "update_parent", "delete_parent"),
    ]) {
      const { repo, command } = open(Parent, strategy);
      const created = await repo.createBulk([]);
      const updated = await repo.updateBulk([]);
      const deleted = await repo.deleteBulk([]);
      expect(command.executeCalls).toEqual([]);
      expect(created.rowsAffected).toBe(0);
      expect(updated.rowsAffected).toBe(0);
      expect(deleted.rowsAffected).toBe(0);
      expect(created.result).toBeNull();
    }
  });
});
