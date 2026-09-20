import type { Constructable } from "./converters/Constructable.ts";
import type { IReadonlyRepository } from "./IReadonlyRepository.ts";
import type { IRepository } from "./IRepository.ts";

export interface IRepositoryFactory {
  register(repository: IRepository): void;
  create<T>(type: Constructable<T>): IReadonlyRepository<T>;
}
