import type { Constructable } from "./converters/Constructable.ts";
import type { IConnection } from "./IConnection.ts";
import type { IRepository } from "./IRepository.ts";

export interface IRepositoryFactory {
  register<T>(
    type: Constructable<T>,
    factory: (connection?: IConnection) => IRepository<T>,
  ): void;
  register(
    entityTypeName: string,
    factory: (connection?: IConnection) => IRepository,
  ): void;
  create<T>(type: Constructable<T>, connection?: IConnection): IRepository<T>;
  create(entityTypeName: string, connection?: IConnection): IRepository;
}
