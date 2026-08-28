import type { IConnection } from "../IConnection.ts";
import type { ITransaction } from "../ITransaction.ts";
import { IsolationLevel, type IsolationLevel as Isolation } from "../IsolationLevel.ts";
import { MiddlewareHost } from "../MiddlewareHost.ts";
import { MockTransaction } from "./mock-transaction.ts";

export class MockConnection extends MiddlewareHost implements IConnection {
  openCalls = 0;
  closeCalls = 0;
  startTransactionCalls = 0;
  openError?: unknown;
  closeError?: unknown;
  startTransactionError?: unknown;
  transaction: ITransaction;
  opened = false;

  constructor(transaction?: ITransaction) {
    super();
    this.transaction = transaction ?? new MockTransaction(this);
  }

  async open(): Promise<void> {
    return this.run("open", async () => {
      this.openCalls += 1;
      if (this.openError) {
        throw this.openError;
      }
      this.opened = true;
    });
  }

  async close(): Promise<void> {
    return this.run("close", async () => {
      this.closeCalls += 1;
      if (this.closeError) {
        throw this.closeError;
      }
      this.opened = false;
    });
  }

  async startTransaction(isolation: Isolation = IsolationLevel.Serializable): Promise<ITransaction> {
    return this.run("startTransaction", async () => {
      this.startTransactionCalls += 1;
      if (this.startTransactionError) {
        throw this.startTransactionError;
      }
      if (this.transaction instanceof MockTransaction) {
        this.transaction.isolation = isolation;
      }
      return this.transaction;
    });
  }
}
