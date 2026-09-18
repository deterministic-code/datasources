import type { IReadonlyRepository } from "./IReadonlyRepository.ts";
import type { IWhere } from "./IWhere.ts";
import type { Response } from "./Response.ts";

export interface ICRUDRepository<T = unknown> extends IReadonlyRepository<T> {
  create(item: T): Promise<Response>;
  update(item: T, byWhere?: IWhere): Promise<Response>;
  delete(item: T, byWhere?: IWhere): Promise<Response>;
}
