import { TypeConverter, type Value } from "@deterministic-code/datasources";

export class BinaryConverter extends TypeConverter<Value, Value> {
  convert = (from: Value): Value => from;
  reverseConvert = (to: Value): Value => to;
}
