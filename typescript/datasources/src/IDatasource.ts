import type { ICommand } from "./ICommand.ts";
import type { IConnection } from "./IConnection.ts";
import type { IReadonlyRepository } from "./IReadonlyRepository.ts";
import type { IRepository } from "./IRepository.ts";

export interface IDatasource {
  readonly provider: string;
  createConnection(): Promise<IConnection>;
  createCommand(connection?: IConnection): ICommand;
  createRepository<T = unknown>(connection?: IConnection): IRepository<T>;
  createReadonlyRepository<T = unknown>(
    connection?: IConnection,
  ): IReadonlyRepository<T>;
}
