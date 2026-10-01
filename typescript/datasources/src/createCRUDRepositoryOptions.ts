import type { Cell } from "./converters/Cell.ts";
import type { Constructable } from "./converters/Constructable.ts";
import type { TypeConverter } from "./converters/TypeConverter.ts";
import type { CRUDRepositoryOptions } from "./CRUDRepositoryOptions.ts";
import {
  createReadonlyRepositoryOptions,
  fieldNamesOf,
} from "./createReadonlyRepositoryOptions.ts";

export { fieldNamesOf } from "./createReadonlyRepositoryOptions.ts";

export const defaultId = (fields: string[]): string[] =>
  fields.includes("id") ? ["id"] : fields.includes("Id") ? ["Id"] : [];

export const createCRUDRepositoryOptions = <T>(
  type: Constructable<T>,
  rowTypeConverters?: Record<string, TypeConverter<Cell, Cell>>,
): CRUDRepositoryOptions<T> => ({
  ...createReadonlyRepositoryOptions(type, rowTypeConverters),
  ids: defaultId(fieldNamesOf(type)),
});
