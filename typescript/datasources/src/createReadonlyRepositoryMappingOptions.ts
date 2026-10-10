import type { Cell } from "./converters/Cell.ts";
import type { Constructable } from "./converters/Constructable.ts";
import type { TypeConverter } from "./converters/TypeConverter.ts";
import type { ReadonlyRepositoryMappingOptions } from "./ReadonlyRepositoryMappingOptions.ts";

type FieldSource = Record<string, string | number | boolean | bigint | null | object>;

export const fieldNamesOf = <T>(type: Constructable<T>): string[] => {
  const instance = new type() as FieldSource;
  return Object.keys(instance).filter((key) => {
    const value = instance[key];
    if (Array.isArray(value)) return false;
    if (value !== null && typeof value === "object" && !(value instanceof Date) && !(value instanceof Uint8Array))
      return "id" in value || "Id" in value;
    return true;
  });
};

export const createReadonlyRepositoryMappingOptions = <T>(
  type: Constructable<T>,
  rowTypeConverters?: Record<string, TypeConverter<Cell, Cell>>,
): ReadonlyRepositoryMappingOptions<T> => ({
  type,
  entity: type.name,
  rowTypeConverters,
});
