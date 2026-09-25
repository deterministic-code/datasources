import { DefaultReadonlyEnrichmentStrategy } from "./DefaultReadonlyEnrichmentStrategy.ts";
import type { ICRUDRepository } from "./ICRUDRepository.ts";
import type { IEnriched } from "./IEnriched.ts";
import type { IExecuteStoredProcedureStatement } from "./statements/IExecuteStoredProcedureStatement.ts";
import type { IQuery } from "./statements/IQuery.ts";
import type { IReadonlyEnrichmentStrategy } from "./IReadonlyEnrichmentStrategy.ts";
import type { IReadonlyRepository } from "./IReadonlyRepository.ts";
import type { IRepositoryTree } from "./IRepositoryTree.ts";
import type { IRepositoryTreeNode } from "./IRepositoryTreeNode.ts";
import type { IWhere } from "./statements/IWhere.ts";
import { WhereText } from "./statements/WhereText.ts";
import { Where } from "./statements/Where.ts";
import { ReadonlyRepository } from "./ReadonlyRepository.ts";
import { RepositoryTree } from "./RepositoryTree.ts";
import { RepositoryTreeNode } from "./RepositoryTreeNode.ts";
import type { Row } from "./Row.ts";
import { readTracked, tracked, type Sourced } from "./ReadonlyRepository.ts";

export class EnrichedReadonlyRepository<T extends object>
  extends ReadonlyRepository<T>
  implements IEnriched<T>
{
  readonly tree: IRepositoryTree;
  private readonly strategy: IReadonlyEnrichmentStrategy<T>;

  constructor(
    private readonly inner: IReadonlyRepository<T>,
    strategy?: IReadonlyEnrichmentStrategy<T>,
  ) {
    super(inner.type);
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

  addLinked<TRelated extends object, TLink extends object>(
    into: keyof T & string,
    from: keyof T & string,
    via: IReadonlyRepository<TRelated>,
    by: keyof TRelated & string,
    link: ICRUDRepository<TLink>,
    parent: keyof TLink & string,
    related: keyof TLink & string,
  ): this {
    this.tree.parent.includeLinked(via, into, from, by, link, parent, related);
    return this;
  }

  enrich = async (items: T[]): Promise<T[]> => {
    await this.strategy.enrich(this.tree, items);
    return items;
  };

  async *read(query?: IQuery | IWhere | IExecuteStoredProcedureStatement): AsyncIterable<T> {
    if (query instanceof Where || query instanceof WhereText) {
      for await (const sourced of this[tracked](query)) yield sourced.item;
      return;
    }
    yield* this.strategy.read(this.inner, this.tree, query);
  }

  async *[tracked](
    query?: IQuery | IWhere | IExecuteStoredProcedureStatement,
  ): AsyncIterable<Sourced<T>> {
    const items: T[] = [];
    const sources: (Row | undefined)[] = [];
    for await (const sourced of readTracked(this.inner, query)) {
      items.push(sourced.item);
      sources.push(sourced.source);
    }
    await this.strategy.enrich(this.tree, items, sources);
    for (let index = 0; index < items.length; index++)
      yield { item: items[index]!, source: sources[index] };
  }
}
