import { DefaultWriteEnrichmentStrategy } from "./DefaultWriteEnrichmentStrategy.ts";

export class LazyWriteEnrichmentStrategy<T> extends DefaultWriteEnrichmentStrategy<T> {
  protected override skipUnloaded = true;
}
