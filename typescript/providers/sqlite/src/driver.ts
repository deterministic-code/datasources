import {
  DatasourceError,
  parametersToRecord,
  type Parameter,
} from "@datasources/core";
import { BetterSqliteDriver } from "./better-sqlite-driver.ts";
import type {
  BetterSqlite3Module,
  ISqliteDriver,
} from "./i-sqlite-driver.ts";

export type {
  BetterSqlite3Module,
  ISqliteDriver,
  SqliteDatabase,
  SqliteDatabaseCtor,
  SqliteExecuteResult,
  SqliteStatement,
} from "./i-sqlite-driver.ts";
export { BetterSqliteDriver } from "./better-sqlite-driver.ts";

export async function loadBetterSqlite3Driver(
  filename: string,
  importer: () => Promise<BetterSqlite3Module> = () => import("better-sqlite3"),
): Promise<ISqliteDriver> {
  const imported = await importer();
  return new BetterSqliteDriver(filename, imported.default ?? imported);
}

export const bindParameters = (
  parameters?: Parameter[] | null,
): Record<string, unknown> => parametersToRecord(parameters);

export function requireDriver(driver?: ISqliteDriver): ISqliteDriver {
  if (!driver) throw new DatasourceError("SQLite driver is required");
  return driver;
}
