import type { IPostgresDriver } from "./driver.ts";

export interface PostgresConnectionOptions {
  url?: string;
  schema?: string;
  driver?: IPostgresDriver;
}
