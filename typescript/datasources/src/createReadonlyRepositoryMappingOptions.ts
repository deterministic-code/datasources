import type { Cell } from "./converters/Cell.ts";
import type { Constructable } from "./converters/Constructable.ts";
import type { TypeConverter } from "./converters/TypeConverter.ts";
import { DeterministicTypeMapper } from "./mappers/DeterministicTypeMapper.ts";
import type { ReadonlyRepositoryMappingOptions } from "./ReadonlyRepositoryMappingOptions.ts";
import { Schema, type SchemaColumn } from "./Schema.ts";

type FieldSource = Record<string, string | number | boolean | bigint | null | object>;

const types = new DeterministicTypeMapper();

export const fieldNamesOf = <T>(type: Constructable<T>): string[] => {
  const instance = new type() as FieldSource;
  return Object.keys(instance).filter((key) => {
    const value = instance[key];
    if (Array.isArray(value)) return false;
    if (value !== null && typeof value === "object" && !(value instanceof Date) && !(value instanceof Uint8Array))
      return "Id" in value;
    return true;
  });
};

const columnOf = (value: FieldSource[string]): SchemaColumn | undefined => {
  if (typeof value === "string") return { type: "string", nullable: false, hasDefault: false };
  if (value === null) return undefined;
  const mapped = value instanceof Date
    ? types.map("Date")
    : value instanceof Uint8Array
      ? types.map("Uint8Array")
      : types.map(typeof value === "object" ? value.constructor.name : typeof value);
  return mapped.length === 0 ? undefined : { type: mapped, nullable: false, hasDefault: false };
};

export const schemaOf = <T>(type: Constructable<T>): Schema => {
  const instance = new type() as FieldSource;
  return new Schema(
    Object.fromEntries(
      fieldNamesOf(type).flatMap((name) => {
        const column = columnOf(instance[name]);
        return column === undefined ? [] : [[name, column]];
      }),
    ),
  );
};

export const createReadonlyRepositoryMappingOptions = <T>(
  type: Constructable<T>,
  rowTypeConverters?: Record<string, TypeConverter<Cell, Cell>>,
): ReadonlyRepositoryMappingOptions<T> => ({
  type,
  entity: type.name,
  schema: schemaOf(type),
  rowTypeConverters,
});
