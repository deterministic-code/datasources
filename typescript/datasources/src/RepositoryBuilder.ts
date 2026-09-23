import type { IConnection } from "./IConnection.ts";
import type { Cells } from "./converters/Cell.ts";
import type { Constructable } from "./converters/Constructable.ts";
import { RowTypeConverter } from "./converters/RowTypeConverter.ts";
import { CRUDMapperOptions } from "./CRUDMapperOptions.ts";
import { createRepositoryOptions, defaultId, fieldNamesOf } from "./createRepositoryOptions.ts";
import type { BuiltRepository } from "./BuiltRepository.ts";
import type { RepositoryOptions } from "./RepositoryOptions.ts";
import { statementsOf } from "./RepositoryStatements.ts";

export type { BuiltRepository } from "./BuiltRepository.ts";
export type { RepositoryStatements } from "./RepositoryStatements.ts";

export const parts = <T>(
  connection: IConnection,
  options: RepositoryOptions<T>,
): BuiltRepository<T> => {
  const types = { ...options.rowTypeConverters };
  const rowConverter =
    options.rowConverter ?? new RowTypeConverter<T>(types);
  if (rowConverter instanceof RowTypeConverter && Object.keys(rowConverter.types).length === 0)
    Object.assign(rowConverter.types, types);
  const crudMapper = options.crudMapper ?? new CRUDMapperOptions();
  const entity = crudMapper.entity.map(options.entity ?? options.type?.name ?? "");
  const columns = [...(options.readColumns ?? (options.type ? fieldNamesOf(options.type) : []))]
    .map((name) => crudMapper.columns.map(name))
    .sort((left, right) => left.localeCompare(right));
  return {
    connection,
    rowConverter,
    crudMapper,
    statements: statementsOf(entity, columns, crudMapper),
    readColumns: columns,
    ids: options.ids ?? (options.type ? defaultId(fieldNamesOf(options.type)) : []),
    type: options.type ?? (Object as never as Constructable<T>),
  };
};

export class RepositoryBuilder<T = Cells> {
  constructor(
    private readonly connection: IConnection,
    private readonly options: RepositoryOptions<T>,
  ) {}

  static from<T>(type: Constructable<T>, connection: IConnection): RepositoryBuilder<T>;
  static from<T>(connection: IConnection, options: RepositoryOptions<T>): RepositoryBuilder<T>;
  static from<T>(
    typeOrConnection: Constructable<T> | IConnection,
    connectionOrOptions: IConnection | RepositoryOptions<T>,
  ): RepositoryBuilder<T> {
    return typeof typeOrConnection === "function"
      ? new RepositoryBuilder(connectionOrOptions as IConnection, createRepositoryOptions(typeOrConnection))
      : new RepositoryBuilder(typeOrConnection, connectionOrOptions as RepositoryOptions<T>);
  }

  parts = (): BuiltRepository<T> => parts(this.connection, this.options);
}
