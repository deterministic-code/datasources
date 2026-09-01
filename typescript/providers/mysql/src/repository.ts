import {
  ProviderNotImplementedError,
  type IQuery,
  type IRepository,
  type IResponse,
  type IWhere,
} from "@datasources/core";
import type { MysqlConnection } from "./connection.ts";

export interface MysqlRepositoryOptions {
  table?: string;
}

export class MysqlRepository<T> implements IRepository<T> {
  readonly table: string;

  constructor(
    _connection: MysqlConnection,
    options: MysqlRepositoryOptions = {},
  ) {
    this.table = options.table ?? "items";
  }

  getSingle = async (_query?: IQuery | IWhere): Promise<T | undefined> => {
    throw new ProviderNotImplementedError("mysql");
  };

  async *get(_query?: IQuery | IWhere): AsyncIterable<T> {
    throw new ProviderNotImplementedError("mysql");
  }

  add = async (_item: T): Promise<IResponse> => {
    throw new ProviderNotImplementedError("mysql");
  };

  update = async (_item: T, _byWhere?: IWhere): Promise<IResponse> => {
    throw new ProviderNotImplementedError("mysql");
  };

  delete = async (_item: T, _byWhere?: IWhere): Promise<IResponse> => {
    throw new ProviderNotImplementedError("mysql");
  };
}
