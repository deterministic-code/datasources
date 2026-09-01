import type { IConnection } from "./IConnection.ts";
import type { Cell, Cells } from "./converters/Cell.ts";
import type { Constructable } from "./converters/Constructable.ts";
import type { TypeConverter } from "./converters/TypeConverter.ts";
import type { IMapper } from "./mappers/IMapper.ts";

export type RepositoryBuilderOptions<T = Cells> = {
  connection: IConnection;
  ids?: string[];
  assignIdentityOnAdd?: boolean;
  entity?: string;
  mapping?: TypeConverter<T, Cells>;
  mapper?: IMapper<T, T>;
  type?: Constructable<T>;
  converters?: Record<string, TypeConverter<Cell, Cell>>;
  fields?: Record<string, TypeConverter<Cell, Cell>>;
  columns?: string[];
};
