import type { ReadonlyRepositoryOptions } from "./ReadonlyRepositoryOptions.ts";
import type { Cells } from "./converters/Cell.ts";
import type { IWriteEnrichmentStrategy } from "./IWriteEnrichmentStrategy.ts";

export type CRUDRepositoryOptions<T = Cells> = ReadonlyRepositoryOptions<T> & {
  ids?: string[];
  write?: IWriteEnrichmentStrategy<T>;
};
