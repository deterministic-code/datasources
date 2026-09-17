import { DefaultReadonlyEnrichmentStrategy } from "./DefaultReadonlyEnrichmentStrategy.ts";
import type { IEnriched } from "./IEnriched.ts";
import type { IQuery } from "./IQuery.ts";
import type { IReadonlyEnrichmentStrategy } from "./IReadonlyEnrichmentStrategy.ts";
import type { IReadonlyRepository } from "./IReadonlyRepository.ts";
import type { IRepositoryTree } from "./IRepositoryTree.ts";
import type { IRepositoryTreeNode } from "./IRepositoryTreeNode.ts";
import type { IWhere } from "./IWhere.ts";
import { RepositoryTree } from "./RepositoryTree.ts";
import { RepositoryTreeNode } from "./RepositoryTreeNode.ts";

export class EnrichedReadonlyRepository<T extends object>
  implements IReadonlyRepository<T>, IEnriched<T>
{
  readonly tree: IRepositoryTree;
  private readonly strategy: IReadonlyEnrichmentStrategy<T>;

  constructor(
    private readonly inner: IReadonlyRepository<T>,
    strategy?: IReadonlyEnrichmentStrategy<T>,
  ) {
    this.strategy = strategy ?? new DefaultReadonlyEnrichmentStrategy<T>();
    this.tree = new RepositoryTree(
      new RepositoryTreeNode(inner, "", undefined, undefined, undefined, this.strategy),
    );
  }

  addEnriched<TRelated extends object>(
    into: keyof T & string,
    from: keyof T & string,
    via: IReadonlyRepository<TRelated>,
    by: keyof TRelated & string,
    select?: keyof TRelated & string,
    many?: boolean,
    nest?: (node: IRepositoryTreeNode) => void,
  ): this {
    const child = this.tree.parent.include(via, into, from, by, select, many);
    nest?.(child);
    return this;
  }

  enrich = async (items: T[]): Promise<T[]> => {
    await this.strategy.enrich(this.tree, items);
    return items;
  };

  getSingle = async (query?: IQuery | IWhere): Promise<T | undefined> => {
    for await (const item of this.get(query)) return item;
    return undefined;
  };

  async *get(query?: IQuery | IWhere): AsyncIterable<T> {
    yield* this.strategy.get(this.inner, this.tree, query);
  }
}
