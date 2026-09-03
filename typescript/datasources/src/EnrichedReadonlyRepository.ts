import type { IEnriched } from "./IEnriched.ts";
import type { IQuery } from "./IQuery.ts";
import type { IReadonlyRepository } from "./IReadonlyRepository.ts";
import type { IRepositoryTree } from "./IRepositoryTree.ts";
import type { IWhere } from "./IWhere.ts";
import { RepositoryTree } from "./RepositoryTree.ts";
import { RepositoryTreeNode } from "./RepositoryTreeNode.ts";

export class EnrichedReadonlyRepository<T extends object>
  implements IReadonlyRepository<T>, IEnriched<T>
{
  readonly tree: IRepositoryTree;

  constructor(private readonly inner: IReadonlyRepository<T>) {
    this.tree = new RepositoryTree(new RepositoryTreeNode(inner, ""));
  }

  addEnriched<TRelated extends object>(
    into: keyof T & string,
    from: keyof T & string,
    via: IReadonlyRepository<TRelated>,
    by: keyof TRelated & string,
    select?: keyof TRelated & string,
    many?: boolean,
  ): this {
    this.tree.parent.include(via, into, from, by, select, many);
    return this;
  }

  enrich = async (items: T[]): Promise<T[]> => {
    await this.tree.parent.enrichFrom(items);
    return items;
  };

  getSingle = async (query?: IQuery | IWhere): Promise<T | undefined> => {
    for await (const item of this.get(query)) return item;
    return undefined;
  };

  async *get(query?: IQuery | IWhere): AsyncIterable<T> {
    const items: T[] = [];
    for await (const item of this.inner.get(query)) items.push(item);
    await this.enrich(items);
    for (const item of items) yield item;
  }
}
