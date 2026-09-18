import {
  DatasourceError,
  IsolationLevel,
  type ICommand,
  type IConnection,
  type IsolationLevelName,
  type ITransaction,
} from "@datasources/core";
import { MemoryCommand } from "./command.ts";
import type { MemoryConnectionOptions } from "./connection-options.ts";
import { MemoryStore, type MemoryRecord } from "./store.ts";
import { MemoryTransaction } from "./transaction.ts";

export class MemoryConnection implements IConnection {
  readonly provider = "memory";
  store: MemoryStore;
  collection: string;
  isOpen = false;
  inTransaction = false;
  private snapshot?: Map<string, MemoryRecord[]>;
  private readonly data?: MemoryRecord[] | Record<string, MemoryRecord[]>;

  constructor(options: MemoryConnectionOptions = {}) {
    this.store = options.store ?? new MemoryStore();
    this.collection = options.collection ?? "";
    this.data = options.data;
  }

  createCommand = (): ICommand => new MemoryCommand(this);

  async open(): Promise<void> {
    if (this.isOpen) {
      return;
    }
    if (this.data !== undefined) {
      this.store.seed(this.data, this.collection);
    }
    this.store.getCollection(this.collection);
    this.isOpen = true;
  }

  async close(): Promise<void> {
    this.isOpen = false;
  }

  async startTransaction(
    isolation: IsolationLevelName = IsolationLevel.Serializable,
  ): Promise<ITransaction> {
    this.ensureOpen();
    this.snapshot = this.store.snapshot();
    this.inTransaction = true;
    return new MemoryTransaction(this, isolation);
  }

  getCollection(): MemoryRecord[] {
    return this.store.getCollection(this.collection);
  }

  finishCommit(): void {
    this.inTransaction = false;
    this.snapshot = undefined;
  }

  finishAbort(): void {
    if (this.snapshot) {
      this.store.restore(this.snapshot);
    }
    this.inTransaction = false;
    this.snapshot = undefined;
  }

  ensureOpen(): void {
    if (!this.isOpen) throw new DatasourceError("Connection is not open");
  }
}
