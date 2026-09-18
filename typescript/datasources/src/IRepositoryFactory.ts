import type { Constructable } from "./converters/Constructable.ts";
import type { IConnection } from "./IConnection.ts";
import type { IReadonlyRepository } from "./IReadonlyRepository.ts";

export interface IRepositoryFactory {
  register<T>(
    type: Constructable<T>,
    factory: (connection?: IConnection) => IReadonlyRepository<T>,
  ): void;
  register(
    entityTypeName: string,
    factory: (connection?: IConnection) => IReadonlyRepository,
  ): void;
  create<T>(type: Constructable<T>, connection?: IConnection): IReadonlyRepository<T>;
  create(entityTypeName: string, connection?: IConnection): IReadonlyRepository;
}
