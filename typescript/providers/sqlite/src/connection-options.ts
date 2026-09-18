import type { ISqliteDriver } from "./driver.ts";

export interface SqliteConnectionOptions {
  filename?: string;
  url?: string;
  driver?: ISqliteDriver;
}
