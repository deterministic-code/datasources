import { CreateEnrichmentStrategy } from "./CreateEnrichmentStrategy.ts";
import { DatasourceError } from "./DatasourceError.ts";
import { enrichmentMatches, type Enrichment } from "./Enrichment.ts";
import type { ICRUDRepository } from "./ICRUDRepository.ts";

export class DefaultCreateEnrichmentStrategy<T> extends CreateEnrichmentStrategy<T> {
  async createChildren<TChild extends object>(writer: ICRUDRepository<TChild>, enrichment: Enrichment, parent: T): Promise<void> {
    const children = this.payloadOf<TChild, T>(parent, enrichment.destinationProperty, enrichment, writer);
    if (children.length === 0) return;
    for (const match of enrichmentMatches(enrichment)) {
      const key = this.requiredValue(parent, match.sourceProperty, enrichment.destinationProperty);
      for (const related of children) this.stampCell(related, match.relatedKey, key);
    }
    for (const related of children) await writer.create(related);
  }

  async createLinked<TChild extends object>(writer: ICRUDRepository<TChild>, enrichment: Enrichment, parent: T): Promise<void> {
    const association = enrichment.association;
    if (!association) throw new DatasourceError("Missing association");
    const parentKey = this.requiredKey(parent, enrichment);
    for (const id of this.relatedIds(parent, enrichment))
      await writer.create({ [association.parent]: parentKey, [association.related]: id } as TChild);
  }
}
