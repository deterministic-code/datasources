import type { IConnection } from "./IConnection.ts";
import type { Cells } from "./converters/Cell.ts";
import type { Constructable } from "./converters/Constructable.ts";
import { DefaultTypeConverters } from "./converters/DefaultTypeConverters.ts";
import { RowTypeConverter } from "./converters/RowTypeConverter.ts";
import { NameSourceDestinationMapper } from "./mappers/NameSourceDestinationMapper.ts";
import { StatementMapper } from "./mappers/StatementMapper.ts";
import { TransformMapper } from "./mappers/TransformMapper.ts";
import { createRepositoryBuilderOptions, fieldNamesOf } from "./createRepositoryBuilderOptions.ts";
import type { BuiltRepository } from "./BuiltRepository.ts";
import type { RepositoryBuilderOptions } from "./RepositoryBuilderOptions.ts";
import { statementsOf } from "./RepositoryStatements.ts";

export type { BuiltRepository } from "./BuiltRepository.ts";
export type { RepositoryStatements } from "./RepositoryStatements.ts";

export const parts = <T>(
  options: RepositoryBuilderOptions<T>,
): BuiltRepository<T> => {
  const types = {
    ...DefaultTypeConverters.for(options.connection.provider),
    ...options.converters,
  };
  const rowConverter =
    options.mapping ?? new RowTypeConverter<T>(types, options.fields);
  if (rowConverter instanceof RowTypeConverter && Object.keys(rowConverter.types).length === 0)
    Object.assign(rowConverter.types, types);
  const names = new NameSourceDestinationMapper(options.connection.provider, "typescript");
  const statementMapper =
    options.statementMapper ??
    new StatementMapper({
      entity: names,
      columns: names,
      rows: options.mapper ?? new TransformMapper<T, T>((value) => value, (value) => value),
    });
  const entity = statementMapper.entity.map(options.entity ?? options.type?.name ?? "");
  const columns = [...(options.columns ?? (options.type ? fieldNamesOf(options.type) : []))]
    .map((name) => statementMapper.columns.map(name))
    .sort((left, right) => left.localeCompare(right));
  return {
    connection: options.connection,
    rowConverter,
    statementMapper,
    statements: statementsOf(entity, columns, statementMapper),
    columns,
    ids: options.ids ?? ["id"],
    assignIdentityOnAdd: options.assignIdentityOnAdd !== false,
  };
};

export class RepositoryBuilder<T = Cells> {
  constructor(private readonly options: RepositoryBuilderOptions<T>) {}

  static from<T>(type: Constructable<T>, connection: IConnection): RepositoryBuilder<T>;
  static from<T>(options: RepositoryBuilderOptions<T>): RepositoryBuilder<T>;
  static from<T>(
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
