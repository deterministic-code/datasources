import type { Cells } from "./converters/Cell.ts";
import type { Enrichment } from "./Enrichment.ts";
import type { IMapper } from "./mappers/IMapper.ts";
import type { IReadonlyRepository } from "./IReadonlyRepository.ts";
import type { Row } from "./Row.ts";
import { DefaultReadonlyEnrichmentStrategy } from "./DefaultReadonlyEnrichmentStrategy.ts";

export class NestedToObjectEnrichmentStrategy<T extends object>
  extends DefaultReadonlyEnrichmentStrategy<T>
{
  constructor(private readonly mapper: IMapper<Iterable<Cells>, T[]>) {
    super();
  }

  override apply<TRelated extends object>(
    _via: IReadonlyRepository<TRelated>,
    _enrichment: Enrichment,
    parents: object[],
    sources?: (Row | undefined)[],
  ): Promise<void> {
    const rows: Cells[] = [];
    for (const source of sources ?? []) if (source) rows.push(source);
    parents.length = 0;
    for (const item of this.mapper.map(rows)) parents.push(item);
    return Promise.resolve();
  }
}
