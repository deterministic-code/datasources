import type { Constructable } from "./converters/Constructable.ts";
import type { Cells } from "./converters/Value.ts";
import type { IConnection } from "./IConnection.ts";
import type { IReadonlyRepository } from "./IReadonlyRepository.ts";
import type { IRepository } from "./IRepository.ts";
import type { IRepositoryFactory } from "./IRepositoryFactory.ts";
import type { ICRUDRepository } from "./ICRUDRepository.ts";
import type { ICRUDRepositoryFactory } from "./ICRUDRepositoryFactory.ts";
import { RepositoryAccess } from "./RepositoryAccess.ts";
import { RepositoryNotRegisteredError } from "./RepositoryNotRegisteredError.ts";

type Named = {
  create: <T>(connection: IConnection) => IReadonlyRepository<T>;
  createReadonly: <T>(connection: IConnection) => IReadonlyRepository<T>;
};

const binders: Array<(factory: RepositoryFactory) => void> = [];

export class RepositoryFactory implements ICRUDRepositoryFactory {
  readonly #readonlyByType = new Map<Constructable<Cells>, IRepository<Cells>>();
  readonly #crudByType = new Map<Constructable<Cells>, IRepository<Cells>>();
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

  register<T>(repository: IRepository<T>): void;
  register<T>(
    typeName: string,
    pluralName: string,
    create: (connection: IConnection) => IReadonlyRepository<T>,
    createReadonly: (connection: IConnection) => IReadonlyRepository<T>,
  ): void;
  register<T>(
    repositoryOrType: IRepository<T> | string,
    pluralName?: string,
    create?: (connection: IConnection) => IReadonlyRepository<T>,
    createReadonly?: (connection: IConnection) => IReadonlyRepository<T>,
  ): void {
    if (typeof repositoryOrType !== "string") {
      this.#put(repositoryOrType);
      return;
    }
    const named = { create: create as Named["create"], createReadonly: createReadonly as Named["createReadonly"] };
    this.#byName.set(repositoryOrType, named);
    this.#byName.set(pluralName as string, named);
  }

  create<T>(type: Constructable<T>): ICRUDRepository<T>;
  create<T>(name: string, connection: IConnection): ICRUDRepository<T>;
  create<T>(
    typeOrName: Constructable<T> | string,
    connection?: IConnection,
  ): ICRUDRepository<T> {
    if (typeof typeOrName !== "string")
      return this.#get(this.#crudByType, typeOrName) as ICRUDRepository<T>;
    this.applyBound();
    return this.#lookup(typeOrName).create(connection as IConnection) as ICRUDRepository<T>;
  }

  createReadonly = <T>(name: string, connection: IConnection): IReadonlyRepository<T> => {
    this.applyBound();
    return this.#lookup(name).createReadonly(connection);
  };

  access = <T>(name: string, connection: IConnection): RepositoryAccess<T> => {
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

  #put = <T>(repository: IRepository<T>, crud = true): void => {
    const type = repository.type as Constructable<Cells>;
    const stored = repository as IRepository<Cells>;
    this.#readonlyByType.set(type, stored);
    if (!crud) return;
    this.#crudByType.set(type, stored);
  };

  #get = (
    map: Map<Constructable<Cells>, IRepository<Cells>>,
    type: Constructable<Cells> | Constructable,
  ): IRepository<Cells> => {
    const repository = map.get(type as Constructable<Cells>);
    if (!repository) throw new RepositoryNotRegisteredError((type as Constructable<Cells>).name);
    return repository;
  };

  #lookup = (name: string): Named => {
    const named = this.#byName.get(name);
    if (!named) throw new RepositoryNotRegisteredError(name);
    return named;
  };
}
