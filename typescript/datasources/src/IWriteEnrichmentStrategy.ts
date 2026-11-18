import type { Cells } from "./converters/Value.ts";
import type { TypeConverter } from "./converters/TypeConverter.ts";
import type { IRepositoryTree } from "./IRepositoryTree.ts";
import type { Schema } from "./Schema.ts";
import type { WriteMode } from "./WriteMode.ts";

export interface IWriteEnrichmentStrategy<T> {
  readonly useTransaction: boolean;
  persist(tree: IRepositoryTree, items: T[], mode: WriteMode): Promise<void>;
  persistBulk?(tree: IRepositoryTree, items: T[], mode: WriteMode): Promise<void>;
  resolveLookups?(tree: IRepositoryTree, items: T[], record: Cells, schema: Schema): Promise<void>;
  lookup?(
    tree: IRepositoryTree,
    items: T[],
    record: Cells,
    schema: Schema,
    column: (property: string) => string,
  ): Promise<void>;
  lookupRow?(
    tree: IRepositoryTree,
    item: T,
    schema: Schema,
    column: (property: string) => string,
    objects: TypeConverter<T, Cells>,
    rows?: TypeConverter<Cells, Cells>,
  ): Promise<Cells>;
  sources?(tree: IRepositoryTree, column: (property: string) => string): Set<string>;
  destination?(tree: IRepositoryTree, name: string): boolean;
}

export async function lookupRow<T>(
  write: IWriteEnrichmentStrategy<T>,
  tree: IRepositoryTree,
  item: T,
  schema: Schema,
  column: (property: string) => string,
  objects: TypeConverter<T, Cells>,
  rows?: TypeConverter<Cells, Cells>,
): Promise<Cells> {
  if (write.lookupRow) return write.lookupRow(tree, item, schema, column, objects, rows);
  const record = objects.reverseConvert(item);
  if (write.lookup) await write.lookup(tree, [item], record, schema, column);
  else await write.resolveLookups?.(tree, [item], record, schema);
  return rows ? rows.reverseConvert(record) : record;
}
