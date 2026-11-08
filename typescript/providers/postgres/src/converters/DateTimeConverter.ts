import { TypeConverter, type Value } from "@deterministic-code/datasources";

export const isIsoDate = (value: string): boolean =>
  /^\d{4}-\d{2}-\d{2}T/.test(value) && !Number.isNaN(Date.parse(value));

export class DateTimeConverter extends TypeConverter<Value, Value> {
  convert = (from: Value): Value =>
    typeof from === "string" ? new Date(from) : from;
  reverseConvert = (to: Value): Value =>
    to instanceof Date ? to.toISOString() : to;
}
