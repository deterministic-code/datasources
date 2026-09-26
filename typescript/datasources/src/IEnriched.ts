import type { Enrichment } from "./Enrichment.ts";
import type { IReadonlyEnrichmentStrategy } from "./IReadonlyEnrichmentStrategy.ts";
import type { IReadonlyRepository } from "./IReadonlyRepository.ts";

export interface IEnriched<T> {
  addEnriched<TRelated extends object>(
    enrichment: Enrichment,
    via: IReadonlyRepository<TRelated>,
    strategy?: IReadonlyEnrichmentStrategy<T & object>,
  ): this;
}
