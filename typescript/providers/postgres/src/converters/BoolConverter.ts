import { TypeConverter, type Value } from "@deterministic-code/datasources";

export class BoolConverter extends TypeConverter<Value, Value> {
  convert = (from: Value): Value => (typeof from === "number" ? from !== 0 : from);
  reverseConvert = (to: Value): Value => to;
}
