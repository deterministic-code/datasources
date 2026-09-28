import { DatasourceError } from "./DatasourceError.ts";
import { DeleteEnrichmentStrategy } from "./DeleteEnrichmentStrategy.ts";
import type { Enrichment } from "./Enrichment.ts";
import type { ICRUDRepository } from "./ICRUDRepository.ts";
import type { StoredEnrichment } from "./StoredEnrichment.ts";
import { UpdateEnrichmentStrategy } from "./UpdateEnrichmentStrategy.ts";

const writerOf = (entry: StoredEnrichment, into: string): ICRUDRepository<object> => {
  if (entry.persist) return entry.persist;
  throw new DatasourceError(`Deep write requires a CRUD repository for "${into}"`);
};

const linkWriter = (entry: StoredEnrichment, into: string): ICRUDRepository<object> => {
  if (entry.link) return entry.link;
  throw new DatasourceError(`Deep write requires a CRUD repository for "${into}"`);
};

export class CascadeDeleteUpdateEnrichmentStrategy<T> extends UpdateEnrichmentStrategy<T> {
  async updateOwned(entry: StoredEnrichment, enrichment: Enrichment, parent: object): Promise<void> {
    const writer = writerOf(entry, enrichment.into);
    for (const row of await DeleteEnrichmentStrategy.load(writer, enrichment, parent)) await writer.delete(row);
    await this.createChildren(writer, enrichment, parent);
  }

  async updateLinked(entry: StoredEnrichment, enrichment: Enrichment, parent: object): Promise<void> {
    const association = enrichment.association;
    if (!association) throw new DatasourceError("Missing association");
    const writer = linkWriter(entry, enrichment.into);
    const parentKey = this.requiredKey(parent, enrichment);
    for (const row of await DeleteEnrichmentStrategy.loadLinks(writer, association.parent, parentKey)) {
      const id = this.getCell(row, association.related);
      if (!this.presentCell(id)) continue;
      await writer.delete(row, this.linkWhere(association, parentKey, id));
    }
    await this.createLinks(writer, enrichment, parent);
  }
}
