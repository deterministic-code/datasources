import type { IConnection } from "./IConnection.ts";
import type { Cells } from "./converters/Cell.ts";
import type { Constructable } from "./converters/Constructable.ts";
import type { TypeConverter } from "./converters/TypeConverter.ts";
import type { IMapper } from "./mappers/IMapper.ts";
import type { ReadonlyRepositoryStatements } from "./ReadonlyRepositoryStatements.ts";

export type BuiltRepository<T = Cells> = {
  connection: IConnection;
  rowConverter?: TypeConverter<Cells, Cells>;
  objectConverter: TypeConverter<T, Cells>;
  propertyMappings: IMapper<string, string>;
  entityMapper: IMapper<string, string>;
  statements: ReadonlyRepositoryStatements;
  ids: string[];
  type: Constructable<T>;
};
