import type { Scalar } from "./builders/Scalar.ts";
import type { Cells } from "./converters/Cell.ts";
import type { IReadonlyRepository } from "./IReadonlyRepository.ts";
import type { IWhere } from "./statements/IWhere.ts";
import type { Response } from "./Response.ts";

export interface ICRUDRepository<T = unknown> extends IReadonlyRepository<T> {
  create(item: T): Promise<Response>;
  update(item: T, byWhere?: IWhere): Promise<Response>;
  delete(item: T, byWhere?: IWhere): Promise<Response>;
  readSingleById(id: Scalar | Cells): Promise<T | undefined>;
}
