import type { Scalar } from "./builders/Scalar.ts";
import { Operator, Where } from "./statements/index.ts";
import type { Enrichment } from "./Enrichment.ts";
import type { IReadonlyRepository } from "./IReadonlyRepository.ts";
import type { Row } from "./Row.ts";
import { DefaultReadonlyEnrichmentStrategy } from "./DefaultReadonlyEnrichmentStrategy.ts";
import { keyOf, props, scalar, sourceKey } from "./RepositoryTreeNode.ts";

const hasKey = (value: Scalar | object | undefined): value is Scalar =>
  value !== undefined && value !== null;

const keysOf = (
  parents: object[],
  sources: (Row | undefined)[] | undefined,
  from: string,
): Map<string, Scalar> => {
  const keys = new Map<string, Scalar>();
  for (const [index, parent] of parents.entries()) {
    const key = sourceKey(parent, from, sources, index);
    if (key === undefined || key === null) continue;
    keys.set(keyOf(key), scalar(key));
  }
  return keys;
};

export class HashsetOptimizedReadonlyEnrichmentStrategy<T extends object>
  extends DefaultReadonlyEnrichmentStrategy<T>
{
  protected override async applyLookup<TRelated extends object>(
    via: IReadonlyRepository<TRelated>,
    enrichment: Enrichment,
    parents: object[],
    sources?: (Row | undefined)[],
  ): Promise<void> {
    const keys = keysOf(parents, sources, enrichment.from);
    if (keys.size === 0) return;
    const index = new Map<string, TRelated>();
    for await (const item of via.read(
      new Where(new Operator.In(enrichment.by, [...keys.values()])),
    )) {
      const by = props(item)[enrichment.by];
      if (hasKey(by)) index.set(keyOf(by), item);
    }
    for (const [indexOfParent, parent] of parents.entries()) {
      const key = sourceKey(parent, enrichment.from, sources, indexOfParent);
      if (key === undefined || key === null) continue;
      const related = index.get(keyOf(key));
      if (!related) continue;
      Object.assign(parent, {
        [enrichment.into]: enrichment.select ? props(related)[enrichment.select] : related,
      });
    }
  }

  protected override async applyMany<TRelated extends object>(
    via: IReadonlyRepository<TRelated>,
    enrichment: Enrichment,
    parents: object[],
    sources?: (Row | undefined)[],
  ): Promise<void> {
    const keys = keysOf(parents, sources, enrichment.from);
    if (keys.size === 0) return;
    const index = new Map<string, TRelated[]>();
    for await (const item of via.read(
      new Where(new Operator.In(enrichment.by, [...keys.values()])),
    )) {
      const by = props(item)[enrichment.by];
      if (hasKey(by)) {
        const cacheKey = keyOf(by);
        const list = index.get(cacheKey) ?? [];
        list.push(item);
        index.set(cacheKey, list);
      }
    }
    for (const [indexOfParent, parent] of parents.entries()) {
      const key = sourceKey(parent, enrichment.from, sources, indexOfParent);
      if (key === undefined || key === null) continue;
      Object.assign(parent, { [enrichment.into]: index.get(keyOf(key)) ?? [] });
    }
  }
}
