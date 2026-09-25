import type { Scalar } from "./builders/Scalar.ts";
import { Operator, Where } from "./statements/index.ts";
import type { Enrichment } from "./Enrichment.ts";
import type { IExecuteStoredProcedureStatement } from "./statements/IExecuteStoredProcedureStatement.ts";
import type { IQuery } from "./statements/IQuery.ts";
import type { IReadonlyEnrichmentStrategy } from "./IReadonlyEnrichmentStrategy.ts";
import type { IReadonlyRepository } from "./IReadonlyRepository.ts";
import type { IRepositoryTree } from "./IRepositoryTree.ts";
import type { IWhere } from "./statements/IWhere.ts";
import type { Row } from "./Row.ts";
import { readTracked } from "./ReadonlyRepository.ts";
import { RepositoryTreeNode, keyOf, props, scalar, sourceKey } from "./RepositoryTreeNode.ts";

export class HashsetOptimizedReadonlyEnrichmentStrategy<T extends object>
  implements IReadonlyEnrichmentStrategy<T>
{
  async *read(
    inner: IReadonlyRepository<T>,
    tree: IRepositoryTree,
    query?: IQuery | IWhere | IExecuteStoredProcedureStatement,
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
    await tree.parent.enrichFrom(items, sources);
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

const applyLookup = async <TRelated extends object>(
  via: IReadonlyRepository<TRelated>,
  enrichment: Enrichment,
  parents: object[],
  sources?: (Row | undefined)[],
): Promise<void> => {
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
      [enrichment.into]: enrichment.select
        ? props(related)[enrichment.select]
        : related,
    });
  }
};

const applyMany = async <TRelated extends object>(
  via: IReadonlyRepository<TRelated>,
  enrichment: Enrichment,
  parents: object[],
  sources?: (Row | undefined)[],
): Promise<void> => {
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
    Object.assign(parent, {
      [enrichment.into]: index.get(keyOf(key)) ?? [],
    });
  }
};
