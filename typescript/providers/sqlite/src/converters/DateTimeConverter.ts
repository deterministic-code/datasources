import { TypeConverter, type Value } from "@deterministic-code/datasources";

export class DateTimeConverter extends TypeConverter<Value, Value> {
  convert = (from: Value): Value =>
    typeof from === "string" ? new Date(from) : from;
  reverseConvert = (to: Value): Value =>
    to instanceof Date ? to.toISOString() : to;
}
