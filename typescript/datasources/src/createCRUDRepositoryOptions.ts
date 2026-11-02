import type { Cell } from "./converters/Cell.ts";
import type { Constructable } from "./converters/Constructable.ts";
import type { TypeConverter } from "./converters/TypeConverter.ts";
import type { CRUDRepositoryOptions } from "./CRUDRepositoryOptions.ts";
import { createReadonlyRepositoryMappingOptions, fieldNamesOf } from "./createReadonlyRepositoryMappingOptions.ts";
import { idsFromSchema } from "./identity.ts";
import type { Schema } from "./Schema.ts";

export { fieldNamesOf };

export const defaultId = (schema?: Schema, names: readonly string[] = []): string[] => {
  const fromSchema = idsFromSchema(schema);
  if (fromSchema.length > 0) return fromSchema;
  if (names.includes("id") || names.length === 0) return ["id"];
  if (names.includes("Id")) return ["Id"];
  return [];
};

export const createCRUDRepositoryOptions = <T>(
  type: Constructable<T>,
  rowTypeConverters?: Record<string, TypeConverter<Cell, Cell>>,
): CRUDRepositoryOptions<T> => {
  const read = createReadonlyRepositoryMappingOptions(type, rowTypeConverters);
  return {
    ...read,
    ids: defaultId(read.schema, fieldNamesOf(type)),
  };
};
