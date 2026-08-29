import type {
  DatasourceFactory,
  DatasourceOptions,
  ICommand,
  IConnection,
  IDatasource,
  IReadonlyRepository,
  IRepository,
} from "@datasources/core";
import {
  MysqlConnection,
  type MysqlConnectionOptions,
} from "./connection.ts";
import { MysqlCommand } from "./command.ts";
import { MysqlRepository } from "./repository.ts";
import { requireDriver } from "./driver.ts";

export interface MysqlDatasourceOptions extends MysqlConnectionOptions {
  table?: string;
}

export class MysqlDatasource implements IDatasource, DatasourceFactory {
  static readonly instance = new MysqlDatasource();
  static readonly provider = "mysql";
  readonly provider = "mysql";

  constructor(private readonly options: MysqlDatasourceOptions = {}) {}

  private connection?: MysqlConnection;

  static create = (options?: DatasourceOptions): IDatasource =>
    new MysqlDatasource((options ?? {}) as MysqlDatasourceOptions);

  create = (options?: DatasourceOptions): IDatasource =>
    MysqlDatasource.create(options);

  private newConnection = (): MysqlConnection => {
    requireDriver(this.options.driver);
    return new MysqlConnection(this.options);
  };

  private defaultConnection = (): MysqlConnection =>
    (this.connection ??= this.newConnection());

  createConnection = (): Promise<IConnection> =>
    Promise.resolve(this.defaultConnection());

  createCommand = (connection?: IConnection): ICommand =>
    new MysqlCommand(
      (connection as MysqlConnection | undefined) ?? this.defaultConnection(),
    );

  createRepository = <T>(connection?: IConnection): IRepository<T> =>
    new MysqlRepository<T>(
      (connection as MysqlConnection | undefined) ?? this.defaultConnection(),
      { table: this.options.table },
    );

  createReadonlyRepository = <T>(
    connection?: IConnection,
  ): IReadonlyRepository<T> => this.createRepository(connection);
}
