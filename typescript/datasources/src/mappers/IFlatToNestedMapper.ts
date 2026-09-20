import type { Cells } from "../converters/Cell.ts";
import type { IMapper } from "./IMapper.ts";
import type { TypeMap } from "./TypeMap.ts";

export interface IFlatToNestedMapper<T> extends IMapper<Iterable<T | Cells>, T[]> {
  readonly spec: TypeMap<T>;
}
