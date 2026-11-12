import {
  DatasourceError,
  IsolationLevel,
  type DatasourceOptions,
  type ICommand,
  type IConnection,
  type IsolationLevelName,
  type ITransaction,
} from "@deterministic-code/datasources";
import { MemoryCommand } from "./command.ts";
import type { MemoryConnectionOptions } from "./connection-options.ts";
import { MemoryEngine, type MemoryRecord } from "./engine.ts";
import { MemoryTransaction } from "./transaction.ts";

export class MemoryConnection implements IConnection {
  readonly provider = "memory";
  engine: MemoryEngine;
  isOpen = false;
  private readonly data?: MemoryRecord[] | Record<string, MemoryRecord[]>;

  static from = (_options?: DatasourceOptions): MemoryConnection =>
    new MemoryConnection();

  constructor(options: MemoryConnectionOptions = {}) {
    this.engine = options.engine ?? new MemoryEngine();
    this.engine.collection = options.collection ?? this.engine.collection;
    this.data = options.data;
  }

  get collection(): string {
    return this.engine.collection;
  }

  set collection(value: string) {
    this.engine.collection = value;
  }

  get inTransaction(): boolean {
    return this.engine.inTransaction;
  }

  createCommand = (): ICommand => new MemoryCommand(this);

  async open(): Promise<void> {
    if (this.isOpen) throw new DatasourceError("Connection is already open");
    if (this.data !== undefined) this.engine.seed(this.data, this.collection);
    this.engine.getCollection(this.collection);
    this.isOpen = true;
  }

  async close(): Promise<void> {
    this.isOpen = false;
  }

  [Symbol.asyncDispose] = (): Promise<void> => this.close();

  async startTransaction(
    isolation: IsolationLevelName = IsolationLevel.ReadCommitted,
  ): Promise<ITransaction> {
    this.ensureOpen();
    this.engine.begin();
    return new MemoryTransaction(this, isolation);
  }

  getCollection(): MemoryRecord[] {
    return this.engine.getCollection(this.collection);
  }

  rows(table: string | undefined, create: boolean): MemoryRecord[] {
    return this.engine.rows(table, create);
  }

  finishCommit(): void {
    this.engine.commit();
  }

  finishAbort(): void {
    this.engine.abort();
  }

  ensureOpen(): void {
    if (!this.isOpen) throw new DatasourceError("Connection is not open");
  }
}
