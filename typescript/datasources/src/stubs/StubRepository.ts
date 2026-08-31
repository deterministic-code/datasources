import type { IConnection } from "../IConnection.ts";
import type { IQuery } from "../IQuery.ts";
import type { IRepository } from "../IRepository.ts";
import type { IResponse } from "../IResponse.ts";
import type { IWhere } from "../IWhere.ts";
import { notImplemented } from "./notImplemented.ts";

export class StubRepository<T = unknown> implements IRepository<T> {
  constructor(
    readonly provider: string,
    _connection: IConnection,
  ) {}

  getSingle = async (_query?: IQuery | IWhere): Promise<T | undefined> =>
    notImplemented(this.provider);

  async *get(_query?: IQuery | IWhere): AsyncIterable<T> {
    notImplemented(this.provider);
  }

  add = async (_obj: T): Promise<IResponse> => notImplemented(this.provider);

  update = async (_obj: T, _byWhere?: IWhere): Promise<IResponse> =>
    notImplemented(this.provider);

  delete = async (_obj: T, _byWhere?: IWhere): Promise<IResponse> =>
    notImplemented(this.provider);
}
