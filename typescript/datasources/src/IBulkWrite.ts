import type { Cells } from "./converters/Cell.ts";
import type { ICommand } from "./ICommand.ts";
import type { Response } from "./Response.ts";
import type { Scalar } from "./builders/Scalar.ts";
import type { IWhere } from "./statements/IWhere.ts";
import type { BulkCommand } from "./BulkCommand.ts";

export interface IBulkWrite<T> {
  readonly command: ICommand;
  createOne(item: T): Promise<Scalar | undefined>;
  updateOne(item: T, byWhere?: IWhere): Promise<Response>;
  deleteOne(item: T, byWhere?: IWhere): Promise<Response>;
  inserts(items: T[]): BulkCommand[];
  updates(items: T[], byWhere?: IWhere): BulkCommand[];
  deletes(items: T[], byWhere?: IWhere): BulkCommand[];
  rows(items: T[]): Cells[];
}
