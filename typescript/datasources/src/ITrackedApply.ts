import type { Enrichment } from "./Enrichment.ts";
import type { IReadonlyRepository } from "./IReadonlyRepository.ts";
import type { Row } from "./Row.ts";

export interface ITrackedApply {
  applyTracked<TRelated extends object>(
    via: IReadonlyRepository<TRelated>,
    enrichment: Enrichment,
    parents: object[],
    sources?: (Row | undefined)[],
  ): Promise<void>;
}
