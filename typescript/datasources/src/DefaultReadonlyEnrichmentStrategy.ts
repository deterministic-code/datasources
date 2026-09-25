import { Operator, Where } from "./statements/index.ts";
import type { Enrichment } from "./Enrichment.ts";
import type { IReadonlyEnrichmentStrategy } from "./IReadonlyEnrichmentStrategy.ts";
import type { IReadonlyRepository } from "./IReadonlyRepository.ts";
import type { ITrackedApply } from "./ITrackedApply.ts";
import type { Row } from "./Row.ts";
import { keyOf, props, scalar, sourceKey } from "./RepositoryTreeNode.ts";

export class DefaultReadonlyEnrichmentStrategy<T extends object>
  implements IReadonlyEnrichmentStrategy<T>, ITrackedApply
{
  apply = <TRelated extends object>(
    via: IReadonlyRepository<TRelated>,
    enrichment: Enrichment,
    parents: object[],
  ): Promise<void> => this.applyTracked(via, enrichment, parents);

  applyTracked = <TRelated extends object>(
    via: IReadonlyRepository<TRelated>,
    enrichment: Enrichment,
    parents: object[],
    sources?: (Row | undefined)[],
  ): Promise<void> =>
    enrichment.many
      ? applyMany(via, enrichment, parents, sources)
      : applyLookup(via, enrichment, parents, sources);
}

const applyLookup = async <TRelated extends object>(
  via: IReadonlyRepository<TRelated>,
  enrichment: Enrichment,
  parents: object[],
  sources?: (Row | undefined)[],
): Promise<void> => {
  const cache = new Map<string, TRelated | undefined>();
  for (const [index, parent] of parents.entries()) {
    const key = sourceKey(parent, enrichment.from, sources, index);
    if (key === undefined || key === null) continue;
    const cacheKey = keyOf(key);
    if (!cache.has(cacheKey))
      cache.set(
        cacheKey,
        await via.readSingle(new Where(new Operator.Equals(enrichment.by, scalar(key)))),
      );
    const related = cache.get(cacheKey);
    if (!related) continue;
    Object.assign(parent, {
      [enrichment.into]: enrichment.select ? props(related)[enrichment.select] : related,
    });
  }
};

const applyMany = async <TRelated extends object>(
  via: IReadonlyRepository<TRelated>,
  enrichment: Enrichment,
  parents: object[],
  sources?: (Row | undefined)[],
): Promise<void> => {
  for (const [index, parent] of parents.entries()) {
    const key = sourceKey(parent, enrichment.from, sources, index);
    if (key === undefined || key === null) continue;
    const related: TRelated[] = [];
    for await (const item of via.read(
      new Where(new Operator.Equals(enrichment.by, scalar(key))),
    ))
      related.push(item);
    Object.assign(parent, { [enrichment.into]: related });
  }
};
