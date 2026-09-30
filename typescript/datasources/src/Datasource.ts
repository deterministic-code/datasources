import type { Cell } from "./converters/Cell.ts";
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
import { RowTypeConverter } from "./converters/RowTypeConverter.ts";
import { createCRUDRepositoryOptions } from "./createCRUDRepositoryOptions.ts";

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
    const built = createCRUDRepositoryOptions(type, this.driver.rowTypeConverters);
    const types = this.driver.rowTypeConverters;
    const rows = options?.rowConverter ?? RowTypeConverter.from(this.driver.rowTypeConverters, columnsOf(type));
    const crud = options !== undefined && ("ids" in options || "write" in options)
      ? (options as CRUDRepositoryOptions<T>)
      : undefined;
    return options === undefined
      ? { ...built, rowTypeConverters: types, rowConverter: rows }
      : {
          entity: options.entity ?? built.entity,
          entityMapper: options.entityMapper,
          propertyMappings: options.propertyMappings,
          rowConverter: rows,
          objectConverter: options.objectConverter,
          readProperties: options.readProperties,
          rowTypeConverters: types,
          type,
          ids: crud?.ids ?? built.ids,
          write: crud?.write,
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

const columnsOf = <T>(type: Constructable<T>): Record<string, string> => {
  const instance = new type() as Record<string, Cell | object>;
  const columns: Record<string, string> = {};
  for (const [name, value] of Object.entries(instance)) {
    if (Array.isArray(value)) continue;
    const canonical = canonicalOf(value);
    if (canonical) columns[name] = canonical;
  }
  return columns;
};

const canonicalOf = (value: Cell | object): string | undefined =>
  typeof value === "boolean"
    ? "bool"
    : value instanceof Date
      ? "datetime"
      : value instanceof Uint8Array
        ? "binary"
        : undefined;
