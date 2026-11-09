import { type Value, type TypeConverter } from "@deterministic-code/datasources";
import { BinaryConverter } from "./BinaryConverter.ts";
import { BoolConverter } from "./BoolConverter.ts";
import { DateTimeConverter } from "./DateTimeConverter.ts";
import { DecimalConverter } from "./DecimalConverter.ts";
import { IntegerConverter } from "./IntegerConverter.ts";
import { UuidConverter } from "./UuidConverter.ts";

const integer = new IntegerConverter();

export const MysqlRowTypeConverters: Record<string, TypeConverter<Value, Value>> = {
  boolean: new BoolConverter(),
  datetime: new DateTimeConverter(),
  uuid: new UuidConverter(),
  decimal: new DecimalConverter(),
  binary: new BinaryConverter(),
  integer,
  unsignedinteger: integer,
  smallinteger: integer,
  unsignedsmallinteger: integer,
  biginteger: integer,
  unsignedbiginteger: integer,
};

export const reverse = (value: Value): Value =>
  typeof value === "boolean"
    ? MysqlRowTypeConverters.boolean.reverseConvert(value)
    : value instanceof Date
      ? MysqlRowTypeConverters.datetime.reverseConvert(value)
      : value instanceof Uint8Array
        ? MysqlRowTypeConverters.binary.reverseConvert(value)
        : value;

