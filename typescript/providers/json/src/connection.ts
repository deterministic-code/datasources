import {
  DatasourceError,
  DatasourceUrlParser,
  IsolationLevel,
  type DatasourceOptions,
  type ICommand,
  type IConnection,
  type IsolationLevelName,
  type ITransaction,
} from "@datasources/core";
import { JsonCommand } from "./command.ts";
import type { JsonConnectionOptions } from "./connection-options.ts";
import { MemoryFileStore, type IFileStore } from "./file-store.ts";
import { JsonTransaction } from "./transaction.ts";

export class JsonConnection implements IConnection {
  readonly provider = "json";
  path: string;
  collection?: string;
  fileStore: IFileStore;
  data: unknown;
  isOpen = false;
  inTransaction = false;
  private snapshot: unknown;
  private readonly initialData?: unknown;
  private readonly hasInitialData: boolean;

  static from = (options?: DatasourceOptions): JsonConnection =>
    new JsonConnection({
      path: DatasourceUrlParser.parse(options?.DATASOURCE_URL)?.datasource ?? "",
    });



  constructor(options: JsonConnectionOptions = {}) {
    this.path = options.path ?? "";
    this.collection = options.collection;
    this.fileStore = options.fileStore ?? new MemoryFileStore();
    this.hasInitialData = Object.prototype.hasOwnProperty.call(options, "data");
    this.initialData = options.data;
    this.data = this.emptyData();
  }

  createCommand = (): ICommand => new JsonCommand(this);

  async open(): Promise<void> {
    if (this.isOpen) throw new DatasourceError("Connection is already open");
    if (this.hasInitialData) {
      this.data = structuredClone(this.initialData);
    } else if (this.path && (await this.fileStore.exists(this.path))) {
      const contents = await this.fileStore.read(this.path);
      this.data = JSON.parse(contents);
    } else {
      this.data = this.emptyData();
    }
    if (this.collection) {
      this.getCollection();
    }
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
    this.snapshot = structuredClone(this.data);
    this.inTransaction = true;
    return new JsonTransaction(this, isolation);
  }

  getCollection(): unknown[] {
    if (this.collection) {
      if (this.data === null || typeof this.data !== "object" || Array.isArray(this.data)) {
        this.data = {};
      }
      const root = this.data as Record<string, unknown>;
      if (!Array.isArray(root[this.collection])) {
        root[this.collection] = [];
      }
      return root[this.collection] as unknown[];
    }
    if (!Array.isArray(this.data)) {
      this.data = [];
    }
    return this.data as unknown[];
  }

  async persist(): Promise<void> {
    if (this.inTransaction || !this.path) {
      return;
    }
    await this.fileStore.write(this.path, JSON.stringify(this.data));
  }

  finishCommit(): void {
    this.inTransaction = false;
    this.snapshot = undefined;
  }

  finishAbort(): void {
    this.data = this.snapshot;
    this.inTransaction = false;
    this.snapshot = undefined;
  }

  ensureOpen(): void {
    if (!this.isOpen) throw new DatasourceError("Connection is not open");
  }

  private emptyData = (): unknown => (this.collection ? {} : []);
}
