import type { Response } from "./Response.ts";
import type { IWhere } from "./statements/IWhere.ts";
import type { IBulkWrite } from "./IBulkWrite.ts";

export interface IBulkWriteStrategy<T> {
  create(items: T[], write: IBulkWrite<T>): Promise<Response>;
  update(items: T[], byWhere: IWhere | undefined, write: IBulkWrite<T>): Promise<Response>;
  delete(items: T[], byWhere: IWhere | undefined, write: IBulkWrite<T>): Promise<Response>;
}
