import type { PgClientCtor } from "./i-postgres-driver.ts";

export interface PostgresConnectionOptions {
  url?: string;
  schema?: string;
  Client?: PgClientCtor;
}
