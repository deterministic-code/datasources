import type { Response } from "./Response.ts";
import type { IWhere } from "./statements/IWhere.ts";
import type { IBulkWrite } from "./IBulkWrite.ts";
import type { IBulkWriteStrategy } from "./IBulkWriteStrategy.ts";

const fold = async <T>(
  items: T[],
  write: (item: T) => Promise<Response>,
): Promise<Response> => {
  let result: Response["result"] = null;
  let rowsAffected = 0;
  for (const item of items) {
    const response = await write(item);
    result = response.result;
    rowsAffected += response.rowsAffected;
  }
  return { result, rowsAffected };
};

export class SingleCommands<T extends object> implements IBulkWriteStrategy<T> {
  create = async (items: T[], write: IBulkWrite<T>): Promise<Response> => {
    let result: Response["result"] = null;
    for (const item of items) {
      const id = await write.createOne(item);
      if (id !== undefined) result = id;
    }
    return { result, rowsAffected: items.length };
  };

  update = (items: T[], byWhere: IWhere | undefined, write: IBulkWrite<T>): Promise<Response> =>
    fold(items, (item) => write.updateOne(item, byWhere));

  delete = (items: T[], byWhere: IWhere | undefined, write: IBulkWrite<T>): Promise<Response> =>
    fold(items, (item) => write.deleteOne(item, byWhere));
}
