import type { Parameter } from "@datasources/core";
import QueryStream from "pg-query-stream";
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
  rows(text: string, values?: Scalar[]): AsyncIterable<Cells>;
  end(): Promise<void>;
}

interface PgNativeClient {
  connect(): Promise<void>;
  query(text: string, values?: Scalar[]): Promise<PgQueryResult>;
  query(query: QueryStream): AsyncIterable<Cells>;
  end(): Promise<void>;
}

export type PgClientCtor = new (options: {
  connectionString: string;
}) => IPgClient;

type PgNativeCtor = new (options: { connectionString: string }) => PgNativeClient;

export type PgModule = {
  Client: PgNativeCtor;
  default?: { Client: PgNativeCtor };
};

export const pgClient = (imported: PgModule): PgClientCtor => {
  const Client = imported.default?.Client ?? imported.Client;
  return class implements IPgClient {
    private readonly inner: PgNativeClient;
    constructor(options: { connectionString: string }) {
      this.inner = new Client(options);
    }
    connect(): Promise<void> {
      return this.inner.connect();
    }
    query(text: string, values?: Scalar[]): Promise<PgQueryResult> {
      return this.inner.query(text, values);
    }
    rows(text: string, values?: Scalar[]): AsyncIterable<Cells> {
      return this.inner.query(new QueryStream(text, values ?? []));
    }
    end(): Promise<void> {
      return this.inner.end();
    }
  };
};
