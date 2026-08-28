import type { IConnection } from "../IConnection.ts";
import type { ITransaction } from "../ITransaction.ts";
import { IsolationLevel, type IsolationLevel as Isolation } from "../IsolationLevel.ts";
import { MockTransaction } from "./mock-transaction.ts";

export class MockConnection implements IConnection {
  openCalls = 0;
  closeCalls = 0;
  startTransactionCalls = 0;
  openError?: unknown;
  closeError?: unknown;
  startTransactionError?: unknown;
  transaction: ITransaction;
  opened = false;

  constructor(transaction?: ITransaction) {
    this.transaction = transaction ?? new MockTransaction(this);
  }

  async open(): Promise<void> {
    this.openCalls += 1;
    if (this.openError) {
      throw this.openError;
    }
    this.opened = true;
  }

  async close(): Promise<void> {
    this.closeCalls += 1;
    if (this.closeError) {
      throw this.closeError;
    }
    this.opened = false;
  }

  async startTransaction(isolation: Isolation = IsolationLevel.Serializable): Promise<ITransaction> {
    this.startTransactionCalls += 1;
    if (this.startTransactionError) {
      throw this.startTransactionError;
    }
    if (this.transaction instanceof MockTransaction) {
      this.transaction.isolation = isolation;
    }
    return this.transaction;
  }
}
