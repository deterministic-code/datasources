import type { Scalar } from "./builders/Scalar.ts";
import type { Cells } from "./converters/Value.ts";
import type { IReadonlyRepository } from "./IReadonlyRepository.ts";
import type { IWhere } from "./statements/IWhere.ts";
import type { Response } from "./Response.ts";

export interface ICRUDRepository<T> extends IReadonlyRepository<T> {
  readonly ids: readonly string[];
  create(item: T): Promise<T>;
  update(item: T, byWhere?: IWhere): Promise<Response>;
  delete(item: T, byWhere?: IWhere): Promise<Response>;
  readSingleById(id: Scalar | Cells): Promise<T | undefined>;
}
