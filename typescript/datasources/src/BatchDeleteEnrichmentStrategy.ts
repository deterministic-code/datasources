import { CascadeDeleteEnrichmentStrategy } from "./CascadeDeleteEnrichmentStrategy.ts";
import { DatasourceError } from "./DatasourceError.ts";
import { DeleteEnrichmentStrategy } from "./DeleteEnrichmentStrategy.ts";
import { enrichmentMatches, type Enrichment } from "./Enrichment.ts";
import type { ICRUDRepository } from "./ICRUDRepository.ts";

/**
 * Same deletes as `CascadeDeleteEnrichmentStrategy`, but the children or links of all parents are
 * loaded with one batched read, and the nested cascade runs once over all their rows. Enrichments
 * keyed by several columns delete per parent.
 */
export class BatchDeleteEnrichmentStrategy<T> extends DeleteEnrichmentStrategy<T> {
  private readonly single = new CascadeDeleteEnrichmentStrategy<T>();

  deleteChildren = <TChild extends object>(
    writer: ICRUDRepository<TChild>,
    enrichment: Enrichment,
    parent: T,
    deleteNested: (rows: TChild[]) => Promise<void>,
  ): Promise<void> => this.single.deleteChildren(writer, enrichment, parent, deleteNested);

  deleteLinked = <TChild extends object>(writer: ICRUDRepository<TChild>, enrichment: Enrichment, parent: T): Promise<void> =>
    this.single.deleteLinked(writer, enrichment, parent);

  override async deleteChildrenBatch<TChild extends object>(
    writer: ICRUDRepository<TChild>,
    enrichment: Enrichment,
    parents: T[],
    deleteNested: (rows: TChild[]) => Promise<void>,
  ): Promise<void> {
    const matches = enrichmentMatches(enrichment);
    if (parents.length < 2 || matches.length !== 1) return super.deleteChildrenBatch(writer, enrichment, parents, deleteNested);
    const existing = await DeleteEnrichmentStrategy.loadGrouped(writer, matches[0]!.relatedKey, matches[0]!.sourceProperty, parents);
    await deleteNested(existing.flat());
    for (const [index, parent] of parents.entries())
      await this.single.deleteExisting(writer, enrichment, parent, existing[index]!);
  }

  override async deleteLinkedBatch<TChild extends object>(
    writer: ICRUDRepository<TChild>,
    enrichment: Enrichment,
    parents: T[],
  ): Promise<void> {
    const association = enrichment.association;
    if (!association) throw new DatasourceError("Missing association");
    if (parents.length < 2) return super.deleteLinkedBatch(writer, enrichment, parents);
    const existing = await DeleteEnrichmentStrategy.loadGrouped(writer, association.parent, enrichment.sourceProperty, parents);
    for (const [index, parent] of parents.entries())
      await this.single.deleteLinkedWith(writer, enrichment, parent, existing[index]!);
  }
}
