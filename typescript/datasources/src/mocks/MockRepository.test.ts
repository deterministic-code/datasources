import { describe, expect, it } from "vitest";
import { collectAsync, MockConnection, MockSelectStatement, MockRepository, MockWhere } from "../index.ts";
import { tracked } from "../ReadonlyRepository.ts";

describe("MockRepository", () => {
  it("records repository CRUD and errors", async () => {
    const repo = new MockRepository<{ id: number }>(new MockConnection());
    expect(await repo.readSingle()).toBeUndefined();
    repo.items = [{ id: 1 }, { id: 2 }];
    expect(await repo.readSingle(new MockSelectStatement("id=1"))).toEqual({ id: 1 });
    expect(await collectAsync(repo.read())).toEqual([{ id: 1 }, { id: 2 }]);
    expect(await collectAsync(repo.read(new MockSelectStatement("all")))).toHaveLength(2);
    expect(await repo.readSingle(new MockWhere("id=1"))).toEqual({ id: 1 });
    expect(await collectAsync(repo.read(new MockWhere("all")))).toHaveLength(2);
    expect(await collectAsync(repo[tracked]())).toEqual([{ item: { id: 1 } }, { item: { id: 2 } }]);
    expect(await repo.readSingleById(1)).toEqual({ id: 1 });
    expect(await repo.create({ id: 3 })).toEqual({ id: 3 });
    expect(
      (await repo.update({ id: 3 }, new MockWhere("id=3"))).rowsAffected,
    ).toBe(1);
    expect((await repo.delete({ id: 3 })).rowsAffected).toBe(1);

    repo.readSingleError = new Error("gs");
    await expect(repo.readSingle()).rejects.toThrow("gs");
    repo.readError = new Error("g");
    await expect(collectAsync(repo.read())).rejects.toThrow("g");
    repo.addError = new Error("ad");
    await expect(repo.create({ id: 0 })).rejects.toThrow("ad");
    repo.updateError = new Error("up");
    await expect(repo.update({ id: 0 })).rejects.toThrow("up");
    repo.deleteError = new Error("del");
    await expect(repo.delete({ id: 0 })).rejects.toThrow("del");
  });
});
