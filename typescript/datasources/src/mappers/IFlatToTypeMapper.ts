import type { Cells } from "../converters/Cell.ts";
import type { IMapper } from "./IMapper.ts";
import type { TypeMap } from "./TypeMap.ts";

export interface IFlatToTypeMapper<T> extends IMapper<Cells, T> {
  readonly spec: TypeMap<T>;
  collapse(rows: Iterable<T | Cells>): T[];
}
