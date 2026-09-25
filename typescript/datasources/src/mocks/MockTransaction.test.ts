import { describe, expect, it } from "vitest";
import { MockConnection, MockTransaction } from "../index.ts";

describe("MockTransaction", () => {
  it("records transaction commit and abort", async () => {
    const connection = new MockConnection();
    const txn = new MockTransaction(connection);
    await txn.commit();
    expect(txn.committed).toBe(true);
    await txn.abort();
    expect(txn.aborted).toBe(true);
    txn.commitError = new Error("c");
    await expect(txn.commit()).rejects.toThrow("c");
    txn.abortError = new Error("a");
    await expect(txn.abort()).rejects.toThrow("a");
  });
});
