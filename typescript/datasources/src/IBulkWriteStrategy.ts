import type { CRUDRepository } from "./CRUDRepository.ts";
import type { CRUDRepositoryOptions } from "./CRUDRepositoryOptions.ts";
import type { ICommand } from "./ICommand.ts";
import type { Response } from "./Response.ts";
import type { IWhere } from "./statements/IWhere.ts";

export interface IBulkWriteStrategy<T extends object> {
  repo: CRUDRepository<T>;
  attach(command: ICommand, options: CRUDRepositoryOptions<T>): void;
  create(items: T[]): Promise<Response>;
  update(items: T[], byWhere: IWhere | undefined): Promise<Response>;
  delete(items: T[], byWhere: IWhere | undefined): Promise<Response>;
}
