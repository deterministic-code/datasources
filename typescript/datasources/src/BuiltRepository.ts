import type { IConnection } from "./IConnection.ts";
import type { Cells } from "./converters/Cell.ts";
import type { Constructable } from "./converters/Constructable.ts";
import type { TypeConverter } from "./converters/TypeConverter.ts";
import type { CRUDMapper } from "./CRUDMapper.ts";
import type { RepositoryStatements } from "./RepositoryStatements.ts";

export type BuiltRepository<T = Cells> = {
  connection: IConnection;
  rowConverter: TypeConverter<T, Cells>;
  crudMapper: CRUDMapper<T>;
  statements: RepositoryStatements;
  columns: string[];
  ids: string[];
  assignIdentityOnCreate: boolean;
  type: Constructable<T>;
};
