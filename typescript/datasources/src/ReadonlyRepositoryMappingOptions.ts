import type { Value, Cells } from "./converters/Value.ts";
import type { Constructable } from "./converters/Constructable.ts";
import type { TypeConverter } from "./converters/TypeConverter.ts";
import type { IReadonlyEnrichmentStrategy } from "./IReadonlyEnrichmentStrategy.ts";
import type { IMapper } from "./mappers/IMapper.ts";
import type { Schema } from "./Schema.ts";

export type ReadonlyRepositoryMappingOptions<T = Cells> = {
  entity?: string;
  entityMapper?: IMapper<string, string>;
  propertyMappings?: IMapper<string, string>;
  rowConverter?: TypeConverter<Cells, Cells>;
  objectConverter?: TypeConverter<T, Cells>;
  type?: Constructable<T>;
  rowTypeConverters?: Record<string, TypeConverter<Value, Value>>;
  readProperties?: string[];
  schema?: Schema;
  enrichment?: IReadonlyEnrichmentStrategy<T & object>;
};
