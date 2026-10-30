import { type Cell, type TypeConverter } from "@deterministic-code/datasources";
import { BinaryConverter } from "./BinaryConverter.ts";
import { BoolConverter } from "./BoolConverter.ts";
import { DateTimeConverter } from "./DateTimeConverter.ts";
import { DecimalConverter } from "./DecimalConverter.ts";
import { IntegerConverter } from "./IntegerConverter.ts";
import { UuidConverter } from "./UuidConverter.ts";

export const MysqlRowTypeConverters: Record<string, TypeConverter<Cell, Cell>> = {
  boolean: new BoolConverter(),
  datetime: new DateTimeConverter(),
  uuid: new UuidConverter(),
  decimal: new DecimalConverter(),
  binary: new BinaryConverter(),
  integer: new IntegerConverter(),
};

export const reverse = (value: Cell): Cell =>
  typeof value === "boolean"
    ? MysqlRowTypeConverters.boolean.reverseConvert(value)
    : value instanceof Date
      || (typeof value === "string" && /^\d{4}-\d{2}-\d{2}T/.test(value))
      ? MysqlRowTypeConverters.datetime.reverseConvert(value)
      : value instanceof Uint8Array
        ? MysqlRowTypeConverters.binary.reverseConvert(value)
        : value;

