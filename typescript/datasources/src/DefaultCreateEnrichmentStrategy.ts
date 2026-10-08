import { CreateEnrichmentStrategy } from "./CreateEnrichmentStrategy.ts";
import { DatasourceError } from "./DatasourceError.ts";
import type { Enrichment } from "./Enrichment.ts";
import type { ICRUDRepository } from "./ICRUDRepository.ts";

export class DefaultCreateEnrichmentStrategy<T> extends CreateEnrichmentStrategy<T> {
  async createOwned(writer: ICRUDRepository<object>, enrichment: Enrichment, parent: object): Promise<void> {
    for (const related of this.payloadOf(parent, enrichment.destinationProperty, enrichment)) {
      this.stampCell(related, enrichment.relatedKey, this.getCell(parent, enrichment.sourceProperty));
      await writer.create(related);
    }
  }

  async createLinked(writer: ICRUDRepository<object>, enrichment: Enrichment, parent: object): Promise<void> {
    const association = enrichment.association;
    if (!association) throw new DatasourceError("Missing association");
    const parentKey = this.requiredKey(parent, enrichment);
    for (const id of this.relatedIds(parent, enrichment))
      await writer.create({ [association.parent]: parentKey, [association.related]: id });
  }
}
