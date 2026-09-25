import type { StoredEnrichment } from "./StoredEnrichment.ts";

export interface IEnrichmentHost {
  readonly enrichments: readonly StoredEnrichment[];
}
