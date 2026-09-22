import type { IConnection } from "./IConnection.ts";
import type { Cells } from "./converters/Cell.ts";
import type { Constructable } from "./converters/Constructable.ts";
import { RowTypeConverter } from "./converters/RowTypeConverter.ts";
import { CRUDMapper } from "./CRUDMapper.ts";
import { TransformMapper } from "./mappers/TransformMapper.ts";
import { createRepositoryBuilderOptions, defaultId, fieldNamesOf } from "./createRepositoryBuilderOptions.ts";
import type { BuiltRepository } from "./BuiltRepository.ts";
import type { RepositoryBuilderOptions } from "./RepositoryBuilderOptions.ts";
import { statementsOf } from "./RepositoryStatements.ts";

export type { BuiltRepository } from "./BuiltRepository.ts";
export type { RepositoryStatements } from "./RepositoryStatements.ts";

export const parts = <T>(
  options: RepositoryBuilderOptions<T>,
): BuiltRepository<T> => {
  const types = {
    ...options.types,
    ...options.converters,
  };
  const rowConverter =
    options.mapping ?? new RowTypeConverter<T>(types, options.fields);
  if (rowConverter instanceof RowTypeConverter && Object.keys(rowConverter.types).length === 0)
    Object.assign(rowConverter.types, types);
  const names = new TransformMapper<string, string>((name) => name, (name) => name);
  const crudMapper =
    options.crudMapper ??
    new CRUDMapper({
      entity: names,
      columns: names,
      rows: options.mapper ?? new TransformMapper<T, T>((value) => value, (value) => value),
    });
  const entity = crudMapper.entity.map(options.entity ?? options.type?.name ?? "");
  const columns = [...(options.columns ?? (options.type ? fieldNamesOf(options.type) : []))]
    .map((name) => crudMapper.columns.map(name))
    .sort((left, right) => left.localeCompare(right));
  return {
    connection: options.connection,
    rowConverter,
    crudMapper,
    statements: statementsOf(entity, columns, crudMapper),
    columns,
    ids: options.ids ?? (options.type ? defaultId(fieldNamesOf(options.type)) : []),
    type: options.type ?? (Object as never as Constructable<T>),
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
