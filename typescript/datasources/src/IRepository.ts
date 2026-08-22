import type { IConnection } from "./IConnection.ts";
import type { IQuery } from "./IQuery.ts";
import type { IWhere } from "./IWhere.ts";
import type { IResponse } from "./IResponse.ts";

export interface IRepository<T = unknown> {
  readonly connection: IConnection;
  getSingle(query?: IQuery): Promise<T | undefined>;
  get(query?: IQuery): AsyncIterable<T>;
  add(obj: T): Promise<IResponse>;
  update(obj: T, byWhere?: IWhere): Promise<IResponse>;
  delete(obj: T, byWhere?: IWhere): Promise<IResponse>;
}
