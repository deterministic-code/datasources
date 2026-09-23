import type { MysqlCreateConnection } from "./i-mysql-engine.ts";

export interface MysqlConnectionOptions {
  url?: string;
  connect?: MysqlCreateConnection;
}
