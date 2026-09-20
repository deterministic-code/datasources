import type { IQuery } from "./IQuery.ts";
import type { IReadonlyEnrichmentBinder } from "./IReadonlyEnrichmentBinder.ts";
import type { IReadonlyRepository } from "./IReadonlyRepository.ts";
import type { IRepositoryTree } from "./IRepositoryTree.ts";
import type { IWhere } from "./IWhere.ts";

export interface IReadonlyEnrichmentStrategy<T extends object>
  extends IReadonlyEnrichmentBinder {
  read(
    inner: IReadonlyRepository<T>,
    tree: IRepositoryTree,
    query?: IQuery | IWhere,
  ): AsyncIterable<T>;
  enrich(tree: IRepositoryTree, items: T[]): Promise<void>;
}
