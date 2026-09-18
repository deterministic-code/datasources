import type { IQuery } from "./IQuery.ts";
import type { IRepository } from "./IRepository.ts";
import type { IWhere } from "./IWhere.ts";

export interface IReadonlyRepository<T = unknown> extends IRepository<T> {
  getSingle(query?: IQuery | IWhere): Promise<T | undefined>;
  get(query?: IQuery | IWhere): AsyncIterable<T>;
}
