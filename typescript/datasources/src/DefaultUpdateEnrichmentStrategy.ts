import { enrichmentMatches } from "./Enrichment.ts";
import { DatasourceError } from "./DatasourceError.ts";
import { DeleteEnrichmentStrategy } from "./DeleteEnrichmentStrategy.ts";
import type { Enrichment } from "./Enrichment.ts";
import type { ICRUDRepository } from "./ICRUDRepository.ts";
import { UpdateEnrichmentStrategy } from "./UpdateEnrichmentStrategy.ts";

export class DefaultUpdateEnrichmentStrategy<T> extends UpdateEnrichmentStrategy<T> {
  async updateChildren<TChild extends object>(writer: ICRUDRepository<TChild>, enrichment: Enrichment, parent: T): Promise<void> {
    await this.updateChildrenWith(writer, enrichment, parent, await DeleteEnrichmentStrategy.load(writer, enrichment, parent));
  }

  /** `updateChildren` for a parent whose current children were already loaded. */
  async updateChildrenWith<TChild extends object>(
    writer: ICRUDRepository<TChild>,
    enrichment: Enrichment,
    parent: T,
    existing: TChild[],
  ): Promise<void> {
    const rows = this.payloadOf<TChild, T>(parent, enrichment.destinationProperty, enrichment, writer);
    for (const related of rows)
      for (const match of enrichmentMatches(enrichment))
        this.stampCell(related, match.relatedKey, this.getCell(parent, match.sourceProperty));
    const keys = this.namesOf(writer, rows[0] ?? existing[0] ?? {} as TChild);
    const keep = new Set(rows.filter((row) => this.hasKey(row, keys)).map((row) => this.keyOf(row, keys)));
    for (const row of existing.filter((row) => !keep.has(this.keyOf(row, keys))))
      await writer.delete(row, DeleteEnrichmentStrategy.childrenWhere(writer, enrichment, parent));
    for (const related of rows)
      this.hasKey(related, keys) && existing.some((row) => this.keyOf(row, keys) === this.keyOf(related, keys))
        ? await writer.update(related)
        : await writer.create(related);
  }

  async updateLinked<TChild extends object>(writer: ICRUDRepository<TChild>, enrichment: Enrichment, parent: T): Promise<void> {
    await this.updateLinkedWith(writer, enrichment, parent);
  }

  /** `updateLinked` for a parent whose current link rows were already loaded (`undefined` loads them). */
  async updateLinkedWith<TChild extends object>(
    writer: ICRUDRepository<TChild>,
    enrichment: Enrichment,
    parent: T,
    loaded?: TChild[],
  ): Promise<void> {
    const association = enrichment.association;
    if (!association) throw new DatasourceError("Missing association");
    const parentKey = this.requiredKey(parent, enrichment);
    const existing = loaded ?? (await DeleteEnrichmentStrategy.loadLinks(writer, association.parent, parentKey));
    const wanted = this.relatedIds(parent, enrichment);
    const keep = new Set(wanted.map((id) => String(id)));
    for (const row of existing) {
      const id = this.getCell(row, association.related);
      if (id !== undefined && id !== null && keep.has(String(id))) continue;
      await writer.delete(row, DeleteEnrichmentStrategy.linkWhere(association, parentKey, DeleteEnrichmentStrategy.linkKey(id)));
    }
    for (const id of wanted) {
      if (existing.some((row) => String(this.getCell(row, association.related) ?? "") === String(id))) continue;
      await writer.create({ [association.parent]: parentKey, [association.related]: id } as TChild);
    }
  }
}
