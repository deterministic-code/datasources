import { DatasourceError } from "./DatasourceError.ts";
import { DeleteEnrichmentStrategy } from "./DeleteEnrichmentStrategy.ts";
import type { Enrichment } from "./Enrichment.ts";
import type { ICRUDRepository } from "./ICRUDRepository.ts";
import type { Scalar } from "./builders/Scalar.ts";

type Props = Record<string, Scalar | object>;

const get = <TItem>(item: TItem, name: string): Scalar | object | undefined =>
  (item as never as Props)[name];

export class CascadeDeleteEnrichmentStrategy<T> extends DeleteEnrichmentStrategy<T> {
  async deleteChildren<TChild extends object>(
    writer: ICRUDRepository<TChild>,
    enrichment: Enrichment,
    parent: T,
    deleteNested: (rows: TChild[]) => Promise<void>,
  ): Promise<void> {
    const existing = await DeleteEnrichmentStrategy.load(writer, enrichment, parent);
    await deleteNested(existing);
    for (const row of existing) await writer.delete(row, DeleteEnrichmentStrategy.childrenWhere(writer, enrichment, parent));
  }

  async deleteLinked<TChild extends object>(writer: ICRUDRepository<TChild>, enrichment: Enrichment, parent: T): Promise<void> {
    const association = enrichment.association;
    if (!association) throw new DatasourceError("Missing association");
    const key = get(parent, enrichment.sourceProperty);
    if (key === undefined || key === null || typeof key === "object") return;
    for (const row of await DeleteEnrichmentStrategy.loadLinks(writer, association.parent, key))
      await writer.delete(
        row,
        DeleteEnrichmentStrategy.linkWhere(association, key, DeleteEnrichmentStrategy.linkKey(get(row, association.related))),
      );
  }
}
