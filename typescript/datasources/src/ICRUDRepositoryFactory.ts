import type { Constructable } from "./converters/Constructable.ts";
import type { IConnection } from "./IConnection.ts";
import type { ICRUDRepository } from "./ICRUDRepository.ts";

export interface ICRUDRepositoryFactory {
  register<T>(
    type: Constructable<T>,
    factory: (connection?: IConnection) => ICRUDRepository<T>,
  ): void;
  register(
    entityTypeName: string,
    factory: (connection?: IConnection) => ICRUDRepository,
  ): void;
  create<T>(type: Constructable<T>, connection?: IConnection): ICRUDRepository<T>;
  create(entityTypeName: string, connection?: IConnection): ICRUDRepository;
}
