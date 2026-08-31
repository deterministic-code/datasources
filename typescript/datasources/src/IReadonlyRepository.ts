import type { IQuery } from "./IQuery.ts";
import type { IWhere } from "./IWhere.ts";

export interface IReadonlyRepository<T = unknown> {
  getSingle(query?: IQuery | IWhere): Promise<T | undefined>;
  get(query?: IQuery | IWhere): AsyncIterable<T>;
}
