import type { Scalar } from "./builders/Scalar.ts";
import { Operator, Where } from "./statements/index.ts";
import type { Enrichment } from "./Enrichment.ts";
import type { IReadonlyRepository } from "./IReadonlyRepository.ts";
import type { Row } from "./Row.ts";
import { DefaultReadonlyEnrichmentStrategy } from "./DefaultReadonlyEnrichmentStrategy.ts";

export class HashsetOptimizedReadonlyEnrichmentStrategy<T extends object>
  extends DefaultReadonlyEnrichmentStrategy<T>
{
  protected override async applyObject<TRelated extends object>(
    via: IReadonlyRepository<TRelated>,
    enrichment: Enrichment,
    parents: T[],
    sources?: (Row | undefined)[],
  ): Promise<void> {
    const keys = this.keysOf(parents, sources, enrichment);
    if (keys.size === 0) return;
    const index = new Map<string, TRelated>();
    for await (const item of via.read(
      new Where(new Operator.In(enrichment.relatedKey, [...keys.values()])),
    )) {
      const cacheKey = this.rowMatchKey(item, enrichment);
      if (cacheKey !== undefined) index.set(cacheKey, item);
    }
    for (const [indexOfParent, parent] of parents.entries()) {
      const cacheKey = this.matchKey(parent, enrichment, sources, indexOfParent);
      if (cacheKey === undefined) continue;
      const related = index.get(cacheKey);
      if (!related) continue;
      Object.assign(parent, {
        [enrichment.destinationProperty]: enrichment.relatedProperty ? this.props(related)[enrichment.relatedProperty] : related,
      });
    }
  }

  protected override async applyList<TRelated extends object>(
    via: IReadonlyRepository<TRelated>,
    enrichment: Enrichment,
    parents: T[],
    sources?: (Row | undefined)[],
  ): Promise<void> {
    const keys = this.keysOf(parents, sources, enrichment);
    if (keys.size === 0) return;
    const index = new Map<string, TRelated[]>();
    for await (const item of via.read(
      new Where(new Operator.In(enrichment.relatedKey, [...keys.values()])),
    )) {
      const cacheKey = this.rowMatchKey(item, enrichment);
      if (cacheKey === undefined) continue;
      const list = index.get(cacheKey) ?? [];
      list.push(item);
      index.set(cacheKey, list);
    }
    for (const [indexOfParent, parent] of parents.entries()) {
      const cacheKey = this.matchKey(parent, enrichment, sources, indexOfParent);
      if (cacheKey === undefined) continue;
      Object.assign(parent, { [enrichment.destinationProperty]: index.get(cacheKey) ?? [] });
    }
  }

  private keysOf(
    parents: T[],
    sources: (Row | undefined)[] | undefined,
    enrichment: Enrichment,
  ): Map<string, Scalar> {
    const keys = new Map<string, Scalar>();
    for (const [index, parent] of parents.entries()) {
      const key = this.sourceKey(parent, enrichment, sources, index);
      if (key === undefined || key === null) continue;
      keys.set(this.keyOf(key), this.scalar(key));
    }
    return keys;
  }
}
