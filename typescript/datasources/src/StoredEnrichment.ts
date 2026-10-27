import type { Enrichment } from "./Enrichment.ts";
import type { ICRUDRepository } from "./ICRUDRepository.ts";
import type { IReadonlyRepository } from "./IReadonlyRepository.ts";
import type { Row } from "./Row.ts";

export type ApplyEnrichment<T> = (
  parents: T[],
  sources?: (Row | undefined)[],
) => Promise<void>;

export class StoredEnrichment<T, TRelated = T, TLink = TRelated> {
  apply: ApplyEnrichment<T>;
  persist?: ICRUDRepository<TRelated>;
  link?: ICRUDRepository<TLink>;
  via?: IReadonlyRepository<TRelated>;

  constructor(
    readonly enrichment: Enrichment,
    apply: ApplyEnrichment<T>,
    persist?: ICRUDRepository<TRelated>,
    link?: ICRUDRepository<TLink>,
    via?: IReadonlyRepository<TRelated>,
  ) {
    this.apply = apply;
    this.persist = persist;
    this.link = link;
    this.via = via;
  }
}
