import type { Cell, Cells } from "./converters/Cell.ts";
import type { Constructable } from "./converters/Constructable.ts";
import type { TypeConverter } from "./converters/TypeConverter.ts";
import type { CRUDMapperOptions } from "./CRUDMapperOptions.ts";

export type RepositoryOptions<T = Cells> = {
  ids?: string[];
  entity?: string;
  rowConverter?: TypeConverter<T, Cells>;
  crudMapper?: CRUDMapperOptions<T>;
  type?: Constructable<T>;
  rowTypeConverters?: Record<string, TypeConverter<Cell, Cell>>;
  readColumns?: string[];
};
