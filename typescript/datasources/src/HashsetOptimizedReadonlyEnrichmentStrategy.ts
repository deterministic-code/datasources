import type { Scalar } from "./builders/Scalar.ts";
import { Operator, Where } from "./statements/index.ts";
import type { Enrichment } from "./Enrichment.ts";
import type { IReadonlyRepository } from "./IReadonlyRepository.ts";
import type { Row } from "./Row.ts";
import { DefaultReadonlyEnrichmentStrategy } from "./DefaultReadonlyEnrichmentStrategy.ts";

const hasKey = (value: Scalar | object | undefined): value is Scalar =>
  value !== undefined && value !== null;

export class HashsetOptimizedReadonlyEnrichmentStrategy<T extends object>
  extends DefaultReadonlyEnrichmentStrategy<T>
{
  protected override async applyLookup<TRelated extends object>(
    via: IReadonlyRepository<TRelated>,
    enrichment: Enrichment,
    parents: object[],
    sources?: (Row | undefined)[],
  ): Promise<void> {
    const keys = this.keysOf(parents, sources, enrichment.from);
    if (keys.size === 0) return;
    const index = new Map<string, TRelated>();
    for await (const item of via.read(
      new Where(new Operator.In(enrichment.by, [...keys.values()])),
    )) {
      const by = this.props(item)[enrichment.by];
      if (hasKey(by)) index.set(this.keyOf(by), item);
    }
    for (const [indexOfParent, parent] of parents.entries()) {
      const key = this.sourceKey(parent, enrichment.from, sources, indexOfParent);
      if (key === undefined || key === null) continue;
      const related = index.get(this.keyOf(key));
      if (!related) continue;
      Object.assign(parent, {
        [enrichment.into]: enrichment.select ? this.props(related)[enrichment.select] : related,
      });
    }
  }

  protected override async applyMany<TRelated extends object>(
    via: IReadonlyRepository<TRelated>,
    enrichment: Enrichment,
    parents: object[],
    sources?: (Row | undefined)[],
  ): Promise<void> {
    const keys = this.keysOf(parents, sources, enrichment.from);
    if (keys.size === 0) return;
    const index = new Map<string, TRelated[]>();
    for await (const item of via.read(
      new Where(new Operator.In(enrichment.by, [...keys.values()])),
    )) {
      const by = this.props(item)[enrichment.by];
      if (hasKey(by)) {
        const cacheKey = this.keyOf(by);
        const list = index.get(cacheKey) ?? [];
        list.push(item);
        index.set(cacheKey, list);
      }
    }
    for (const [indexOfParent, parent] of parents.entries()) {
      const key = this.sourceKey(parent, enrichment.from, sources, indexOfParent);
      if (key === undefined || key === null) continue;
      Object.assign(parent, { [enrichment.into]: index.get(this.keyOf(key)) ?? [] });
    }
  }

  private keysOf(
    parents: object[],
    sources: (Row | undefined)[] | undefined,
    from: string,
  ): Map<string, Scalar> {
    const keys = new Map<string, Scalar>();
    for (const [index, parent] of parents.entries()) {
      const key = this.sourceKey(parent, from, sources, index);
      if (key === undefined || key === null) continue;
      keys.set(this.keyOf(key), this.scalar(key));
    }
    return keys;
  }
}
