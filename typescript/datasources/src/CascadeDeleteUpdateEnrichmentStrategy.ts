import { DefaultCreateEnrichmentStrategy } from "./DefaultCreateEnrichmentStrategy.ts";
import { DatasourceError } from "./DatasourceError.ts";
import { DeleteEnrichmentStrategy } from "./DeleteEnrichmentStrategy.ts";
import type { Enrichment } from "./Enrichment.ts";
import type { ICRUDRepository } from "./ICRUDRepository.ts";
import { UpdateEnrichmentStrategy } from "./UpdateEnrichmentStrategy.ts";

export class CascadeDeleteUpdateEnrichmentStrategy<T> extends UpdateEnrichmentStrategy<T> {
  private readonly create = new DefaultCreateEnrichmentStrategy<T>();

  async updateOwned(writer: ICRUDRepository<object>, enrichment: Enrichment, parent: object): Promise<void> {
    for (const row of await DeleteEnrichmentStrategy.load(writer, enrichment, parent)) await writer.delete(row);
    await this.create.createOwned(writer, enrichment, parent);
  }

  async updateLinked(writer: ICRUDRepository<object>, enrichment: Enrichment, parent: object): Promise<void> {
    const association = enrichment.association;
    if (!association) throw new DatasourceError("Missing association");
    const parentKey = this.requiredKey(parent, enrichment);
    for (const row of await DeleteEnrichmentStrategy.loadLinks(writer, association.parent, parentKey)) {
      const id = this.getCell(row, association.related);
      if (!this.presentCell(id)) continue;
      await writer.delete(row, this.linkWhere(association, parentKey, id));
    }
    await this.create.createLinked(writer, enrichment, parent);
  }
}
