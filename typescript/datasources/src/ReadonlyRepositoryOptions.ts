import type { Cell, Cells } from "./converters/Cell.ts";
import type { Constructable } from "./converters/Constructable.ts";
import type { TypeConverter } from "./converters/TypeConverter.ts";
import type { IMapper } from "./mappers/IMapper.ts";
import type { IQuery } from "./statements/IQuery.ts";

export type ReadonlyRepositoryOptions<T = Cells> = {
  entity?: string;
  entityMapper?: IMapper<string, string>;
  columns?: IMapper<string, string>;
  query?: IMapper<IQuery, IQuery>;
  rowsMapper?: IMapper<T, T>;
  rowConverter?: TypeConverter<T, Cells>;
  type?: Constructable<T>;
  rowTypeConverters?: Record<string, TypeConverter<Cell, Cell>>;
  readColumns?: string[];
};
