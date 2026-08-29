import type { DatasourceOptions } from "../DatasourceOptions.ts";
import type { ICommand } from "../ICommand.ts";
import type { IConnection } from "../IConnection.ts";
import type { IDatasource } from "../IDatasource.ts";
import type { IReadonlyRepository } from "../IReadonlyRepository.ts";
import type { IRepository } from "../IRepository.ts";
import { MockCommand } from "./mock-command.ts";
import { MockConnection } from "./mock-connection.ts";
import { MockRepository } from "./mock-repository.ts";

export class MockDatasource implements IDatasource {
  createConnectionCalls = 0;
  createCommandCalls: Array<IConnection | undefined> = [];
  createRepositoryCalls: Array<IConnection | undefined> = [];
  connection: IConnection;
  command?: ICommand;
  repository?: IRepository;

  constructor(readonly provider = "mock") {
    this.connection = new MockConnection();
  }

  create = (_options?: DatasourceOptions): IDatasource =>
    new MockDatasource(this.provider);

  createConnection(): Promise<IConnection> {
    this.createConnectionCalls += 1;
    return Promise.resolve(this.connection);
  }

  createCommand(connection?: IConnection): ICommand {
    this.createCommandCalls.push(connection);
    return this.command ?? new MockCommand(connection ?? this.connection);
  }

  createRepository<T>(connection?: IConnection): IRepository<T> {
    this.createRepositoryCalls.push(connection);
    return (this.repository as IRepository<T> | undefined) ??
      new MockRepository<T>(connection ?? this.connection);
  }

  createReadonlyRepository<T>(
    connection?: IConnection,
  ): IReadonlyRepository<T> {
    return this.createRepository(connection);
  }
}
