import { CRUDRepository } from "./CRUDRepository.ts";
import type { ICRUDRepository } from "./ICRUDRepository.ts";
import type { Response } from "./Response.ts";
import type { IWhere } from "./statements/IWhere.ts";

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

export class BulkCRUDRepository<T extends object> extends CRUDRepository<T> {
  constructor(crud: ICRUDRepository<T>) {
    super(crud);
  }

  createBulk = (items: T[]): Promise<Response> =>
    fold(items, (item) => this.create(item));

  updateBulk = (items: T[], byWhere?: IWhere): Promise<Response> =>
    fold(items, (item) => this.update(item, byWhere));

  deleteBulk = (items: T[], byWhere?: IWhere): Promise<Response> =>
    fold(items, (item) => this.delete(item, byWhere));
}
