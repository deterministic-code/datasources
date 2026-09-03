import type { Constructable } from "./converters/Constructable.ts";
import type { IConnection } from "./IConnection.ts";
import type { IReadonlyRepository } from "./IReadonlyRepository.ts";
import type { IReadonlyRepositoryFactory } from "./IReadonlyRepositoryFactory.ts";
import type { IRepository } from "./IRepository.ts";
import type { IRepositoryFactory } from "./IRepositoryFactory.ts";
import { RepositoryNotRegisteredError } from "./RepositoryNotRegisteredError.ts";

type CreateFn = (connection?: IConnection) => unknown;

const nameOf = (typeOrName: Constructable<unknown> | string) =>
  typeof typeOrName === "string" ? typeOrName : typeOrName.name;

export class RepositoryFactory implements IRepositoryFactory {
  readonly #readonlyByName = new Map<string, CreateFn>();
  readonly #readonlyByType = new Map<Constructable<unknown>, CreateFn>();
  readonly #crudByName = new Map<string, CreateFn>();
  readonly #crudByType = new Map<Constructable<unknown>, CreateFn>();

  register = (
    typeOrName: Constructable<unknown> | string,
    factory: CreateFn,
  ): void => this.#put(typeOrName, factory, true);

  registerReadonly = (
    typeOrName: Constructable<unknown> | string,
    factory: CreateFn,
  ): void => this.#put(typeOrName, factory, false);

  create = (
    typeOrName: Constructable<unknown> | string,
    connection?: IConnection,
  ): IRepository =>
    this.#invoke(this.#lookup(this.#crudByType, this.#crudByName, typeOrName), connection) as IRepository;

  createReadonly = (
    typeOrName: Constructable<unknown> | string,
    connection?: IConnection,
  ): IReadonlyRepository =>
    this.#invoke(
      this.#lookup(this.#readonlyByType, this.#readonlyByName, typeOrName),
      connection,
    ) as IReadonlyRepository;

  readonly asReadonly: IReadonlyRepositoryFactory = {
    register: this.registerReadonly,
    create: this.createReadonly,
  };

  #put = (
    typeOrName: Constructable<unknown> | string,
    factory: CreateFn,
    crud: boolean,
  ): void => {
    const name = nameOf(typeOrName);
    this.#readonlyByName.set(name, factory);
    if (typeof typeOrName !== "string") this.#readonlyByType.set(typeOrName, factory);
    if (!crud) return;
    this.#crudByName.set(name, factory);
    if (typeof typeOrName !== "string") this.#crudByType.set(typeOrName, factory);
  };

  #lookup = (
    byType: Map<Constructable<unknown>, CreateFn>,
    byName: Map<string, CreateFn>,
    typeOrName: Constructable<unknown> | string,
  ): CreateFn => {
    const factory =
      typeof typeOrName === "string"
        ? byName.get(typeOrName)
        : (byType.get(typeOrName) ?? byName.get(typeOrName.name));
    if (!factory) throw new RepositoryNotRegisteredError(nameOf(typeOrName));
    return factory;
  };

  #invoke = (factory: CreateFn, connection?: IConnection): unknown => factory(connection);
}
