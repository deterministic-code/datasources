import type { ICommand } from "./ICommand.ts";
import type { IConnection } from "./IConnection.ts";
import type { IReadonlyRepository } from "./IReadonlyRepository.ts";
import type { ICRUDRepository } from "./ICRUDRepository.ts";

export interface IDatasource {
  readonly provider: string;
  createConnection(): Promise<IConnection>;
  createCommand(connection?: IConnection): ICommand;
  createRepository<T>(connection?: IConnection): ICRUDRepository<T>;
  createReadonlyRepository<T>(
    connection?: IConnection,
  ): IReadonlyRepository<T>;
}
