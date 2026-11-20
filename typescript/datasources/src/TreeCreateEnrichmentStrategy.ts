import { CreateEnrichmentStrategy } from "./CreateEnrichmentStrategy.ts";
import { DatasourceError } from "./DatasourceError.ts";
import { enrichmentMatches, type Enrichment } from "./Enrichment.ts";
import type { ICRUDRepository } from "./ICRUDRepository.ts";
import type { Scalar } from "./builders/Scalar.ts";

export class TreeCreateEnrichmentStrategy<T extends object> extends CreateEnrichmentStrategy<T> {
  private readonly inserted = new Map<string, Scalar>();
  private depth = 0;

  createLinked = async <TChild extends object>(
    _writer: ICRUDRepository<TChild>,
    _enrichment: Enrichment,
    _parent: T,
  ): Promise<void> => {};

  createChildren = async <TChild extends object>(writer: ICRUDRepository<TChild>, enrichment: Enrichment, parent: T): Promise<void> => {
    this.depth += 1;
    try {
      this.inserted.set(nodeKey(parent), this.requiredValue(parent, enrichment.sourceProperty, enrichment.destinationProperty));
      const children = this.payloadOf<TChild, T>(parent, enrichment.destinationProperty, enrichment, writer);
      for (const child of children) {
        for (const match of enrichmentMatches(enrichment))
          this.stampCell(child, match.relatedKey, this.requiredValue(parent, match.sourceProperty, enrichment.destinationProperty));
        const key = nodeKey(child);
        if (this.inserted.has(key)) throw new DatasourceError(`Tree create cycle at "${key}"`);
        const created = await writer.create(child);
        const relatedId = this.requiredValue(created, enrichment.sourceProperty, enrichment.destinationProperty);
        this.inserted.set(key, relatedId);
        this.stampCell(child, enrichment.sourceProperty, relatedId);
        await this.createChildren(writer, enrichment, child as never as T);
      }
    } finally {
      this.depth -= 1;
      if (this.depth === 0) this.inserted.clear();
    }
  };
}

export const nodeKey = (item: object): string => {
  const record = item as Record<string, Scalar | object | undefined>;
  const name = record.Name ?? record.name ?? record.Value ?? record.value;
  if (name !== undefined && name !== null && typeof name !== "object" && name !== "") return String(name);
  const id = record.Id ?? record.id;
  if (id !== undefined && id !== null && typeof id !== "object" && id !== 0 && id !== 0n) return String(id);
  const region = record.RegionId ?? record.regionId;
  const category = record.CategoryId ?? record.categoryId;
  if (region !== undefined && category !== undefined) return `${String(region)}:${String(category)}`;
  throw new DatasourceError("Tree create requires Name, Value, Id, or RegionId/CategoryId");
};
