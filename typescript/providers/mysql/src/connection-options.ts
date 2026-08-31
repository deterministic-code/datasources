import type { IMysqlDriver } from "./driver.ts";

export interface MysqlConnectionOptions {
  url?: string;
  driver?: IMysqlDriver;
}
