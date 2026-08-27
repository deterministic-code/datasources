import type { ICommand } from "../ICommand.ts";
import type { IConnection } from "../IConnection.ts";
import type { IDatasource } from "../IDatasource.ts";
import type { IRepository } from "../IRepository.ts";
import type { RepositoryOptions } from "../RepositoryOptions.ts";
import { MockCommand } from "./mock-command.ts";
import { MockConnection } from "./mock-connection.ts";
import { MockRepository } from "./mock-repository.ts";

export class MockDatasource implements IDatasource {
  createConnectionCalls = 0;
  createCommandCalls: Array<IConnection | undefined> = [];
  createRepositoryCalls: Array<{
    connection?: IConnection;
    options?: RepositoryOptions;
  }> = [];
  connection: IConnection;
  command?: ICommand;
  repository?: IRepository;

  constructor(readonly provider = "mock") {
    this.connection = new MockConnection();
  }

  create(_options?: unknown): IDatasource {
    return new MockDatasource(this.provider);
  }

  createConnection(): Promise<IConnection> {
    this.createConnectionCalls += 1;
    return Promise.resolve(this.connection);
  }

  createCommand(connection?: IConnection): ICommand {
    this.createCommandCalls.push(connection);
    if (this.command) {
      return this.command;
    }
    return new MockCommand(connection ?? this.connection);
  }

  createRepository<T = unknown>(
    connection?: IConnection,
    options?: RepositoryOptions,
  ): IRepository<T> {
    this.createRepositoryCalls.push({ connection, options });
    if (this.repository) {
      return this.repository as IRepository<T>;
    }
    return new MockRepository<T>(connection ?? this.connection);
  }
}
