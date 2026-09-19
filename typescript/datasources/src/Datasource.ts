import type { ICommand } from "./ICommand.ts";
import type { IConnection } from "./IConnection.ts";
import type { IDatasource } from "./IDatasource.ts";
import type { IReadonlyRepository } from "./IReadonlyRepository.ts";
import type { ICRUDRepository } from "./ICRUDRepository.ts";
import type { DatasourceOptions } from "./DatasourceOptions.ts";

export class Datasource implements IDatasource {
  private connection?: IConnection;

  constructor(
    readonly provider: string,
    private readonly newConnection: () => IConnection,
    private readonly newRepository: (connection: IConnection) => object,
  ) {}

  private defaultConnection = (): IConnection =>
    (this.connection ??= this.newConnection());

  createConnection = (): Promise<IConnection> =>
    Promise.resolve(this.defaultConnection());

  createCommand = (connection?: IConnection): ICommand =>
    (connection ?? this.defaultConnection()).createCommand();

  createRepository = <T>(connection?: IConnection): ICRUDRepository<T> =>
    this.newRepository(connection ?? this.defaultConnection()) as ICRUDRepository<T>;

  createReadonlyRepository = <T>(
    connection?: IConnection,
  ): IReadonlyRepository<T> => this.createRepository(connection);

  static create: (options?: DatasourceOptions) => IDatasource;
}
