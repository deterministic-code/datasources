import type { IQuery } from "./statements/IQuery.ts";
import type { IRepository } from "./IRepository.ts";
import type { IWhere } from "./statements/IWhere.ts";

export interface IReadonlyRepository<T = unknown> extends IRepository<T> {
  readSingle(query?: IQuery | IWhere): Promise<T | undefined>;
  read(query?: IQuery | IWhere): AsyncIterable<T>;
}
