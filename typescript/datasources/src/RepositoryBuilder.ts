import type { IConnection } from "./IConnection.ts";
import type { Cells } from "./converters/Cell.ts";
import type { Constructable } from "./converters/Constructable.ts";
import { MetaDataRowTypeConverter } from "./converters/MetaDataRowTypeConverter.ts";
import { createCRUDRepositoryOptions, defaultId, fieldNamesOf } from "./createCRUDRepositoryOptions.ts";
import type { BuiltRepository } from "./BuiltRepository.ts";
import type { ReadonlyRepositoryOptions } from "./ReadonlyRepositoryOptions.ts";
import type { CRUDRepositoryOptions } from "./CRUDRepositoryOptions.ts";
import { statementsOf } from "./RepositoryStatements.ts";
import { IdentityMapper } from "./mappers/IdentityMapper.ts";

const isCrud = <T>(options: ReadonlyRepositoryOptions<T>): options is CRUDRepositoryOptions<T> =>
  "ids" in options ||
  "insert" in options ||
  "update" in options ||
  "delete" in options ||
  "return" in options ||
  "write" in options;

export const parts = <T>(
  connection: IConnection,
  options: ReadonlyRepositoryOptions<T>,
): BuiltRepository<T> => {
  const rowConverter = options.rowConverter;
  const objectConverter = options.objectConverter ?? new MetaDataRowTypeConverter<T>();
  const entityMapper = options.entityMapper ?? new IdentityMapper<string>();
  const columnMappings = options.columnMappings ?? new IdentityMapper<string>();
  const rowsMapper = options.rowsMapper ?? new IdentityMapper<T>();
  const entity = entityMapper.map(options.entity ?? options.type?.name ?? "");
  const names = options.readProperties ?? (options.type ? fieldNamesOf(options.type) : []);
  if (objectConverter instanceof MetaDataRowTypeConverter && options.type) {
    const instance = new options.type() as Record<string, unknown>;
    objectConverter.references.clear();
    objectConverter.referenceColumns = columnMappings;
    for (const name of names) {
      const value = instance[name];
      if (value !== null && typeof value === "object" && !Array.isArray(value))
        objectConverter.references.add(name);
    }
  }
  const columns = [...names]
    .map((name) => columnMappings.map(name))
    .sort((left, right) => left.localeCompare(right));
  const crud = isCrud(options) ? options : undefined;
  const ids = crud
    ? (crud.ids ?? (options.type ? defaultId(fieldNamesOf(options.type)) : ["id"]))
    : [];
  return {
    connection,
    rowConverter,
    objectConverter,
    rowsMapper,
    columnMappings,
    entityMapper,
    statements: statementsOf(entity, columns, options.select, crud?.insert, crud?.update, crud?.delete),
    readColumns: columns,
    ids,
    type: options.type ?? (Object as never as Constructable<T>),
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
      ? new RepositoryBuilder(connectionOrOptions as IConnection, createCRUDRepositoryOptions(typeOrConnection))
      : new RepositoryBuilder(typeOrConnection, connectionOrOptions as ReadonlyRepositoryOptions<T>);
  }

  parts = (): BuiltRepository<T> => parts(this.connection, this.options);
}
