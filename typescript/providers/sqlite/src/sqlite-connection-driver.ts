import type { SqliteDatabase, SqliteDatabaseCtor } from "./i-sqlite-driver.ts";

export class SqliteConnectionDriver {
  private db: SqliteDatabase | undefined;

  constructor(
    private readonly filename: string,
    private readonly Database: SqliteDatabaseCtor,
  ) {}

  async open(_filename: string): Promise<void> {
    if (this.db) return;
    this.db = new this.Database(this.filename, { timeout: 0 });
  }

  async close(): Promise<void> {
    this.db?.close();
    this.db = undefined;
  }

  require(): SqliteDatabase {
    if (!this.db) throw new Error("SQLite driver is not open");
    return this.db;
  }
}
