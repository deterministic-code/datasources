import type { IConnection } from "./IConnection.ts";
import type { Cells } from "./converters/Cell.ts";
import type { TypeConverter } from "./converters/TypeConverter.ts";
import type { StatementMapper } from "./mappers/StatementMapper.ts";
import type { RepositoryStatements } from "./RepositoryStatements.ts";

export type BuiltRepository<T = Cells> = {
  connection: IConnection;
  rowConverter: TypeConverter<T, Cells>;
  statementMapper: StatementMapper<T>;
  statements: RepositoryStatements;
  columns: string[];
  ids: string[];
  assignIdentityOnCreate: boolean;
};
