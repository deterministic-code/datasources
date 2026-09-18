import type { IConnection } from "../IConnection.ts";
import type { IQuery } from "../IQuery.ts";
import type { ICRUDRepository } from "../ICRUDRepository.ts";
import type { Response } from "../Response.ts";
import type { IWhere } from "../IWhere.ts";
import { notImplemented } from "./notImplemented.ts";

export class StubRepository<T = unknown> implements ICRUDRepository<T> {
  constructor(
    readonly provider: string,
    _connection: IConnection,
  ) {}

  getSingle = async (_query?: IQuery | IWhere): Promise<T | undefined> =>
    notImplemented(this.provider);

  async *get(_query?: IQuery | IWhere): AsyncIterable<T> {
    notImplemented(this.provider);
  }

  create = async (_item: T): Promise<Response> => notImplemented(this.provider);

  update = async (_item: T, _byWhere?: IWhere): Promise<Response> =>
    notImplemented(this.provider);

  delete = async (_item: T, _byWhere?: IWhere): Promise<Response> =>
    notImplemented(this.provider);
}
