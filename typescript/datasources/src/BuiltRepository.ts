import type { IConnection } from "./IConnection.ts";
import type { Cells } from "./converters/Cell.ts";
import type { TypeConverter } from "./converters/TypeConverter.ts";
import type { IMapper } from "./mappers/IMapper.ts";
import type { NameSourceDestinationMapper } from "./mappers/NameSourceDestinationMapper.ts";
import type { RepositoryStatements } from "./RepositoryStatements.ts";

export type BuiltRepository<T = Cells> = {
  connection: IConnection;
  rowConverter: TypeConverter<T, Cells>;
  mapper: IMapper<T, T>;
  mapping: NameSourceDestinationMapper;
  statements: RepositoryStatements;
  ids: string[];
  assignIdentityOnAdd: boolean;
};
