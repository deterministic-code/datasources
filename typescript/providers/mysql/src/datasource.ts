import type {
  DatasourceFactory,
  ICommand,
  IConnection,
  IDatasource,
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
  static readonly factory = new MysqlDatasource();
  static readonly provider = "mysql";
  readonly provider = "mysql";

  constructor(private readonly options: MysqlDatasourceOptions = {}) {}

  static create = (options?: unknown): IDatasource =>
    new MysqlDatasource((options ?? {}) as MysqlDatasourceOptions);

  create = (options?: unknown): IDatasource => MysqlDatasource.create(options);

  private newConnection = (): MysqlConnection => {
    requireDriver(this.options.driver);
    return new MysqlConnection(this.options);
  };

  createConnection = (): Promise<IConnection> =>
    Promise.resolve(this.newConnection());

  createCommand = (connection?: IConnection): ICommand =>
    new MysqlCommand(
      (connection as MysqlConnection | undefined) ?? this.newConnection(),
    );

  createRepository = <T>(connection?: IConnection): IRepository<T> =>
    new MysqlRepository<T>(
      (connection as MysqlConnection | undefined) ?? this.newConnection(),
      { table: this.options.table },
    );
}
