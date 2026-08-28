import type { IsolationLevelName } from "@datasources/core";
import type { Cells, Scalar } from "./scalar.ts";

export interface PostgresExecuteResult {
  rowsAffected: number;
  result?: Scalar;
}

export interface IPostgresDriver {
  open(url: string): Promise<void>;
  close(): Promise<void>;
  execute(
    sql: string,
    params?: Record<string, Scalar>,
  ): Promise<PostgresExecuteResult>;
  query(sql: string, params?: Record<string, Scalar>): Promise<Cells[]>;
  begin(isolation?: IsolationLevelName): Promise<void>;
  commit(): Promise<void>;
  rollback(): Promise<void>;
}

export interface PgQueryResult {
  rowCount: number | null;
  rows: Cells[];
}

export interface IPgClient {
  connect(): Promise<void>;
  query(text: string, values?: Scalar[]): Promise<PgQueryResult>;
  end(): Promise<void>;
}

export type PgClientCtor = new (options: {
  connectionString: string;
}) => IPgClient;

export type PgModule = {
  Client: PgClientCtor;
  default?: { Client: PgClientCtor };
};
