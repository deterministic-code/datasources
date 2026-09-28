import type { BulkCRUDRepository } from "./BulkCRUDRepository.ts";
import type { Response } from "./Response.ts";
import type { IWhere } from "./statements/IWhere.ts";

export interface IBulkWriteStrategy<T extends object> {
  create(items: T[], repo: BulkCRUDRepository<T>): Promise<Response>;
  update(items: T[], byWhere: IWhere | undefined, repo: BulkCRUDRepository<T>): Promise<Response>;
  delete(items: T[], byWhere: IWhere | undefined, repo: BulkCRUDRepository<T>): Promise<Response>;
}
