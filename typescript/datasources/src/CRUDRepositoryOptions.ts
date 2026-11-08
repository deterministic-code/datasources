import type { ReadonlyRepositoryMappingOptions } from "./ReadonlyRepositoryMappingOptions.ts";
import type { Cells } from "./converters/Value.ts";
import type { IWriteEnrichmentStrategy } from "./IWriteEnrichmentStrategy.ts";

export type CRUDRepositoryOptions<T = Cells> = ReadonlyRepositoryMappingOptions<T> & {
  ids?: string[];
  createProperties?: string[];
  updateProperties?: string[];
  write?: IWriteEnrichmentStrategy<T>;
};
