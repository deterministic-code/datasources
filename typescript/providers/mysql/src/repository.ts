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
    readonly connection: MysqlConnection,
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

  add = async (_obj: T): Promise<IResponse> => {
    throw new ProviderNotImplementedError("mysql");
  };

  update = async (_obj: T, _byWhere?: IWhere): Promise<IResponse> => {
    throw new ProviderNotImplementedError("mysql");
  };

  delete = async (_obj: T, _byWhere?: IWhere): Promise<IResponse> => {
    throw new ProviderNotImplementedError("mysql");
  };
}
