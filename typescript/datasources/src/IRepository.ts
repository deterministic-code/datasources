import type { IReadonlyRepository } from "./IReadonlyRepository.ts";
import type { IWhere } from "./IWhere.ts";
import type { IResponse } from "./IResponse.ts";

export interface IRepository<T = unknown> extends IReadonlyRepository<T> {
  add(obj: T): Promise<IResponse>;
  update(obj: T, byWhere?: IWhere): Promise<IResponse>;
  delete(obj: T, byWhere?: IWhere): Promise<IResponse>;
}
