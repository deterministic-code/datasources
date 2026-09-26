import type { Enrichment } from "./Enrichment.ts";
import type { ICRUDRepository } from "./ICRUDRepository.ts";
import type { Row } from "./Row.ts";

export type ApplyEnrichment = (
  parents: object[],
  sources?: (Row | undefined)[],
) => Promise<void>;

export class StoredEnrichment {
  apply: ApplyEnrichment;
  persist?: ICRUDRepository<object>;
  link?: ICRUDRepository<object>;

  constructor(
    readonly enrichment: Enrichment,
    apply: ApplyEnrichment,
    persist?: ICRUDRepository<object>,
    link?: ICRUDRepository<object>,
  ) {
    this.apply = apply;
    this.persist = persist;
    this.link = link;
  }
}
