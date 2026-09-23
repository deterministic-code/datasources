import type { IConnection } from "./IConnection.ts";
import type { Cells } from "./converters/Cell.ts";
import type { Constructable } from "./converters/Constructable.ts";
import type { TypeConverter } from "./converters/TypeConverter.ts";
import type { CRUDMapperOptions } from "./CRUDMapperOptions.ts";
import type { RepositoryStatements } from "./RepositoryStatements.ts";

export type BuiltRepository<T = Cells> = {
  connection: IConnection;
  rowConverter: TypeConverter<T, Cells>;
  crudMapper: CRUDMapperOptions<T>;
  statements: RepositoryStatements;
  readColumns: string[];
  ids: string[];
  type: Constructable<T>;
};
