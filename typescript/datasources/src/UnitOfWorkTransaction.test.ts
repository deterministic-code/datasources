import { describe, expect, it } from "vitest";
import { DatasourceError, MockConnection, MockTransaction, UnitOfWorkTransaction } from "./index.ts";

describe("UnitOfWorkTransaction", () => {
  it("joins a transaction already open on the connection", async () => {
    const connection = new MockConnection();
    const txn = connection.transaction as MockTransaction;
    const outer = new UnitOfWorkTransaction(connection);
    await outer.begin();
    const inner = new UnitOfWorkTransaction(connection);
    await inner.begin();
    await inner.commit();
    await outer.commit();
    expect(connection.startTransactionCalls).toBe(1);
    expect(txn.commitCalls).toBe(1);
    expect(txn.abortCalls).toBe(0);
  });

  it("aborts the outer transaction once when a joined unit aborts", async () => {
    const connection = new MockConnection();
    const txn = connection.transaction as MockTransaction;
    const outer = new UnitOfWorkTransaction(connection);
    await outer.begin();
    const inner = new UnitOfWorkTransaction(connection);
    await inner.begin();
    await inner.abort();
    await outer.abort();
    expect(connection.startTransactionCalls).toBe(1);
    expect(txn.commitCalls).toBe(0);
    expect(txn.abortCalls).toBe(1);
  });

  it("rejects a second begin and a unit that never began", async () => {
    const failed = new MockConnection();
    failed.startTransactionError = new Error("nope");
    const broken = new UnitOfWorkTransaction(failed);
    await expect(broken.begin()).rejects.toThrow("nope");
    await expect(broken.begin()).rejects.toThrow(DatasourceError);
    const idle = new UnitOfWorkTransaction(new MockConnection());
    await expect(idle.commit()).rejects.toThrow("has not begun");
    await expect(idle.abort()).rejects.toThrow("has not begun");
  });
});
