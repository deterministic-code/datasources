import { DatasourceError } from "@datasources/core";
import { PgDriver } from "./pg-driver.ts";
import type { IPostgresDriver, PgModule } from "./i-postgres-driver.ts";

export type {
  IPgClient,
  IPostgresDriver,
  PgClientCtor,
  PgModule,
  PgQueryResult,
  PostgresExecuteResult,
} from "./i-postgres-driver.ts";
export { bindParameters, toPostgres, withReturning } from "./bind.ts";
export { PgDriver } from "./pg-driver.ts";

export async function loadPgDriver(
  url: string,
  importer: () => Promise<PgModule> = () => import("pg"),
  schema?: string,
): Promise<IPostgresDriver> {
  const imported = await importer();
  const Client = imported.default?.Client ?? imported.Client;
  return new PgDriver(url, Client, schema);
}

export function requireDriver(driver?: IPostgresDriver): IPostgresDriver {
  if (!driver) throw new DatasourceError("Postgres driver is required");
  return driver;
}
