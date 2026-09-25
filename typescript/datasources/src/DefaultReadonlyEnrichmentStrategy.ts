import { Operator, Where } from "./statements/index.ts";
import type { Enrichment } from "./Enrichment.ts";
import type { IExecuteStoredProcedureStatement } from "./statements/IExecuteStoredProcedureStatement.ts";
import type { ISelectStatement } from "./statements/ISelectStatement.ts";
import type { IReadonlyEnrichmentStrategy } from "./IReadonlyEnrichmentStrategy.ts";
import type { IReadonlyRepository } from "./IReadonlyRepository.ts";
import type { IRepositoryTree } from "./IRepositoryTree.ts";
import type { IWhere } from "./statements/IWhere.ts";
import type { Row } from "./Row.ts";
import { readTracked } from "./ReadonlyRepository.ts";
import { RepositoryTreeNode, keyOf, props, scalar, sourceKey } from "./RepositoryTreeNode.ts";

export class DefaultReadonlyEnrichmentStrategy<T extends object>
  implements IReadonlyEnrichmentStrategy<T>
{
  async *read(
    inner: IReadonlyRepository<T>,
    tree: IRepositoryTree,
    query?: ISelectStatement | IWhere | IExecuteStoredProcedureStatement,
  ): AsyncIterable<T> {
    const items: T[] = [];
    const sources: (Row | undefined)[] = [];
    for await (const sourced of readTracked(inner, query)) {
      items.push(sourced.item);
      sources.push(sourced.source);
    }
    await this.enrich(tree, items, sources);
    for (const item of items) yield item;
  }

  enrich = async (
    tree: IRepositoryTree,
    items: T[],
    sources?: (Row | undefined)[],
  ): Promise<void> => {
    await (tree.parent as RepositoryTreeNode).enrichFrom(items, sources);
  };

  apply = async <TRelated extends object>(
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
