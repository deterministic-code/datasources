import type { Constructable } from "./converters/Constructable.ts";
import type { IQuery } from "./IQuery.ts";
import type { IReadonlyRepository } from "./IReadonlyRepository.ts";
import type { IWhere } from "./IWhere.ts";

export const untyped = <T>(): Constructable<T> => Object as never as Constructable<T>;

export abstract class ReadonlyRepository<T> implements IReadonlyRepository<T> {
  constructor(readonly type: Constructable<T>) {}

  abstract get(query?: IQuery | IWhere): AsyncIterable<T>;

  async getSingle(query?: IQuery | IWhere): Promise<T | undefined> {
    for await (const item of this.get(query)) return item;
    return undefined;
  }
}
