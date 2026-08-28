import type { ICommand } from "../ICommand.ts";
import type { IConnection } from "../IConnection.ts";
import type { IDatasource } from "../IDatasource.ts";
import type { IReadonlyRepository } from "../IReadonlyRepository.ts";
import type { IRepository } from "../IRepository.ts";
import { StubCommand } from "./StubCommand.ts";
import { StubConnection } from "./StubConnection.ts";
import { StubRepository } from "./StubRepository.ts";

export class StubDatasource implements IDatasource {
  constructor(readonly provider: string) {}

  create = (_options?: unknown): IDatasource =>
    new StubDatasource(this.provider);

  private newConnection = (): IConnection =>
    new StubConnection(this.provider);

  createConnection = (): Promise<IConnection> =>
    Promise.resolve(this.newConnection());

  createCommand = (connection?: IConnection): ICommand =>
    new StubCommand(this.provider, connection ?? this.newConnection());

  createRepository = <T = unknown>(connection?: IConnection): IRepository<T> =>
    new StubRepository<T>(this.provider, connection ?? this.newConnection());

  createReadonlyRepository = <T = unknown>(
    connection?: IConnection,
  ): IReadonlyRepository<T> => this.createRepository(connection);
}
