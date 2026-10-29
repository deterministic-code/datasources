import type { Constructable } from "./converters/Constructable.ts";
import type { CRUDRepositoryOptions } from "./CRUDRepositoryOptions.ts";
import type { ICommand } from "./ICommand.ts";
import type { IConnection } from "./IConnection.ts";
import type { IReadonlyRepository } from "./IReadonlyRepository.ts";
import type { ICRUDRepository } from "./ICRUDRepository.ts";
import type { ReadonlyRepositoryMappingOptions } from "./ReadonlyRepositoryMappingOptions.ts";
import type { RepositoryFactory } from "./RepositoryFactory.ts";

export interface IDatasource {
  readonly provider: string;
  readonly repositories: RepositoryFactory;
  close(): Promise<void>;
  [Symbol.asyncDispose](): Promise<void>;
  createConnection(): Promise<IConnection>;
  createCommand(connection?: IConnection): Promise<ICommand>;
  createRepository<T>(
    type: Constructable<T>,
    connection?: IConnection,
    options?: CRUDRepositoryOptions<T>,
  ): ICRUDRepository<T>;
  createRepository(typeName: string, connection?: IConnection): ICRUDRepository;
  createReadonlyRepository<T>(
    type: Constructable<T>,
    connection?: IConnection,
    options?: ReadonlyRepositoryMappingOptions<T>,
  ): IReadonlyRepository<T>;
  createReadonlyRepository(typeName: string, connection?: IConnection): IReadonlyRepository;
}
