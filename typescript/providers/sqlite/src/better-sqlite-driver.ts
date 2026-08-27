import type {
  ISqliteDriver,
  SqliteDatabase,
  SqliteDatabaseCtor,
  SqliteExecuteResult,
} from "./i-sqlite-driver.ts";

export class BetterSqliteDriver implements ISqliteDriver {
  private db: SqliteDatabase | undefined;

  constructor(
    private readonly filename: string,
    private readonly Database: SqliteDatabaseCtor,
  ) {}

  async open(_filename: string): Promise<void> {
    if (this.db) return;
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
    if (!sql.trim()) return { rowsAffected: 0 };
    const info = this.requireDb().prepare(sql).run(params);
    return { rowsAffected: info.changes, result: info.lastInsertRowid };
  }

  async query(
    sql: string,
    params?: Record<string, unknown>,
  ): Promise<Record<string, unknown>[]> {
    if (!sql.trim()) return [];
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
    if (!this.db) throw new Error("SQLite driver is not open");
    return this.db;
  }
}
