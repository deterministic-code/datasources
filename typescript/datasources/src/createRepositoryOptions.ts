import type { Cell } from "./converters/Cell.ts";
import type { Constructable } from "./converters/Constructable.ts";
import type { TypeConverter } from "./converters/TypeConverter.ts";
import type { RepositoryOptions } from "./RepositoryOptions.ts";

type FieldSource = Record<string, string | number | boolean | bigint | null | object>;

export const fieldNamesOf = <T>(type: Constructable<T>): string[] => {
  const instance = new type() as FieldSource;
  return Object.keys(instance).filter((key) => !Array.isArray(instance[key]));
};

export const defaultId = (fields: string[]): string[] =>
  fields.includes("id") ? ["id"] : fields.includes("Id") ? ["Id"] : [];

export const createRepositoryOptions = <T>(
  type: Constructable<T>,
  rowTypeConverters?: Record<string, TypeConverter<Cell, Cell>>,
): RepositoryOptions<T> => ({
  type,
  ids: defaultId(fieldNamesOf(type)),
  entity: type.name,
  rowTypeConverters,
});
