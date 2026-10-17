import {
  DatasourceError,
  IsolationLevel,
  StatementTextRenderer,
  type DatasourceOptions,
  type ICommand,
  type IConnection,
  type IsolationLevelName,
  type ITransaction,
} from "@datasources/core";
import { createRequire } from "node:module";
import { SqliteCommand } from "./command.ts";
import type { SqliteConnectionOptions } from "./connection-options.ts";
import type { SqliteDatabase, SqliteDatabaseCtor } from "./i-sqlite-driver.ts";
import { filenameOf } from "./sqlitePath.ts";
import { SqliteStatementRenderer } from "./SqliteStatementRenderer.ts";
import { SqliteTransaction } from "./transaction.ts";

const require = createRequire(import.meta.url);

const pragma = (isolation: IsolationLevelName): string =>
  isolation === IsolationLevel.ReadUncommitted
    ? "PRAGMA read_uncommitted = 1"
    : "PRAGMA read_uncommitted = 0";

export class SqliteConnection implements IConnection {
  readonly provider = "sqlite";
  filename: string;
  isOpen = false;
  private db?: SqliteDatabase;
  private readonly Database?: SqliteDatabaseCtor;

  static from = (options?: DatasourceOptions): SqliteConnection =>
    new SqliteConnection({
      url: options?.DATASOURCE_URL,
      Database: require("better-sqlite3") as SqliteDatabaseCtor,
    });

  constructor(options: SqliteConnectionOptions = {}) {
    this.filename = filenameOf(options);
    this.Database = options.Database;
  }

  createCommand = (): ICommand =>
    new SqliteCommand(this, new StatementTextRenderer(t => new SqliteStatementRenderer(t)));

  async open(): Promise<void> {
    if (this.isOpen) throw new DatasourceError("Connection is already open");
    if (!this.Database) throw new DatasourceError("SQLite driver is required");
    this.db = new this.Database(this.filename, { timeout: 0 });
    this.isOpen = true;
  }

  async close(): Promise<void> {
    this.db?.close();
    this.db = undefined;
    this.isOpen = false;
  }

  [Symbol.asyncDispose] = (): Promise<void> => this.close();

  async startTransaction(
    isolation: IsolationLevelName = IsolationLevel.Serializable,
  ): Promise<ITransaction> {
    this.ensureOpen();
    this.exec(pragma(isolation));
    this.exec("BEGIN");
    return new SqliteTransaction(this, isolation);
  }

  ensureOpen(): void {
    if (!this.isOpen) throw new DatasourceError("Connection is not open");
  }

  require(): SqliteDatabase {
    if (!this.db) throw new DatasourceError("Connection is not open");
    return this.db;
  }

  exec(sql: string): void {
    this.require().exec(sql);
  }
}
