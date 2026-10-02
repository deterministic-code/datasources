import type { Cell } from "./converters/Cell.ts";
import type { Constructable } from "./converters/Constructable.ts";
import type { TypeConverter } from "./converters/TypeConverter.ts";
import type { CRUDRepositoryOptions } from "./CRUDRepositoryOptions.ts";
import {
  createReadonlyRepositoryMappingOptions,
  fieldNamesOf,
} from "./createReadonlyRepositoryMappingOptions.ts";

export { fieldNamesOf } from "./createReadonlyRepositoryMappingOptions.ts";

export const defaultId = (fields: string[]): string[] =>
  fields.includes("id") ? ["id"] : fields.includes("Id") ? ["Id"] : [];

export const createCRUDRepositoryOptions = <T>(
  type: Constructable<T>,
  rowTypeConverters?: Record<string, TypeConverter<Cell, Cell>>,
): CRUDRepositoryOptions<T> => ({
  ...createReadonlyRepositoryMappingOptions(type, rowTypeConverters),
  ids: defaultId(fieldNamesOf(type)),
});
