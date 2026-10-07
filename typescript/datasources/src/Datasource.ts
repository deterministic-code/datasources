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
import type { ReadonlyRepositoryMappingOptions } from "./ReadonlyRepositoryMappingOptions.ts";
import { DatasourceError } from "./DatasourceError.ts";
import { CRUDRepository } from "./CRUDRepository.ts";
import { ReadonlyRepository } from "./ReadonlyRepository.ts";
import { RepositoryFactory } from "./RepositoryFactory.ts";
import { RepositoryNotRegisteredError } from "./RepositoryNotRegisteredError.ts";
import { RowTypeConverter } from "./converters/RowTypeConverter.ts";
import { createCRUDRepositoryOptions } from "./createCRUDRepositoryOptions.ts";
import { createReadonlyRepositoryMappingOptions } from "./createReadonlyRepositoryMappingOptions.ts";

export class Datasource implements IDatasource {
  private connection?: IConnection;
  readonly repositories = new RepositoryFactory();

  constructor(
    readonly provider: string,
    private readonly driver: IDriver,
    private readonly options?: DatasourceOptions,
  ) {
    const self = this;
    return new Proxy(this, {
      get(target, prop, receiver) {
        if (typeof prop !== "string" || Reflect.has(target, prop))
          return Reflect.get(target, prop, receiver);
        if (self.repositories.has(prop))
          return self.repositories.access(prop, self.defaultConnection());
        throw new RepositoryNotRegisteredError(prop);
      },
    });
  }

  private defaultConnection = (): IConnection =>
    (this.connection ??= this.driver.connection(this.options));

  createConnection = (): Promise<IConnection> =>
    Promise.resolve(this.defaultConnection());

  createCommand = async (connection?: IConnection): Promise<ICommand> => {
    if (!connection) {
      connection = this.defaultConnection();
      if (connection.isOpen) throw new DatasourceError("Connection is already open");
      await connection.open();
    }
    return this.driver.command(connection);
  };

  private applyCrud = <T>(
    type: Constructable<T>,
    options?: CRUDRepositoryOptions<T>,
  ): CRUDRepositoryOptions<T> => {
    const built = createCRUDRepositoryOptions(type, this.driver.rowTypeConverters);
    const types = this.driver.rowTypeConverters;
    const rows = options?.rowConverter ?? RowTypeConverter.from(this.driver.rowTypeConverters, columnsOf(type));
    return {
      ...(options ?? built),
      entity: options?.entity ?? built.entity,
      rowConverter: rows,
      rowTypeConverters: types,
      type,
    };
  };

  private applyReadonly = <T>(
    type: Constructable<T>,
    options?: ReadonlyRepositoryMappingOptions<T>,
  ): ReadonlyRepositoryMappingOptions<T> => {
    const built = createReadonlyRepositoryMappingOptions(type, this.driver.rowTypeConverters);
    const types = this.driver.rowTypeConverters;
    const rows = options?.rowConverter ?? RowTypeConverter.from(this.driver.rowTypeConverters, columnsOf(type));
    return {
      ...(options ?? built),
      entity: options?.entity ?? built.entity,
      rowConverter: rows,
      rowTypeConverters: types,
      type,
    };
  };

  createRepository<T>(
    type: Constructable<T>,
    connection?: IConnection,
    options?: CRUDRepositoryOptions<T>,
  ): ICRUDRepository<T>;
  createRepository(typeName: string, connection?: IConnection): ICRUDRepository;
  createRepository<T>(
    typeOrName: Constructable<T> | string,
    connection?: IConnection,
    options?: CRUDRepositoryOptions<T>,
  ): ICRUDRepository<T> {
    if (typeof typeOrName === "string")
      return this.repositories.create(typeOrName, connection ?? this.defaultConnection()) as ICRUDRepository<T>;
    return new CRUDRepository(
      connection ?? this.defaultConnection(),
      this.applyCrud(typeOrName, options),
    );
  }

  createReadonlyRepository<T>(
    type: Constructable<T>,
    connection?: IConnection,
    options?: ReadonlyRepositoryMappingOptions<T>,
  ): IReadonlyRepository<T>;
  createReadonlyRepository(typeName: string, connection?: IConnection): IReadonlyRepository;
  createReadonlyRepository<T>(
    typeOrName: Constructable<T> | string,
    connection?: IConnection,
    options?: ReadonlyRepositoryMappingOptions<T>,
  ): IReadonlyRepository<T> {
    if (typeof typeOrName === "string")
      return this.repositories.createReadonly(typeOrName, connection ?? this.defaultConnection()) as IReadonlyRepository<T>;
    return new ReadonlyRepository(
      connection ?? this.defaultConnection(),
      this.applyReadonly(typeOrName, options),
    );
  }

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
