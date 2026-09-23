import type { SqliteDatabaseCtor } from "./i-sqlite-driver.ts";

export interface SqliteConnectionOptions {
  filename?: string;
  url?: string;
  Database?: SqliteDatabaseCtor;
}
