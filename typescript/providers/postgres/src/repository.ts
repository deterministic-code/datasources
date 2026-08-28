import {
  ProviderNotImplementedError,
  type IQuery,
  type IRepository,
  type IResponse,
  type IWhere,
} from "@datasources/core";
import type { PostgresConnection } from "./connection.ts";

export interface PostgresRepositoryOptions {
  table?: string;
}

export class PostgresRepository<T> implements IRepository<T> {
  readonly table: string;

  constructor(
    readonly connection: PostgresConnection,
    options: PostgresRepositoryOptions = {},
  ) {
    this.table = options.table ?? "items";
  }

  getSingle = async (_query?: IQuery): Promise<T | undefined> => {
    throw new ProviderNotImplementedError("postgres");
  };

  async *get(_query?: IQuery): AsyncIterable<T> {
    throw new ProviderNotImplementedError("postgres");
  }

  add = async (_obj: T): Promise<IResponse> => {
    throw new ProviderNotImplementedError("postgres");
  };

  update = async (_obj: T, _byWhere?: IWhere): Promise<IResponse> => {
    throw new ProviderNotImplementedError("postgres");
  };

  delete = async (_obj: T, _byWhere?: IWhere): Promise<IResponse> => {
    throw new ProviderNotImplementedError("postgres");
  };
}
