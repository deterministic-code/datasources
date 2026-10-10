import type { CRUDRepository } from "./CRUDRepository.ts";
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

export class SingleCommandIBulkWriteStrategy<T extends object> implements IBulkWriteStrategy<T> {
  repo!: CRUDRepository<T>;

  attach = (): void => {};

  create = async (items: T[]): Promise<Response> => {
    for (const item of items) await this.repo.create(item);
    return { result: null, rowsAffected: items.length };
  };

  update = (items: T[], byWhere: IWhere | undefined): Promise<Response> =>
    fold(items, (item) => this.repo.update(item, byWhere));

  delete = (items: T[], byWhere: IWhere | undefined): Promise<Response> =>
    fold(items, (item) => this.repo.delete(item, byWhere));
}
