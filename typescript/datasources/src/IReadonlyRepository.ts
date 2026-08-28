import type { IConnection } from "./IConnection.ts";
import type { IQuery } from "./IQuery.ts";

export interface IReadonlyRepository<T = unknown> {
  readonly connection: IConnection;
  getSingle(query?: IQuery): Promise<T | undefined>;
  get(query?: IQuery): AsyncIterable<T>;
}
