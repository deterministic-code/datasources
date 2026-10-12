import type { Constructable } from "./converters/Constructable.ts";
import type { IConnection } from "./IConnection.ts";
import type { IReadonlyRepository } from "./IReadonlyRepository.ts";
import type { IRepository } from "./IRepository.ts";
import type { IRepositoryFactory } from "./IRepositoryFactory.ts";
import type { ICRUDRepository } from "./ICRUDRepository.ts";
import type { ICRUDRepositoryFactory } from "./ICRUDRepositoryFactory.ts";
import { RepositoryAccess } from "./RepositoryAccess.ts";
import { RepositoryNotRegisteredError } from "./RepositoryNotRegisteredError.ts";

type Named = {
  create: (connection: IConnection) => ICRUDRepository;
  createReadonly: (connection: IConnection) => IReadonlyRepository;
};

const binders: Array<(factory: RepositoryFactory) => void> = [];

export class RepositoryFactory implements ICRUDRepositoryFactory {
  readonly #readonlyByType = new Map<Constructable<unknown>, IRepository>();
  readonly #crudByType = new Map<Constructable<unknown>, IRepository>();
  readonly #byName = new Map<string, Named>();

  constructor() {
    this.applyBound();
  }

  static bind = (register: (factory: RepositoryFactory) => void): void => {
    binders.push(register);
  };

  static resetBinders = (): void => {
    binders.length = 0;
  };

  private applyBound(): void {
    for (const register of binders) register(this);
  }

  register(repository: IRepository): void;
  register(
    typeName: string,
    pluralName: string,
    create: (connection: IConnection) => ICRUDRepository,
    createReadonly: (connection: IConnection) => IReadonlyRepository,
  ): void;
  register(
    repositoryOrType: IRepository | string,
    pluralName?: string,
    create?: (connection: IConnection) => ICRUDRepository,
    createReadonly?: (connection: IConnection) => IReadonlyRepository,
  ): void {
    if (typeof repositoryOrType !== "string") {
      this.#put(repositoryOrType, true);
      return;
    }
    const named = { create: create as Named["create"], createReadonly: createReadonly as Named["createReadonly"] };
    this.#byName.set(repositoryOrType, named);
    this.#byName.set(pluralName as string, named);
  }

  create<T>(type: Constructable<T>): ICRUDRepository<T>;
  create(name: string, connection: IConnection): ICRUDRepository;
  create(
    typeOrName: Constructable<unknown> | string,
    connection?: IConnection,
  ): ICRUDRepository {
    if (typeof typeOrName !== "string")
      return this.#get(this.#crudByType, typeOrName) as ICRUDRepository;
    this.applyBound();
    return this.#lookup(typeOrName).create(connection as IConnection);
  }

  createReadonly = (name: string, connection: IConnection): IReadonlyRepository => {
    this.applyBound();
    return this.#lookup(name).createReadonly(connection);
  };

  access = (name: string, connection: IConnection): RepositoryAccess => {
    this.applyBound();
    const named = this.#lookup(name);
    return new RepositoryAccess(named.create, named.createReadonly, connection);
  };

  has = (name: string): boolean => {
    this.applyBound();
    return this.#byName.has(name);
  };

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

  #lookup = (name: string): Named => {
    const named = this.#byName.get(name);
    if (!named) throw new RepositoryNotRegisteredError(name);
    return named;
  };
}
