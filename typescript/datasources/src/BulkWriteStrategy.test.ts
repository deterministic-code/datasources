import { describe, expect, it } from "vitest";
import { DatasourceError } from "./DatasourceError.ts";
import { BatchCommandIBulkWriteStrategy } from "./BatchCommandIBulkWriteStrategy.ts";
import { BulkCRUDRepository } from "./BulkCRUDRepository.ts";
import { MockCommand } from "./mocks/mock-command.ts";
import { MockConnection } from "./mocks/mock-connection.ts";
import { SingleCommandIBulkWriteStrategy } from "./SingleCommandIBulkWriteStrategy.ts";
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
    const { repo, command } = open(Parent, new SingleCommandIBulkWriteStrategy<Parent>());
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

  it("leaves a shorter name inside a longer parameter", async () => {
    class Coded {
      id = 0;
      code = "";
      code_extra = "";
    }
    const { repo, command } = open(Coded, new BatchCommandIBulkWriteStrategy<Coded>());
    await repo.createBulk([Object.assign(new Coded(), { code: "a", code_extra: "ab" })]);
    expect(command.executeCalls[0]?.text).toContain("$code__0");
    expect(command.executeCalls[0]?.text).toContain("$code_extra__0");
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
    expect(command.executeCalls[0]?.text).toBe("CALL bulk_create_parent($payload, $ids)");
    expect(command.executeCalls[0]?.parameters?.[0]).toMatchObject({
      name: "payload",
      value: '[{"label":"a"},{"label":"b"}]',
    });
    expect(command.executeCalls[0]?.parameters?.[1]).toMatchObject({ name: "ids", direction: "inputOutput" });

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

  it("payload encodes the remaining scalars", async () => {
    class Odd {
      flag = false;
      huge = 0;
      note = "";
      rank = 0n;
      amy = 2;
      zed = 1;
    }
    const { repo, command } = open(
      Odd,
      new StoredProcedureBulkWriteStrategy<Odd>("create_odd", "update_odd", "delete_odd"),
    );
    await repo.createBulk([
      Object.assign(new Odd(), { flag: false, note: "a\u0001b", rank: 4n }),
    ]);
    expect(command.executeCalls[0]?.parameters?.[0]?.value).toContain('"flag":false');
    expect(command.executeCalls[0]?.parameters?.[0]?.value).toContain('"rank":4');
    expect(command.executeCalls[0]?.parameters?.[0]?.value).toContain("\\u0001");
    await expect(
      repo.createBulk([Object.assign(new Odd(), { huge: Number.POSITIVE_INFINITY })]),
    ).rejects.toThrow(DatasourceError);
    await expect(
      repo.createBulk([Object.assign(new Odd(), { note: { bad: true } as never })]),
    ).rejects.toThrow(DatasourceError);
  });

  class Child {
    id: number | null = null;
    parent_id: number | null = null;
    label = "";
  }

  class Family {
    id: number | null = null;
    label = "";
    children: Child[] = [];
  }

  const family = (
    parents: IBulkWriteStrategy<Family>,
    children: IBulkWriteStrategy<Child> = new BatchCommandIBulkWriteStrategy<Child>(),
  ) => {
    const connection = new MockConnection(undefined, "memory");
    const command = new MockCommand(connection);
    connection.command = command;
    const childrenRepo = new BulkCRUDRepository<Child>(connection, { type: Child, entity: "children" }, children);
    const parentsRepo = new BulkCRUDRepository<Family>(connection, { type: Family, entity: "parents" }, parents);
    parentsRepo.addEnriched(
      { destinationProperty: "children", sourceProperty: "id", relatedKey: "parent_id", type: "list" },
      childrenRepo,
    );
    return { parentsRepo, command };
  };

  it("batch persists children in one command", async () => {
    const { parentsRepo, command } = family(new BatchCommandIBulkWriteStrategy<Family>());
    await parentsRepo.createBulk([
      Object.assign(new Family(), { id: 1, label: "a", children: [Object.assign(new Child(), { id: 10, label: "x" })] }),
      Object.assign(new Family(), { id: 2, label: "b", children: [Object.assign(new Child(), { id: 11, label: "y" })] }),
    ]);
    expect(command.executeCalls).toHaveLength(2);
    expect(command.executeCalls[0]?.text).toContain("INSERT INTO parents");
    expect(command.executeCalls[1]?.text).toContain("INSERT INTO children");
    expect(command.executeCalls[1]?.text).toContain("label__0");
    expect(command.executeCalls[1]?.text).toContain("label__1");
  });

  it("stored procedure persists children in one call", async () => {
    const { parentsRepo, command } = family(
      new StoredProcedureBulkWriteStrategy<Family>("bulk_create_parent", "bulk_update_parent", "bulk_delete_parent"),
      new StoredProcedureBulkWriteStrategy<Child>("bulk_create_child", "bulk_update_child", "bulk_delete_child"),
    );
    await parentsRepo.createBulk([
      Object.assign(new Family(), { id: 1, label: "a", children: [Object.assign(new Child(), { id: 10, label: "x" })] }),
      Object.assign(new Family(), { id: 2, label: "b", children: [Object.assign(new Child(), { id: 11, label: "y" })] }),
    ]);
    expect(command.executeCalls).toHaveLength(2);
    expect(command.executeCalls[0]?.text).toBe("CALL bulk_create_parent($payload)");
    expect(command.executeCalls[1]?.text).toBe("CALL bulk_create_child($payload)");
  });

  it("stored procedure create writes generated ids onto items", async () => {
    const { repo, command } = open(
      Parent,
      new StoredProcedureBulkWriteStrategy<Parent>("bulk_create_parent", "bulk_update_parent", "bulk_delete_parent"),
    );
    command.response = {
      result: null,
      rowsAffected: 2,
      parameters: [
        { name: "payload", value: "[]" },
        { name: "ids", value: "[7,8]", direction: "inputOutput" },
      ],
    };
    const items = [Object.assign(new Parent(), { label: "a" }), Object.assign(new Parent(), { label: "b" })];
    await repo.createBulk(items);
    expect(items[0]?.id).toBe(7);
    expect(items[1]?.id).toBe(8);
  });

  it("batch create requires a parent id when children are present", async () => {
    const { parentsRepo } = family(new BatchCommandIBulkWriteStrategy<Family>());
    await expect(
      parentsRepo.createBulk([
        Object.assign(new Family(), { label: "a", children: [Object.assign(new Child(), { label: "x" })] }),
      ]),
    ).rejects.toThrow(/id/);
  });

  it("empty list does not execute", async () => {
    for (const strategy of [
      new SingleCommandIBulkWriteStrategy<Parent>(),
      new BatchCommandIBulkWriteStrategy<Parent>(),
      new StoredProcedureBulkWriteStrategy<Parent>("create_parent", "update_parent", "delete_parent"),
    ]) {
      const { repo, command } = open(Parent, strategy);
      const created = await repo.createBulk([]);
      const updated = await repo.updateBulk([]);
      const deleted = await repo.deleteBulk([]);
      expect(command.executeCalls).toEqual([]);
      expect(created).toEqual([]);
      expect(updated.rowsAffected).toBe(0);
      expect(deleted.rowsAffected).toBe(0);
    }
  });
});
