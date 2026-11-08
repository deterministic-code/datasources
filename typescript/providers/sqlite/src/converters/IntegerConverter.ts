import { TypeConverter, type Value } from "@deterministic-code/datasources";

export class IntegerConverter extends TypeConverter<Value, Value> {
  convert = (from: Value): Value =>
    from === "" ? 0 : typeof from === "string" && /^-?\d+$/.test(from) ? Number(from) : from;

  reverseConvert = (to: Value): Value => to;
}
