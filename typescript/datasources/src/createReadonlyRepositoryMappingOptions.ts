import type { Value } from "./converters/Value.ts";
import type { Constructable } from "./converters/Constructable.ts";
import type { TypeConverter } from "./converters/TypeConverter.ts";
import type { ReadonlyRepositoryMappingOptions } from "./ReadonlyRepositoryMappingOptions.ts";

export const fieldNamesOf = <T>(type: Constructable<T>): string[] => {
  const instance = new type() as T & Record<string, T>;
  return Object.keys(instance).filter((key) => !Array.isArray(instance[key]));
};

export const createReadonlyRepositoryMappingOptions = <T>(
  type: Constructable<T>,
  rowTypeConverters?: Record<string, TypeConverter<Value, Value>>,
): ReadonlyRepositoryMappingOptions<T> => ({
  type,
  entity: type.name,
  rowTypeConverters,
});
