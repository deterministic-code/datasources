import type { IConnection } from "../IConnection.ts";
import type { IQuery } from "../IQuery.ts";
import type { IRepository } from "../IRepository.ts";
import type { IResponse } from "../IResponse.ts";
import type { IWhere } from "../IWhere.ts";
import { notImplemented } from "./notImplemented.ts";

export class StubRepository<T = unknown> implements IRepository<T> {
  constructor(
    readonly provider: string,
    readonly connection: IConnection,
  ) {}

  async getSingle(_query?: IQuery): Promise<T | undefined> {
    notImplemented(this.provider);
  }

  async *get(_query?: IQuery): AsyncIterable<T> {
    notImplemented(this.provider);
  }

  async add(_obj: T): Promise<IResponse> {
    notImplemented(this.provider);
  }

  async update(_obj: T, _byWhere?: IWhere): Promise<IResponse> {
    notImplemented(this.provider);
  }

  async delete(_obj: T, _byWhere?: IWhere): Promise<IResponse> {
    notImplemented(this.provider);
  }
}
