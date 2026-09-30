import type { Enrichment } from "./Enrichment.ts";
import type { ICRUDRepository } from "./ICRUDRepository.ts";
import type { IReadonlyRepository } from "./IReadonlyRepository.ts";
import type { Row } from "./Row.ts";

export interface IReadonlyEnrichmentStrategy<T extends object> {
  apply<TRelated extends object>(
    via: IReadonlyRepository<TRelated>,
    enrichment: Enrichment,
    parents: object[],
    sources?: (Row | undefined)[],
  ): Promise<void>;
  applyLinked<TRelated extends object, TLink extends object>(
    via: IReadonlyRepository<TRelated>,
    link: ICRUDRepository<TLink>,
    enrichment: Enrichment,
    parents: object[],
    sources?: (Row | undefined)[],
  ): Promise<void>;
}
