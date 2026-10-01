import type { Constructable } from "../converters/Constructable.ts";
import type { ICommand } from "../ICommand.ts";
import type { IConnection } from "../IConnection.ts";
import type { IDatasource } from "../IDatasource.ts";
import type { IReadonlyRepository } from "../IReadonlyRepository.ts";
import type { ICRUDRepository } from "../ICRUDRepository.ts";
import { DatasourceError } from "../DatasourceError.ts";
import { StubCommand } from "./StubCommand.ts";
import { StubConnection } from "./StubConnection.ts";
import { StubRepository } from "./StubRepository.ts";
export class StubDatasource implements IDatasource {
  private connection?: IConnection;

  constructor(readonly provider: string) {}

  private defaultConnection = (): IConnection =>
    (this.connection ??= new StubConnection(this.provider));

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

  createRepository = <T>(
    type: Constructable<T>,
    connection?: IConnection,
  ): ICRUDRepository<T> =>
    new StubRepository<T>(this.provider, connection ?? this.defaultConnection(), type);

  createReadonlyRepository = <T>(
    type: Constructable<T>,
    connection?: IConnection,
  ): IReadonlyRepository<T> => this.createRepository(type, connection);
}
