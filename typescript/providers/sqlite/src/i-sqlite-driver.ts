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
  run(params?: Record<string, unknown>): {
    changes: number;
    lastInsertRowid?: unknown;
  };
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
