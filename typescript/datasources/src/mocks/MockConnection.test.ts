import { describe, expect, it } from "vitest";
import { IsolationLevel, MockCommand, MockConnection, MockTransaction } from "../index.ts";

describe("MockConnection", () => {
  it("records connection operations and errors", async () => {
    const connection = new MockConnection();
    await connection.open();
    expect(connection.openCalls).toBe(1);
    expect(connection.isOpen).toBe(true);
    const txn = await connection.startTransaction(IsolationLevel.ReadUncommitted);
    expect(txn).toBe(connection.transaction);
    expect(txn.isolation).toBe(IsolationLevel.ReadUncommitted);
    await connection.close();
    expect(connection.isOpen).toBe(false);

    connection.openError = new Error("open-fail");
    await expect(connection.open()).rejects.toThrow("open-fail");
    connection.closeError = new Error("close-fail");
    await expect(connection.close()).rejects.toThrow("close-fail");
    connection.startTransactionError = new Error("tx-fail");
    await expect(connection.startTransaction()).rejects.toThrow("tx-fail");

    const customTxn = new MockTransaction(connection);
    const withTxn = new MockConnection(customTxn);
    expect(withTxn.transaction).toBe(customTxn);
    const opened = new MockConnection();
    await opened.open();
    await expect(opened.open()).rejects.toThrow("already open");
  });

  it("creates a command when none is assigned", () => {
    expect(new MockConnection().createCommand()).toBeInstanceOf(MockCommand);
  });
});
