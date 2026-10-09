import { CreateEnrichmentStrategy } from "./CreateEnrichmentStrategy.ts";
import { DatasourceError } from "./DatasourceError.ts";
import type { Enrichment } from "./Enrichment.ts";
import type { ICRUDRepository } from "./ICRUDRepository.ts";

export class DefaultCreateEnrichmentStrategy<T> extends CreateEnrichmentStrategy<T> {
  async createChildren<TChild extends object>(writer: ICRUDRepository<TChild>, enrichment: Enrichment, parent: T): Promise<void> {
    for (const related of this.payloadOf<TChild, T>(parent, enrichment.destinationProperty, enrichment)) {
      this.stampCell(related, enrichment.relatedKey, this.getCell(parent, enrichment.sourceProperty));
      await writer.create(related);
    }
  }

  async createLinked<TChild extends object>(writer: ICRUDRepository<TChild>, enrichment: Enrichment, parent: T): Promise<void> {
    const association = enrichment.association;
    if (!association) throw new DatasourceError("Missing association");
    const parentKey = this.requiredKey(parent, enrichment);
    for (const id of this.relatedIds(parent, enrichment))
      await writer.create({ [association.parent]: parentKey, [association.related]: id } as TChild);
  }
}
