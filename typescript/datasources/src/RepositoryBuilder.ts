import type { IConnection } from "./IConnection.ts";
import type { Cells } from "./converters/Cell.ts";
import type { Constructable } from "./converters/Constructable.ts";
import { DefaultTypeConverters } from "./converters/DefaultTypeConverters.ts";
import { RowTypeConverter } from "./converters/RowTypeConverter.ts";
import { NameSourceDestinationMapper } from "./mappers/NameSourceDestinationMapper.ts";
import { createRepositoryBuilderOptions, fieldNamesOf } from "./createRepositoryBuilderOptions.ts";
import type { BuiltRepository } from "./BuiltRepository.ts";
import type { RepositoryBuilderOptions } from "./RepositoryBuilderOptions.ts";
import { statementsOf } from "./RepositoryStatements.ts";

export type { BuiltRepository } from "./BuiltRepository.ts";
export type { RepositoryStatements } from "./RepositoryStatements.ts";

export const parts = <T extends Cells>(
  options: RepositoryBuilderOptions<T>,
): BuiltRepository<T> => {
  const types = {
    ...DefaultTypeConverters.for(options.connection.provider),
    ...options.converters,
  };
  const rowConverter =
    options.mapping ?? new RowTypeConverter(types, options.fields);
  if (rowConverter instanceof RowTypeConverter && Object.keys(rowConverter.types).length === 0)
    Object.assign(rowConverter.types, types);
  const mapping = new NameSourceDestinationMapper(options.connection.provider, "typescript");
  const table = options.table ?? options.type?.name ?? "";
  return {
    connection: options.connection,
    rowConverter,
    mapping,
    statements: statementsOf(table, options.type ? fieldNamesOf(options.type).sort() : []),
    ids: options.ids ?? ["id"],
    assignIdentityOnAdd: options.assignIdentityOnAdd !== false,
  };
};

export class RepositoryBuilder<T extends Cells> {
  constructor(private readonly options: RepositoryBuilderOptions<T>) {}

  static from<T extends Cells>(type: Constructable<T>, connection: IConnection): RepositoryBuilder<T>;
  static from<T extends Cells>(options: RepositoryBuilderOptions<T>): RepositoryBuilder<T>;
  static from<T extends Cells>(
    typeOrOptions: Constructable<T> | RepositoryBuilderOptions<T>,
    connection?: IConnection,
  ): RepositoryBuilder<T> {
    return new RepositoryBuilder(
      connection === undefined
        ? (typeOrOptions as RepositoryBuilderOptions<T>)
        : createRepositoryBuilderOptions(typeOrOptions as Constructable<T>, connection),
    );
  }

  parts = (): BuiltRepository<T> => parts(this.options);
}
