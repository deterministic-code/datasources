import type { Cells } from "./converters/Cell.ts";
import type { Enrichment } from "./Enrichment.ts";
import type { IQuery } from "./statements/IQuery.ts";
import type { IReadonlyEnrichmentStrategy } from "./IReadonlyEnrichmentStrategy.ts";
import type { IReadonlyRepository } from "./IReadonlyRepository.ts";
import type { IRepositoryTree } from "./IRepositoryTree.ts";
import type { IWhere } from "./statements/IWhere.ts";
import type { IFlatToNestedMapper } from "./mappers/IFlatToNestedMapper.ts";
import type { IMapper } from "./mappers/IMapper.ts";

export class FlatToNestedReadonlyEnrichmentStrategy<T extends object>
  implements IReadonlyEnrichmentStrategy<T>
{
  constructor(
    private readonly source: IReadonlyRepository<Cells>,
    private readonly mapper: IFlatToNestedMapper<T>,
    private readonly hydrate: IMapper<T, T>,
    private readonly command?: IQuery | IWhere,
  ) {}

  async *read(
    _inner: IReadonlyRepository<T>,
    _tree: IRepositoryTree,
    query?: IQuery | IWhere,
  ): AsyncIterable<T> {
    const rows: Cells[] = [];
    for await (const row of this.source.read(this.command ?? query)) rows.push(row);
    for (const item of this.mapper.map(rows).map(this.hydrate.map))
      yield item;
  }

  enrich = async (_tree: IRepositoryTree, _items: T[]): Promise<void> => {};

  apply = async <TRelated extends object>(
    _via: IReadonlyRepository<TRelated>,
    _enrichment: Enrichment,
    _parents: object[],
  ): Promise<void> => {};
}
