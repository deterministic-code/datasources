import type {
  IDatasourceFactory,
  DatasourceOptions,
  ICommand,
  IConnection,
  IDatasource,
  IReadonlyRepository,
  ICRUDRepository,
} from "@datasources/core";
import { Repository, RowTypeConverter } from "@datasources/core";
import type { MysqlConnectionOptions } from "./connection-options.ts";
import { MysqlConnection } from "./connection.ts";
import { MysqlCommand } from "./command.ts";
import { requireDriver } from "./driver.ts";
import "./converters/MysqlTypes.ts";

export interface MysqlDatasourceOptions extends MysqlConnectionOptions {
  table?: string;
}

export class MysqlDatasource implements IDatasource, IDatasourceFactory {
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

  createRepository = <T>(connection?: IConnection): ICRUDRepository<T> =>
    new Repository({
      connection:
        (connection as MysqlConnection | undefined) ?? this.defaultConnection(),
      entity: this.options.table ?? "items",
      mapping: new RowTypeConverter(),
    }) as ICRUDRepository<T>;

  createReadonlyRepository = <T>(
    connection?: IConnection,
  ): IReadonlyRepository<T> => this.createRepository(connection);
}
