import type { Cells } from "./converters/Value.ts";
import type { IRepositoryTree } from "./IRepositoryTree.ts";
import type { Schema } from "./Schema.ts";
import type { WriteMode } from "./WriteMode.ts";

export interface IWriteEnrichmentStrategy<T> {
  readonly useTransaction: boolean;
  persist(tree: IRepositoryTree, items: T[], mode: WriteMode): Promise<void>;
  persistBulk?(tree: IRepositoryTree, items: T[], mode: WriteMode): Promise<void>;
  resolveLookups?(tree: IRepositoryTree, items: T[], record: Cells, schema: Schema): Promise<void>;
}
