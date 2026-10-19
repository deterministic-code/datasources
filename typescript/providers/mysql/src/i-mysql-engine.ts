import type { Parameter } from "@deterministic-code/datasources";
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
  queryRows(sql: string, params?: Record<string, Scalar>): AsyncIterable<Cells>;
  beginTransaction(): Promise<void>;
  commit(): Promise<void>;
  rollback(): Promise<void>;
  end(): Promise<void>;
}

interface MysqlCoreQuery {
  stream(): AsyncIterable<Cells>;
}

interface Mysql2PromiseConnection extends IMysqlClient {
  connection: {
    query(sql: string, params?: Record<string, Scalar>): MysqlCoreQuery;
  };
}

export type MysqlCreateConnection = (url: string) => Promise<IMysqlClient>;

export type Mysql2Module = {
  createConnection: (
    options: { uri: string; namedPlaceholders: boolean },
  ) => Promise<IMysqlClient>;
};

export const mysql2Connect = (mysql: Mysql2Module): MysqlCreateConnection => async (uri: string) => {
  const client = (await mysql.createConnection({
    uri,
    namedPlaceholders: true,
  })) as Mysql2PromiseConnection;
  return {
    query: (sql, params) => client.query(sql, params),
    queryRows: (sql, params) => client.connection.query(sql, params).stream(),
    beginTransaction: () => client.beginTransaction(),
    commit: () => client.commit(),
    rollback: () => client.rollback(),
    end: () => client.end(),
  };
};
