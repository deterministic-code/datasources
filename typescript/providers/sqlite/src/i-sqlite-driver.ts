import type { Value, Cells, Scalar } from "@deterministic-code/datasources";

export interface SqliteStatement {
  run(params?: Record<string, Scalar>): {
    changes: number;
    lastInsertRowid?: Value;
  };
  get?(params?: Record<string, Scalar>): Cells | undefined;
  all(params?: Record<string, Scalar>): Cells[];
  iterate(params?: Record<string, Scalar>): IterableIterator<Cells>;
}

export interface SqliteDatabase {
  prepare(sql: string): SqliteStatement;
  exec(sql: string): void;
  close(): void;
}

export type SqliteDatabaseCtor = new (
  filename: string,
  options?: { timeout?: number },
) => SqliteDatabase;
