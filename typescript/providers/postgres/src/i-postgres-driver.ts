import type { Parameter } from "@datasources/core";
import type { Cells, Scalar } from "./scalar.ts";

export interface PostgresExecuteResult {
  rowsAffected: number;
  result?: Scalar;
  parameters?: Parameter[];
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

export const pgClient = (imported: PgModule): PgClientCtor =>
  imported.default?.Client ?? imported.Client;
