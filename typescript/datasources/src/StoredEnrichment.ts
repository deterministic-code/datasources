import type { Enrichment } from "./Enrichment.ts";
import type { ICRUDRepository } from "./ICRUDRepository.ts";
import type { Row } from "./Row.ts";

export type ApplyEnrichment = (
  parents: object[],
  sources?: (Row | undefined)[],
) => Promise<void>;

export class StoredEnrichment {
  constructor(
    readonly enrichment: Enrichment,
    readonly apply: ApplyEnrichment,
    readonly persist?: ICRUDRepository<object>,
    readonly link?: ICRUDRepository<object>,
  ) {}
}
