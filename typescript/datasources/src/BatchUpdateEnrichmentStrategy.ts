import { DatasourceError } from "./DatasourceError.ts";
import { DefaultUpdateEnrichmentStrategy } from "./DefaultUpdateEnrichmentStrategy.ts";
import { DeleteEnrichmentStrategy } from "./DeleteEnrichmentStrategy.ts";
import { enrichmentMatches, type Enrichment } from "./Enrichment.ts";
import type { ICRUDRepository } from "./ICRUDRepository.ts";
import { UpdateEnrichmentStrategy } from "./UpdateEnrichmentStrategy.ts";

/**
 * Same writes as `DefaultUpdateEnrichmentStrategy`, but the current children or links of all parents
 * are loaded with one batched read instead of one read per parent. Enrichments keyed by several
 * columns load per parent.
 */
export class BatchUpdateEnrichmentStrategy<T> extends UpdateEnrichmentStrategy<T> {
  private readonly single = new DefaultUpdateEnrichmentStrategy<T>();

  updateChildren = <TChild extends object>(writer: ICRUDRepository<TChild>, enrichment: Enrichment, parent: T): Promise<void> =>
    this.single.updateChildren(writer, enrichment, parent);

  updateLinked = <TChild extends object>(writer: ICRUDRepository<TChild>, enrichment: Enrichment, parent: T): Promise<void> =>
    this.single.updateLinked(writer, enrichment, parent);

  override async updateChildrenBatch<TChild extends object>(
    writer: ICRUDRepository<TChild>,
    enrichment: Enrichment,
    parents: T[],
  ): Promise<void> {
    const matches = enrichmentMatches(enrichment);
    if (parents.length < 2 || matches.length !== 1) return super.updateChildrenBatch(writer, enrichment, parents);
    const existing = await DeleteEnrichmentStrategy.loadGrouped(writer, matches[0]!.relatedKey, matches[0]!.sourceProperty, parents);
    for (const [index, parent] of parents.entries())
      await this.single.updateChildrenWith(writer, enrichment, parent, existing[index]!);
  }

  override async updateLinkedBatch<TChild extends object>(
    writer: ICRUDRepository<TChild>,
    enrichment: Enrichment,
    parents: T[],
  ): Promise<void> {
    const association = enrichment.association;
    if (!association) throw new DatasourceError("Missing association");
    if (parents.length < 2) return super.updateLinkedBatch(writer, enrichment, parents);
    const existing = await DeleteEnrichmentStrategy.loadGrouped(writer, association.parent, enrichment.sourceProperty, parents);
    for (const [index, parent] of parents.entries())
      await this.single.updateLinkedWith(writer, enrichment, parent, existing[index]!);
  }
}
