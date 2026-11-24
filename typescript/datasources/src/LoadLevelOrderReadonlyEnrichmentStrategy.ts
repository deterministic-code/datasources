import { DatasourceError } from "./DatasourceError.ts";
import { DefaultReadonlyEnrichmentStrategy } from "./DefaultReadonlyEnrichmentStrategy.ts";

export class LoadLevelOrderReadonlyEnrichmentStrategy<T extends object> extends DefaultReadonlyEnrichmentStrategy<T> {
  constructor(batchSize = 500) {
    super();
    if (batchSize < 1) throw new DatasourceError("batchSize must be at least 1");
    this.batchSize = batchSize;
  }

  withBatchSize = (batchSize: number): LoadLevelOrderReadonlyEnrichmentStrategy<T> =>
    new LoadLevelOrderReadonlyEnrichmentStrategy<T>(batchSize);
}
