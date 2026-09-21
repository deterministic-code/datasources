import { Operator, WhereStatement } from "./statements/index.ts";
import type { Enrichment } from "./Enrichment.ts";
import type { IQuery } from "./IQuery.ts";
import type { IReadonlyEnrichmentStrategy } from "./IReadonlyEnrichmentStrategy.ts";
import type { IReadonlyRepository } from "./IReadonlyRepository.ts";
import type { IRepositoryTree } from "./IRepositoryTree.ts";
import type { IWhere } from "./IWhere.ts";
import { keyOf, props, scalar } from "./RepositoryTreeNode.ts";

export class DefaultReadonlyEnrichmentStrategy<T extends object>
  implements IReadonlyEnrichmentStrategy<T>
{
  async *read(
    inner: IReadonlyRepository<T>,
    tree: IRepositoryTree,
    query?: IQuery | IWhere,
  ): AsyncIterable<T> {
    const items: T[] = [];
    for await (const item of inner.read(query)) items.push(item);
    await this.enrich(tree, items);
    for (const item of items) yield item;
  }

  enrich = async (tree: IRepositoryTree, items: T[]): Promise<void> => {
    await tree.parent.enrichFrom(items);
  };

  apply = async <TRelated extends object>(
    via: IReadonlyRepository<TRelated>,
    enrichment: Enrichment,
    parents: object[],
  ): Promise<void> =>
    enrichment.many
      ? applyMany(via, enrichment, parents)
      : applyLookup(via, enrichment, parents);
}

const applyLookup = async <TRelated extends object>(
  via: IReadonlyRepository<TRelated>,
  enrichment: Enrichment,
  parents: object[],
): Promise<void> => {
  const cache = new Map<string, TRelated | undefined>();
  for (const parent of parents) {
    const key = props(parent)[enrichment.from];
    if (key === undefined || key === null) continue;
    const cacheKey = keyOf(key);
    if (!cache.has(cacheKey))
      cache.set(
        cacheKey,
        await via.readSingle(new WhereStatement(new Operator.Equals(enrichment.by, scalar(key)))),
      );
    const related = cache.get(cacheKey);
    if (!related) continue;
    const value = enrichment.select
      ? props(related)[enrichment.select]
      : related;
    Object.assign(parent, { [enrichment.into]: value });
  }
};

const applyMany = async <TRelated extends object>(
  via: IReadonlyRepository<TRelated>,
  enrichment: Enrichment,
  parents: object[],
): Promise<void> => {
  for (const parent of parents) {
    const key = props(parent)[enrichment.from];
    if (key === undefined || key === null) continue;
    const related: TRelated[] = [];
    for await (const item of via.read(
      new WhereStatement(new Operator.Equals(enrichment.by, scalar(key))),
    ))
      related.push(item);
    Object.assign(parent, { [enrichment.into]: related });
  }
};
