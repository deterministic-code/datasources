import { describe, expect, it } from "vitest";
import { MockConnection, MockTransaction, UnitOfWork } from "./index.ts";

describe("UnitOfWork", () => {
  it("joins a transaction already open on the connection", async () => {
    const connection = new MockConnection();
    const txn = connection.transaction as MockTransaction;
    await new UnitOfWork(connection).inTransaction(async () => {
      await new UnitOfWork(connection).inTransaction(async () => 0);
      return 0;
    });
    expect(connection.startTransactionCalls).toBe(1);
    expect(txn.commitCalls).toBe(1);
    expect(txn.abortCalls).toBe(0);
  });
});
