import { DatasourceError } from "@datasources/core";
import { Mysql2Driver } from "./mysql2-driver.ts";
import type { IMysqlDriver, Mysql2Module } from "./i-mysql-driver.ts";

export type {
  IMysqlClient,
  IMysqlDriver,
  Mysql2Module,
  MysqlCreateConnection,
  MysqlExecuteResult,
  MysqlQueryHeader,
} from "./i-mysql-driver.ts";
export { bindParameters, toMysql, withoutReturning } from "./bind.ts";
export { Mysql2Driver } from "./mysql2-driver.ts";

export const mysql2Connect = (mysql: Mysql2Module) => (uri: string) =>
  mysql.createConnection({ uri, namedPlaceholders: true });

export async function loadMysql2Driver(
  url: string,
  importer: () => Promise<Mysql2Module> = () => import("mysql2/promise"),
): Promise<IMysqlDriver> {
  return new Mysql2Driver(url, mysql2Connect(await importer()));
}

export function requireDriver(driver?: IMysqlDriver): IMysqlDriver {
  if (!driver) throw new DatasourceError("MySQL driver is required");
  return driver;
}
