import type { CRUDRepository } from "./CRUDRepository.ts";
import type { Response } from "./Response.ts";
import type { IWhere } from "./statements/IWhere.ts";

export interface IBulkWriteStrategy<T extends object> {
  repo: CRUDRepository<T>;
  create(items: T[]): Promise<Response>;
  update(items: T[], byWhere: IWhere | undefined): Promise<Response>;
  delete(items: T[], byWhere: IWhere | undefined): Promise<Response>;
}
