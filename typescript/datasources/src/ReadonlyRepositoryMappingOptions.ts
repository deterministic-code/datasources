import type { Cell, Cells } from "./converters/Cell.ts";
import type { Constructable } from "./converters/Constructable.ts";
import type { TypeConverter } from "./converters/TypeConverter.ts";
import type { IMapper } from "./mappers/IMapper.ts";
import type { IReadonlyEnrichmentStrategy } from "./IReadonlyEnrichmentStrategy.ts";

export type ReadonlyRepositoryMappingOptions<T = Cells> = {
  entity?: string;
  entityMapper?: IMapper<string, string>;
  propertyMappings?: IMapper<string, string>;
  rowConverter?: TypeConverter<Cells, Cells>;
  objectConverter?: TypeConverter<T, Cells>;
  type?: Constructable<T>;
  rowTypeConverters?: Record<string, TypeConverter<Cell, Cell>>;
  readProperties?: string[];
  readEnrichment?: IReadonlyEnrichmentStrategy<T & object>;
};
