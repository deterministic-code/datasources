import type { IReadonlyRepository } from "./IReadonlyRepository.ts";
import type { IWhere } from "./IWhere.ts";
import type { IResponse } from "./IResponse.ts";

export interface IRepository<T = unknown> extends IReadonlyRepository<T> {
  add(item: T): Promise<IResponse>;
  update(item: T, byWhere?: IWhere): Promise<IResponse>;
  delete(item: T, byWhere?: IWhere): Promise<IResponse>;
}
