import type { IConnection } from "./IConnection.ts";
import type { Cells } from "./converters/Cell.ts";
import type { Constructable } from "./converters/Constructable.ts";
import { RowTypeConverter } from "./converters/RowTypeConverter.ts";
import { createRepositoryOptions, defaultId, fieldNamesOf } from "./createRepositoryOptions.ts";
import type { BuiltRepository } from "./BuiltRepository.ts";
import type { ReadonlyRepositoryOptions } from "./ReadonlyRepositoryOptions.ts";
import type { CRUDRepositoryOptions } from "./CRUDRepositoryOptions.ts";
import { statementsOf } from "./RepositoryStatements.ts";
import { IdentityMapper } from "./mappers/IdentityMapper.ts";
import { TransformMapper } from "./mappers/TransformMapper.ts";
import type { IMapper } from "./mappers/IMapper.ts";
import type { Scalar } from "./builders/Scalar.ts";

export type { BuiltRepository } from "./BuiltRepository.ts";
export type { RepositoryStatements } from "./RepositoryStatements.ts";

export const identityCreate = <T>(ids: readonly string[]): IMapper<Scalar, T> =>
  new TransformMapper(
    (value) =>
      value == null || ids.length !== 1 ? ({} as T) : ({ [ids[0]!]: value } as T),
    () => null,
  );

const isCrud = <T>(options: ReadonlyRepositoryOptions<T>): options is CRUDRepositoryOptions<T> =>
  "ids" in options ||
  "insert" in options ||
  "update" in options ||
  "delete" in options ||
  "return" in options ||
  "create" in options ||
  "write" in options;

export const parts = <T>(
  connection: IConnection,
  options: ReadonlyRepositoryOptions<T>,
): BuiltRepository<T> => {
  const types = { ...options.rowTypeConverters };
  const rowConverter = options.rowConverter ?? new RowTypeConverter<T>(types);
  if (rowConverter instanceof RowTypeConverter && Object.keys(rowConverter.types).length === 0)
    Object.assign(rowConverter.types, types);
  const entityMapper = options.entityMapper ?? new IdentityMapper<string>();
  const columnMappings = options.columnMappings ?? new IdentityMapper<string>();
  const rowsMapper = options.rowsMapper ?? new IdentityMapper<T>();
  const entity = entityMapper.map(options.entity ?? options.type?.name ?? "");
  const columns = [...(options.readProperties ?? (options.type ? fieldNamesOf(options.type) : []))]
    .map((name) => columnMappings.map(name))
    .sort((left, right) => left.localeCompare(right));
  const crud = isCrud(options) ? options : undefined;
  const ids = crud
    ? (crud.ids ?? (options.type ? defaultId(fieldNamesOf(options.type)) : ["id"]))
    : [];
  return {
    connection,
    rowConverter,
    rowsMapper,
    columnMappings,
    statements: statementsOf(entity, columns, options.query, crud?.insert, crud?.update, crud?.delete),
    readColumns: columns,
    ids,
    type: options.type ?? (Object as never as Constructable<T>),
    create: crud ? (crud.create ?? identityCreate<T>(ids)) : undefined,
  };
};

export class RepositoryBuilder<T = Cells> {
  constructor(
    private readonly connection: IConnection,
    private readonly options: ReadonlyRepositoryOptions<T>,
  ) {}

  static from<T>(type: Constructable<T>, connection: IConnection): RepositoryBuilder<T>;
  static from<T>(connection: IConnection, options: ReadonlyRepositoryOptions<T>): RepositoryBuilder<T>;
  static from<T>(
    typeOrConnection: Constructable<T> | IConnection,
    connectionOrOptions: IConnection | ReadonlyRepositoryOptions<T>,
  ): RepositoryBuilder<T> {
    return typeof typeOrConnection === "function"
      ? new RepositoryBuilder(connectionOrOptions as IConnection, createRepositoryOptions(typeOrConnection))
      : new RepositoryBuilder(typeOrConnection, connectionOrOptions as ReadonlyRepositoryOptions<T>);
  }

  parts = (): BuiltRepository<T> => parts(this.connection, this.options);
}
