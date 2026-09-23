import { type Cell, type TypeConverter } from "@datasources/core";
import { BinaryConverter } from "./BinaryConverter.ts";
import { BoolConverter } from "./BoolConverter.ts";
import { DateTimeConverter } from "./DateTimeConverter.ts";
import { DecimalConverter } from "./DecimalConverter.ts";
import { UuidConverter } from "./UuidConverter.ts";

export const MysqlRowTypeConverters: Record<string, TypeConverter<Cell, Cell>> = {
  bool: new BoolConverter(),
  datetime: new DateTimeConverter(),
  uuid: new UuidConverter(),
  decimal: new DecimalConverter(),
  binary: new BinaryConverter(),
};

export const reverse = (value: Cell): Cell =>
  typeof value === "boolean"
    ? MysqlRowTypeConverters.bool.reverseConvert(value)
    : value instanceof Date
      ? MysqlRowTypeConverters.datetime.reverseConvert(value)
      : value instanceof Uint8Array
        ? MysqlRowTypeConverters.binary.reverseConvert(value)
        : value;

