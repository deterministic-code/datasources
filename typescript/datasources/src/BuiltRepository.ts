import type { IConnection } from "./IConnection.ts";
import type { Cells } from "./converters/Cell.ts";
import type { Constructable } from "./converters/Constructable.ts";
import type { TypeConverter } from "./converters/TypeConverter.ts";
import type { IMapper } from "./mappers/IMapper.ts";
import type { Scalar } from "./builders/Scalar.ts";
import type { RepositoryStatements } from "./RepositoryStatements.ts";

export type BuiltRepository<T = Cells> = {
  connection: IConnection;
  rowConverter: TypeConverter<T, Cells>;
  rowsMapper: IMapper<T, T>;
  result?: IMapper<Iterable<Cells>, T[]>;
  columnMappings: IMapper<string, string>;
  statements: RepositoryStatements;
  readColumns: string[];
  ids: string[];
  type: Constructable<T>;
  create?: IMapper<Scalar, T>;
};
