import {
  DatasourceError,
  IsolationLevel,
  MiddlewareHost,
  type IConnection,
  type IsolationLevelName,
  type ITransaction,
} from "@datasources/core";
import { MemoryStore, type MemoryRecord } from "./store.ts";
import { MemoryTransaction } from "./transaction.ts";

export interface MemoryConnectionOptions {
  collection?: string;
  store?: MemoryStore;
  data?: MemoryRecord[] | Record<string, MemoryRecord[]>;
}

export class MemoryConnection extends MiddlewareHost implements IConnection {
  store: MemoryStore;
  collection: string;
  opened = false;
  inTransaction = false;
  private snapshot?: Map<string, MemoryRecord[]>;
  private readonly data?: MemoryRecord[] | Record<string, MemoryRecord[]>;

  constructor(options: MemoryConnectionOptions = {}) {
    super();
    this.store = options.store ?? new MemoryStore();
    this.collection = options.collection ?? "";
    this.data = options.data;
  }

  async open(): Promise<void> {
    return this.run("open", async () => {
      if (this.opened) {
        return;
      }
      if (this.data !== undefined) {
        this.store.seed(this.data, this.collection);
      }
      this.store.getCollection(this.collection);
      this.opened = true;
    });
  }

  async close(): Promise<void> {
    return this.run("close", async () => {
      this.opened = false;
    });
  }

  async startTransaction(
    isolation: IsolationLevelName = IsolationLevel.Serializable,
  ): Promise<ITransaction> {
    return this.run("startTransaction", async () => {
      this.ensureOpen();
      this.snapshot = this.store.snapshot();
      this.inTransaction = true;
      return new MemoryTransaction(this, isolation);
    });
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
    if (!this.opened) throw new DatasourceError("Connection is not open");
  }
}
