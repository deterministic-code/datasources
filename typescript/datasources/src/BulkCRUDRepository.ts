import { DeepCRUDRepository } from "./DeepCRUDRepository.ts";
import type { IRepository } from "./IRepository.ts";
import type { IResponse } from "./IResponse.ts";
import type { IWhere } from "./IWhere.ts";

const fold = async <T>(
  items: T[],
  write: (item: T) => Promise<IResponse>,
): Promise<IResponse> => {
  let result: IResponse["result"] = null;
  let rowsAffected = 0;
  for (const item of items) {
    const response = await write(item);
    result = response.result;
    rowsAffected += response.rowsAffected;
  }
  return { result, rowsAffected };
};

export class BulkCRUDRepository<T extends object> extends DeepCRUDRepository<T> {
  constructor(crud: IRepository<T>) {
    super(crud);
  }

  addBulk = (items: T[]): Promise<IResponse> =>
    fold(items, (item) => this.add(item));

  updateBulk = (items: T[], byWhere?: IWhere): Promise<IResponse> =>
    fold(items, (item) => this.update(item, byWhere));

  deleteBulk = (items: T[], byWhere?: IWhere): Promise<IResponse> =>
    fold(items, (item) => this.delete(item, byWhere));
}
