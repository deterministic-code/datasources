import { describe, expect, it } from "vitest";
import { MockConnection, MockTransaction, UnitOfWorkTransaction } from "./index.ts";

describe("UnitOfWorkTransaction", () => {
  it("joins a transaction already open on the connection", async () => {
    const connection = new MockConnection();
    const txn = connection.transaction as MockTransaction;
    await new UnitOfWorkTransaction(connection).doWork(async () => {
      await new UnitOfWorkTransaction(connection).doWork(async () => 0);
      return 0;
    });
    expect(connection.startTransactionCalls).toBe(1);
    expect(txn.commitCalls).toBe(1);
    expect(txn.abortCalls).toBe(0);
  });
});
