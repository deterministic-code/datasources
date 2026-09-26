import type { Cells } from "./converters/Cell.ts";
import type { Enrichment } from "./Enrichment.ts";
import type { IReadonlyRepository } from "./IReadonlyRepository.ts";
import { FlatToNestedMapper } from "./mappers/FlatToNestedMapper.ts";
import type { TypeMap } from "./mappers/TypeMap.ts";
import type { Row } from "./Row.ts";
import { DefaultReadonlyEnrichmentStrategy } from "./DefaultReadonlyEnrichmentStrategy.ts";

export class NestedToObjectEnrichmentStrategy<T extends object>
  extends DefaultReadonlyEnrichmentStrategy<T>
{
  private readonly nested: FlatToNestedMapper<T>;

  constructor(typeMap: TypeMap<T>) {
    super();
    this.nested = new FlatToNestedMapper(typeMap);
  }

  override apply<TRelated extends object>(
    _via: IReadonlyRepository<TRelated>,
    _enrichment: Enrichment,
    parents: object[],
    sources?: (Row | undefined)[],
  ): Promise<void> {
    parents.length = 0;
    for (const item of this.nested.map(rows(sources))) parents.push(item);
    return Promise.resolve();
  }
}

function* rows(sources?: (Row | undefined)[]): Iterable<Cells> {
  for (const source of sources ?? []) if (source) yield source;
}
