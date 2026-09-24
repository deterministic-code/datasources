import type { IRepositoryTree } from "./IRepositoryTree.ts";
import type { WriteMode } from "./WriteMode.ts";

export interface IWriteEnrichmentStrategy<T> {
  persist(tree: IRepositoryTree, items: T[], mode: WriteMode): Promise<void>;
}
