import type { Parameter } from "@datasources/core";
import type { Cells, Scalar } from "./scalar.ts";

export interface MysqlExecuteResult {
  rowsAffected: number;
  result?: Scalar;
  parameters?: Parameter[];
}

export interface MysqlQueryHeader {
  affectedRows?: number;
  insertId?: Scalar;
}

export interface IMysqlClient {
  query(
    sql: string,
    params?: Record<string, Scalar>,
  ): Promise<[MysqlQueryHeader | Cells[]]>;
  beginTransaction(): Promise<void>;
  commit(): Promise<void>;
  rollback(): Promise<void>;
  end(): Promise<void>;
}

export type MysqlCreateConnection = (url: string) => Promise<IMysqlClient>;

export type Mysql2Module = {
  createConnection: (
    options: { uri: string; namedPlaceholders: boolean },
  ) => Promise<IMysqlClient>;
};

export const mysql2Connect = (mysql: Mysql2Module): MysqlCreateConnection => (uri: string) =>
  mysql.createConnection({ uri, namedPlaceholders: true });
