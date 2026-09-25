import type { IReadonlyEnrichmentBinder } from "./IReadonlyEnrichmentBinder.ts";

export interface IReadonlyEnrichmentStrategy<T extends object>
  extends IReadonlyEnrichmentBinder {}
