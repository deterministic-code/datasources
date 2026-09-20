import type { Constructable } from "./converters/Constructable.ts";
import type { IReadonlyRepository } from "./IReadonlyRepository.ts";
import type { IRepository } from "./IRepository.ts";
import type { IRepositoryFactory } from "./IRepositoryFactory.ts";
import type { ICRUDRepository } from "./ICRUDRepository.ts";
import type { ICRUDRepositoryFactory } from "./ICRUDRepositoryFactory.ts";
import { RepositoryNotRegisteredError } from "./RepositoryNotRegisteredError.ts";

export class RepositoryFactory implements ICRUDRepositoryFactory {
  readonly #readonlyByType = new Map<Constructable<unknown>, IRepository>();
  readonly #crudByType = new Map<Constructable<unknown>, IRepository>();

  register = (repository: IRepository): void => this.#put(repository, true);

  create = <T>(type: Constructable<T>): ICRUDRepository<T> =>
    this.#get(this.#crudByType, type) as ICRUDRepository<T>;

  readonly asReadonly: IRepositoryFactory = {
    register: (repository) => this.#put(repository, false),
    create: <T>(type: Constructable<T>) =>
      this.#get(this.#readonlyByType, type) as IReadonlyRepository<T>,
  };

  #put = (repository: IRepository, crud: boolean): void => {
    this.#readonlyByType.set(repository.type, repository);
    if (!crud) return;
    this.#crudByType.set(repository.type, repository);
  };

  #get = (
    map: Map<Constructable<unknown>, IRepository>,
    type: Constructable<unknown>,
  ): IRepository => {
    const repository = map.get(type);
    if (!repository) throw new RepositoryNotRegisteredError(type.name);
    return repository;
  };
}
