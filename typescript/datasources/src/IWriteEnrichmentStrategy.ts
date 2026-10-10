import type { IRepositoryTree } from "./IRepositoryTree.ts";
import type { WriteMode } from "./WriteMode.ts";

export interface IWriteEnrichmentStrategy<T> {
  readonly useTransaction: boolean;
  persist(tree: IRepositoryTree, items: T[], mode: WriteMode): Promise<void>;
  persistBulk?(tree: IRepositoryTree, items: T[], mode: WriteMode): Promise<void>;
}
