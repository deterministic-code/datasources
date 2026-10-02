import type { ReadonlyRepositoryMappingOptions } from "./ReadonlyRepositoryMappingOptions.ts";
import type { Cells } from "./converters/Cell.ts";
import type { IWriteEnrichmentStrategy } from "./IWriteEnrichmentStrategy.ts";

export type CRUDRepositoryOptions<T = Cells> = ReadonlyRepositoryMappingOptions<T> & {
  ids?: string[];
  write?: IWriteEnrichmentStrategy<T>;
};
