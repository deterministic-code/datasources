import type { Constructable } from "../converters/Constructable.ts";
import type { ICommand } from "../ICommand.ts";
import type { IConnection } from "../IConnection.ts";
import type { IDatasource } from "../IDatasource.ts";
import type { IReadonlyRepository } from "../IReadonlyRepository.ts";
import type { ICRUDRepository } from "../ICRUDRepository.ts";
import { DatasourceError } from "../DatasourceError.ts";
import { RepositoryFactory } from "../RepositoryFactory.ts";
import { MockCommand } from "./mock-command.ts";
import { MockConnection } from "./mock-connection.ts";
import { MockRepository } from "./mock-repository.ts";
export class MockDatasource implements IDatasource {
  createConnectionCalls = 0;
  createCommandCalls: Array<IConnection | undefined> = [];
  createRepositoryCalls: Array<IConnection | undefined> = [];
  connection: IConnection;
  command?: ICommand;
  repository?: ICRUDRepository;
  readonly repositories = new RepositoryFactory();

  constructor(readonly provider = "mock") {
    this.connection = new MockConnection();
  }

  createConnection(): Promise<IConnection> {
    this.createConnectionCalls += 1;
    return Promise.resolve(this.connection);
  }

  async createCommand(connection?: IConnection): Promise<ICommand> {
    this.createCommandCalls.push(connection);
    if (connection === undefined) {
      if (this.connection.isOpen) throw new DatasourceError("Connection is already open");
      await this.connection.open();
    }
    return this.command ?? new MockCommand(connection ?? this.connection);
  }

  createRepository<T>(type: Constructable<T>, connection?: IConnection): ICRUDRepository<T>;
  createRepository(typeName: string, connection?: IConnection): ICRUDRepository;
  createRepository<T>(
    typeOrName: Constructable<T> | string,
    connection?: IConnection,
  ): ICRUDRepository<T> {
    this.createRepositoryCalls.push(connection);
    if (typeof typeOrName === "string")
      return this.repositories.create(typeOrName, connection ?? this.connection) as ICRUDRepository<T>;
    return (this.repository as ICRUDRepository<T> | undefined) ??
      new MockRepository<T>(connection ?? this.connection, typeOrName);
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
      return this.repositories.createReadonly(typeOrName, connection ?? this.connection) as IReadonlyRepository<T>;
    return this.createRepository(typeOrName, connection);
  }
}
