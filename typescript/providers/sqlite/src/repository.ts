import {
  ProviderNotImplementedError,
  type IQuery,
  type IRepository,
  type IResponse,
  type IWhere,
} from "@datasources/core";
import type { SqliteConnection } from "./connection.ts";

export interface SqliteRepositoryOptions {
  table?: string;
}

export class SqliteRepository<T = unknown> implements IRepository<T> {
  readonly table: string;

  constructor(
    readonly connection: SqliteConnection,
    options: SqliteRepositoryOptions = {},
  ) {
    this.table = options.table ?? "items";
  }

  getSingle = async (_query?: IQuery): Promise<T | undefined> => {
    throw new ProviderNotImplementedError("sqlite");
  };

  async *get(_query?: IQuery): AsyncIterable<T> {
    throw new ProviderNotImplementedError("sqlite");
  }

  add = async (_obj: T): Promise<IResponse> => {
    throw new ProviderNotImplementedError("sqlite");
  };

  update = async (_obj: T, _byWhere?: IWhere): Promise<IResponse> => {
    throw new ProviderNotImplementedError("sqlite");
  };

  delete = async (_obj: T, _byWhere?: IWhere): Promise<IResponse> => {
    throw new ProviderNotImplementedError("sqlite");
  };
}
