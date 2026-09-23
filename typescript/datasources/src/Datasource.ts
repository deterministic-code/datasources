import type { Constructable } from "./converters/Constructable.ts";
import type { ICommand } from "./ICommand.ts";
import type { IConnection } from "./IConnection.ts";
import type { IDatasource } from "./IDatasource.ts";
import type { IDriver } from "./IDriver.ts";
import type { IReadonlyRepository } from "./IReadonlyRepository.ts";
import type { ICRUDRepository } from "./ICRUDRepository.ts";
import type { DatasourceOptions } from "./DatasourceOptions.ts";
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

  createRepository = <T>(
    type: Constructable<T>,
    connection?: IConnection,
  ): ICRUDRepository<T> =>
    new CRUDRepository(
      connection ?? this.defaultConnection(),
      createRepositoryOptions(type, this.driver.rowTypeConverters),
    );

  createReadonlyRepository = <T>(
    type: Constructable<T>,
    connection?: IConnection,
  ): IReadonlyRepository<T> =>
    new ReadonlyRepository(
      connection ?? this.defaultConnection(),
      createRepositoryOptions(type, this.driver.rowTypeConverters),
    );

  static create: (options?: DatasourceOptions) => IDatasource;

  static get default(): IDatasource {
    return Datasource.create();
  }
}
