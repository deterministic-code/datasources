import { type Cell, type TypeConverter } from "@datasources/core";
import { BinaryConverter } from "./BinaryConverter.ts";
import { BoolConverter } from "./BoolConverter.ts";
import { DateTimeConverter } from "./DateTimeConverter.ts";
import { DecimalConverter } from "./DecimalConverter.ts";
import { UuidConverter } from "./UuidConverter.ts";

export const MysqlTypes: Record<string, TypeConverter<Cell, Cell>> = {
  bool: new BoolConverter(),
  datetime: new DateTimeConverter(),
  uuid: new UuidConverter(),
  decimal: new DecimalConverter(),
  binary: new BinaryConverter(),
};

