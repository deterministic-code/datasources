import type { Scalar } from "./builders/Scalar.ts";
import { WhereBuilder } from "./builders/WhereBuilder.ts";
import type { Enrichment } from "./Enrichment.ts";
import type { IQuery } from "./IQuery.ts";
import type { IReadonlyEnrichmentStrategy } from "./IReadonlyEnrichmentStrategy.ts";
import type { IReadonlyRepository } from "./IReadonlyRepository.ts";
import type { IRepositoryTree } from "./IRepositoryTree.ts";
import type { IWhere } from "./IWhere.ts";
import { keyOf, props, scalar } from "./RepositoryTreeNode.ts";

export class HashsetOptimizedReadonlyEnrichmentStrategy<T extends object>
  implements IReadonlyEnrichmentStrategy<T>
{
  async *get(
    inner: IReadonlyRepository<T>,
    tree: IRepositoryTree,
    query?: IQuery | IWhere,
  ): AsyncIterable<T> {
    const items: T[] = [];
    for await (const item of inner.get(query)) items.push(item);
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

const hasKey = (value: Scalar | object | undefined): value is Scalar =>
  value !== undefined && value !== null;

const keysOf = (parents: object[], from: string): Map<string, Scalar> => {
  const keys = new Map<string, Scalar>();
  for (const parent of parents) {
    const key = props(parent)[from];
    if (key === undefined || key === null) continue;
    keys.set(keyOf(key), scalar(key));
  }
  return keys;
};

const applyLookup = async <TRelated extends object>(
  via: IReadonlyRepository<TRelated>,
  enrichment: Enrichment,
  parents: object[],
): Promise<void> => {
  const keys = keysOf(parents, enrichment.from);
  if (keys.size === 0) return;
  const index = new Map<string, TRelated>();
  for await (const item of via.get(
    WhereBuilder.in(enrichment.by, [...keys.values()]),
  )) {
    const by = props(item)[enrichment.by];
    if (hasKey(by)) index.set(keyOf(by), item);
  }
  for (const parent of parents) {
    const key = props(parent)[enrichment.from];
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
): Promise<void> => {
  const keys = keysOf(parents, enrichment.from);
  if (keys.size === 0) return;
  const index = new Map<string, TRelated[]>();
  for await (const item of via.get(
    WhereBuilder.in(enrichment.by, [...keys.values()]),
  )) {
    const by = props(item)[enrichment.by];
    if (hasKey(by)) {
      const cacheKey = keyOf(by);
      const list = index.get(cacheKey) ?? [];
      list.push(item);
      index.set(cacheKey, list);
    }
  }
  for (const parent of parents) {
    const key = props(parent)[enrichment.from];
    if (key === undefined || key === null) continue;
    Object.assign(parent, {
      [enrichment.into]: index.get(keyOf(key)) ?? [],
    });
  }
};
