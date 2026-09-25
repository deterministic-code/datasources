import type { Constructable } from "./converters/Constructable.ts";
import type { CRUDRepositoryOptions } from "./CRUDRepositoryOptions.ts";
import type { ICommand } from "./ICommand.ts";
import type { IConnection } from "./IConnection.ts";
import type { IDatasource } from "./IDatasource.ts";
import type { IDriver } from "./IDriver.ts";
import type { IReadonlyRepository } from "./IReadonlyRepository.ts";
import type { ICRUDRepository } from "./ICRUDRepository.ts";
import type { DatasourceOptions } from "./DatasourceOptions.ts";
import type { ReadonlyRepositoryOptions } from "./ReadonlyRepositoryOptions.ts";
import { CRUDRepository } from "./CRUDRepository.ts";
import { ReadonlyRepository } from "./ReadonlyRepository.ts";
import { createRepositoryOptions } from "./createRepositoryOptions.ts";

export class Datasource implements IDatasource {
  private connection?: IConnection;

  constructor(
    readonly provider: string,
    private readonly driver: IDriver,
    private readonly options?: DatasourceOptions,
  ) {}

  private defaultConnection = (): IConnection =>
    (this.connection ??= this.driver.connection(this.options));

  createConnection = (): Promise<IConnection> =>
    Promise.resolve(this.defaultConnection());

  createCommand = (connection?: IConnection): ICommand =>
    this.driver.command(connection ?? this.defaultConnection());

  private apply = <T>(
    type: Constructable<T>,
    options?: ReadonlyRepositoryOptions<T>,
  ): CRUDRepositoryOptions<T> => {
    const built = createRepositoryOptions(type, this.driver.rowTypeConverters);
    const types = this.driver.rowTypeConverters;
    return options === undefined
      ? { ...built, rowTypeConverters: types }
      : "ids" in options ||
          "insert" in options ||
          "update" in options ||
          "delete" in options ||
          "return" in options ||
          "create" in options ||
          "write" in options
        ? {
            entity: options.entity ?? built.entity,
            entityMapper: options.entityMapper,
            columnMappings: options.columnMappings,
            query: options.query,
            rowsMapper: options.rowsMapper,
            result: options.result,
            rowConverter: options.rowConverter,
            readProperties: options.readProperties,
            rowTypeConverters: types,
            type,
            ids: (options as CRUDRepositoryOptions<T>).ids ?? built.ids,
            insert: (options as CRUDRepositoryOptions<T>).insert,
            update: (options as CRUDRepositoryOptions<T>).update,
            delete: (options as CRUDRepositoryOptions<T>).delete,
            return: (options as CRUDRepositoryOptions<T>).return,
            create: (options as CRUDRepositoryOptions<T>).create,
            write: (options as CRUDRepositoryOptions<T>).write,
          }
        : {
            entity: options.entity ?? built.entity,
            entityMapper: options.entityMapper,
            columnMappings: options.columnMappings,
            query: options.query,
            rowsMapper: options.rowsMapper,
            result: options.result,
            rowConverter: options.rowConverter,
            readProperties: options.readProperties,
            rowTypeConverters: types,
            type,
            ids: built.ids,
            create: built.create,
          };
  };

  createRepository = <T>(
    type: Constructable<T>,
    connection?: IConnection,
    options?: CRUDRepositoryOptions<T>,
  ): ICRUDRepository<T> =>
    new CRUDRepository(
      connection ?? this.defaultConnection(),
      this.apply(type, options),
    );

  createReadonlyRepository = <T>(
    type: Constructable<T>,
    connection?: IConnection,
    options?: ReadonlyRepositoryOptions<T>,
  ): IReadonlyRepository<T> =>
    new ReadonlyRepository(
      connection ?? this.defaultConnection(),
      this.apply(type, options),
    );

  static create: (options?: DatasourceOptions) => IDatasource;

  static get default(): IDatasource {
    return Datasource.create();
  }
}
