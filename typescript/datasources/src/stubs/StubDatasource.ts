import type { Constructable } from "../converters/Constructable.ts";
import type { ICommand } from "../ICommand.ts";
import type { IConnection } from "../IConnection.ts";
import type { IDatasource } from "../IDatasource.ts";
import type { IReadonlyRepository } from "../IReadonlyRepository.ts";
import type { ICRUDRepository } from "../ICRUDRepository.ts";
import { DatasourceError } from "../DatasourceError.ts";
import { RepositoryFactory } from "../RepositoryFactory.ts";
import { StubCommand } from "./StubCommand.ts";
import { StubConnection } from "./StubConnection.ts";
import { StubRepository } from "./StubRepository.ts";
export class StubDatasource implements IDatasource {
  private connection?: IConnection;
  readonly repositories = new RepositoryFactory();

  constructor(readonly provider: string) {}

  private defaultConnection = (): IConnection =>
    (this.connection ??= new StubConnection(this.provider));

  close = async (): Promise<void> => {
    const connection = this.connection;
    this.connection = undefined;
    await connection?.close();
  };

  [Symbol.asyncDispose] = (): Promise<void> => this.close();

  createConnection = (): Promise<IConnection> =>
    Promise.resolve(this.defaultConnection());

  createCommand = async (connection?: IConnection): Promise<ICommand> => {
    if (!connection) {
      connection = this.defaultConnection();
      if (connection.isOpen) throw new DatasourceError("Connection is already open");
      await connection.open();
    }
    return new StubCommand(this.provider, connection);
  };

  createRepository<T>(
    type: Constructable<T>,
    connection?: IConnection,
  ): ICRUDRepository<T>;
  createRepository(typeName: string, connection?: IConnection): ICRUDRepository;
  createRepository<T>(
    typeOrName: Constructable<T> | string,
    connection?: IConnection,
  ): ICRUDRepository<T> {
    if (typeof typeOrName === "string")
      return this.repositories.create(typeOrName, connection ?? this.defaultConnection()) as ICRUDRepository<T>;
    return new StubRepository<T>(this.provider, connection ?? this.defaultConnection(), typeOrName);
  }

  createReadonlyRepository<T>(
    type: Constructable<T>,
    connection?: IConnection,
  ): IReadonlyRepository<T>;
  createReadonlyRepository(typeName: string, connection?: IConnection): IReadonlyRepository;
  createReadonlyRepository<T>(
    typeOrName: Constructable<T> | string,
    connection?: IConnection,
  ): IReadonlyRepository<T> {
    if (typeof typeOrName === "string")
      return this.repositories.createReadonly(typeOrName, connection ?? this.defaultConnection()) as IReadonlyRepository<T>;
    return this.createRepository(typeOrName, connection);
  }
}
