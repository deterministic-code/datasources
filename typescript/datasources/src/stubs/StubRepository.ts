import type { Constructable } from "../converters/Constructable.ts";
import type { IConnection } from "../IConnection.ts";
import type { IQuery } from "../statements/IQuery.ts";
import type { ICRUDRepository } from "../ICRUDRepository.ts";
import { ReadonlyRepository, untyped } from "../ReadonlyRepository.ts";
import type { Response } from "../Response.ts";
import type { IWhere } from "../statements/IWhere.ts";
import { notImplemented } from "./notImplemented.ts";

export class StubRepository<T = unknown> extends ReadonlyRepository<T> implements ICRUDRepository<T> {
  constructor(
    readonly provider: string,
    _connection: IConnection,
    type: Constructable<T> = untyped<T>(),
  ) {
    super(type);
  }

  async *read(_query?: IQuery | IWhere): AsyncIterable<T> {
    notImplemented(this.provider);
  }

  create = async (_item: T): Promise<Response> => notImplemented(this.provider);

  update = async (_item: T, _byWhere?: IWhere): Promise<Response> =>
    notImplemented(this.provider);

  delete = async (_item: T, _byWhere?: IWhere): Promise<Response> =>
    notImplemented(this.provider);
}
