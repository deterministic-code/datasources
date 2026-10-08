import { DatasourceError } from "./DatasourceError.ts";
import { DeleteEnrichmentStrategy } from "./DeleteEnrichmentStrategy.ts";
import type { Enrichment } from "./Enrichment.ts";
import type { ICRUDRepository } from "./ICRUDRepository.ts";
import { UpdateEnrichmentStrategy } from "./UpdateEnrichmentStrategy.ts";

export class DefaultUpdateEnrichmentStrategy<T> extends UpdateEnrichmentStrategy<T> {
  async updateOwned(writer: ICRUDRepository<object>, enrichment: Enrichment, parent: T): Promise<void> {
    const existing = await DeleteEnrichmentStrategy.load(writer, enrichment, parent);
    const rows = this.payloadOf(parent, enrichment.destinationProperty, enrichment);
    for (const related of rows) this.stampCell(related, enrichment.relatedKey, this.getCell(parent, enrichment.sourceProperty));
    const keys = this.namesOf(writer, rows[0] ?? existing[0] ?? {});
    const keep = new Set(rows.filter((row) => this.hasKey(row, keys)).map((row) => this.keyOf(row, keys)));
    for (const row of existing.filter((row) => !keep.has(this.keyOf(row, keys)))) await writer.delete(row);
    for (const related of rows)
      this.hasKey(related, keys) && existing.some((row) => this.keyOf(row, keys) === this.keyOf(related, keys))
        ? await writer.update(related)
        : await writer.create(related);
  }

  async updateLinked(writer: ICRUDRepository<object>, enrichment: Enrichment, parent: T): Promise<void> {
    const association = enrichment.association;
    if (!association) throw new DatasourceError("Missing association");
    const parentKey = this.requiredKey(parent, enrichment);
    const existing = await DeleteEnrichmentStrategy.loadLinks(writer, association.parent, parentKey);
    const wanted = this.relatedIds(parent, enrichment);
    const keep = new Set(wanted.map((id) => String(id)));
    for (const row of existing) {
      const id = this.getCell(row, association.related);
      if (this.presentCell(id) && keep.has(String(id))) continue;
      if (!this.presentCell(id)) continue;
      await writer.delete(row, this.linkWhere(association, parentKey, id));
    }
    for (const id of wanted) {
      if (existing.some((row) => String(this.getCell(row, association.related) ?? "") === String(id))) continue;
      await writer.create({ [association.parent]: parentKey, [association.related]: id });
    }
  }
}
