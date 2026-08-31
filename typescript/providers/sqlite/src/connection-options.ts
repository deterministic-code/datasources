import type { ISqliteDriver } from "./driver.ts";

export interface SqliteConnectionOptions {
  filename?: string;
  driver?: ISqliteDriver;
}
