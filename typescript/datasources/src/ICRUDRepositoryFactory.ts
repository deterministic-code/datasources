import type { Constructable } from "./converters/Constructable.ts";
import type { ICRUDRepository } from "./ICRUDRepository.ts";
import type { IRepository } from "./IRepository.ts";

export interface ICRUDRepositoryFactory {
  register(repository: IRepository): void;
  create<T>(type: Constructable<T>): ICRUDRepository<T>;
}
