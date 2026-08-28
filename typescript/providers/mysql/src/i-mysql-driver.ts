import type { IsolationLevelName } from "@datasources/core";
import type { Cells, Scalar } from "./scalar.ts";

export interface MysqlExecuteResult {
  rowsAffected: number;
  result?: Scalar;
}

export interface IMysqlDriver {
  open(url: string): Promise<void>;
  close(): Promise<void>;
  execute(
    sql: string,
    params?: Record<string, Scalar>,
  ): Promise<MysqlExecuteResult>;
  query(sql: string, params?: Record<string, Scalar>): Promise<Cells[]>;
  begin(isolation?: IsolationLevelName): Promise<void>;
  commit(): Promise<void>;
  rollback(): Promise<void>;
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
