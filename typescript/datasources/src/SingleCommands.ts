import type { BulkCRUDRepository } from "./BulkCRUDRepository.ts";
import type { Response } from "./Response.ts";
import type { IWhere } from "./statements/IWhere.ts";
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
  create = async (items: T[], repo: BulkCRUDRepository<T>): Promise<Response> => {
    let result: Response["result"] = null;
    for (const item of items) {
      const id = await repo.createOne(item);
      if (id !== undefined) result = id;
    }
    return { result, rowsAffected: items.length };
  };

  update = (items: T[], byWhere: IWhere | undefined, repo: BulkCRUDRepository<T>): Promise<Response> =>
    fold(items, (item) => repo.updateOne(item, byWhere));

  delete = (items: T[], byWhere: IWhere | undefined, repo: BulkCRUDRepository<T>): Promise<Response> =>
    fold(items, (item) => repo.deleteOne(item, byWhere));
}
