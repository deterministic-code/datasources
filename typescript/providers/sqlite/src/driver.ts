import {
  DatasourceError,
  parametersToRecord,
  type Parameter,
} from "@datasources/core";

export interface SqliteExecuteResult {
  rowsAffected: number;
  result?: unknown;
}

export interface ISqliteDriver {
  open(filename: string): Promise<void>;
  close(): Promise<void>;
  execute(
    sql: string,
    params?: Record<string, unknown>,
  ): Promise<SqliteExecuteResult>;
  query(
    sql: string,
    params?: Record<string, unknown>,
  ): Promise<Record<string, unknown>[]>;
  begin(): Promise<void>;
  commit(): Promise<void>;
  rollback(): Promise<void>;
}

export interface SqliteStatement {
  run(params?: Record<string, unknown>): { changes: number; lastInsertRowid?: unknown };
  all(params?: Record<string, unknown>): Record<string, unknown>[];
}

export interface SqliteDatabase {
  prepare(sql: string): SqliteStatement;
  exec(sql: string): void;
  close(): void;
}

export type SqliteDatabaseCtor = new (filename: string) => SqliteDatabase;

export type BetterSqlite3Module = {
  default?: SqliteDatabaseCtor;
} & SqliteDatabaseCtor;

export class BetterSqliteDriver implements ISqliteDriver {
  private db: SqliteDatabase | undefined;

  constructor(
    private readonly filename: string,
    private readonly Database: SqliteDatabaseCtor,
  ) {}

  async open(_filename: string): Promise<void> {
    if (this.db) {
      return;
    }
    this.db = new this.Database(this.filename);
  }

  async close(): Promise<void> {
    this.db?.close();
    this.db = undefined;
  }

  async execute(
    sql: string,
    params?: Record<string, unknown>,
  ): Promise<SqliteExecuteResult> {
    const db = this.requireDb();
    if (!sql.trim()) {
      return { rowsAffected: 0 };
    }
    const statement = db.prepare(sql);
    const info = statement.run(params);
    return { rowsAffected: info.changes, result: info.lastInsertRowid };
  }

  async query(
    sql: string,
    params?: Record<string, unknown>,
  ): Promise<Record<string, unknown>[]> {
    if (!sql.trim()) {
      return [];
    }
    return this.requireDb().prepare(sql).all(params);
  }

  async begin(): Promise<void> {
    this.requireDb().exec("BEGIN");
  }

  async commit(): Promise<void> {
    this.requireDb().exec("COMMIT");
  }

  async rollback(): Promise<void> {
    this.requireDb().exec("ROLLBACK");
  }

  private requireDb(): SqliteDatabase {
    if (!this.db) {
      throw new Error("SQLite driver is not open");
    }
    return this.db;
  }
}

export async function loadBetterSqlite3Driver(
  filename: string,
  importer: () => Promise<BetterSqlite3Module> = () => import("better-sqlite3"),
): Promise<ISqliteDriver> {
  const imported = await importer();
  const Database = imported.default ?? imported;
  return new BetterSqliteDriver(filename, Database);
}

export function bindParameters(
  parameters?: Parameter[] | null,
): Record<string, unknown> {
  return parametersToRecord(parameters);
}

export function requireDriver(driver?: ISqliteDriver): ISqliteDriver {
  if (!driver) {
    throw new DatasourceError("SQLite driver is required");
  }
  return driver;
}
