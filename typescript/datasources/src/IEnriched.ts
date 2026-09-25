import type { Enrichment } from "./Enrichment.ts";
import type { ICRUDRepository } from "./ICRUDRepository.ts";
import type { IReadonlyEnrichmentStrategy } from "./IReadonlyEnrichmentStrategy.ts";
import type { IReadonlyRepository } from "./IReadonlyRepository.ts";

export interface IEnriched<T> {
  addEnriched<TRelated extends object>(
    enrichment: Enrichment,
    via: IReadonlyRepository<TRelated>,
    strategy?: IReadonlyEnrichmentStrategy<T & object>,
  ): this;
  addLinked<TRelated extends object, TLink extends object>(
    enrichment: Enrichment,
    via: IReadonlyRepository<TRelated>,
    link: ICRUDRepository<TLink>,
    strategy?: IReadonlyEnrichmentStrategy<T & object>,
  ): this;
}
