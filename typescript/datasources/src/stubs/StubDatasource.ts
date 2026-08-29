import type { DatasourceOptions } from "../DatasourceOptions.ts";
import type { ICommand } from "../ICommand.ts";
import type { IConnection } from "../IConnection.ts";
import type { IDatasource } from "../IDatasource.ts";
import type { IReadonlyRepository } from "../IReadonlyRepository.ts";
import type { IRepository } from "../IRepository.ts";
import { StubCommand } from "./StubCommand.ts";
import { StubConnection } from "./StubConnection.ts";
import { StubRepository } from "./StubRepository.ts";

export class StubDatasource implements IDatasource {
  private connection?: IConnection;

  constructor(readonly provider: string) {}

  create = (_options?: DatasourceOptions): IDatasource =>
    new StubDatasource(this.provider);

  private defaultConnection = (): IConnection =>
    (this.connection ??= new StubConnection(this.provider));

  createConnection = (): Promise<IConnection> =>
    Promise.resolve(this.defaultConnection());

  createCommand = (connection?: IConnection): ICommand =>
    new StubCommand(this.provider, connection ?? this.defaultConnection());

  createRepository = <T>(connection?: IConnection): IRepository<T> =>
    new StubRepository<T>(this.provider, connection ?? this.defaultConnection());

  createReadonlyRepository = <T>(
    connection?: IConnection,
  ): IReadonlyRepository<T> => this.createRepository(connection);
}
