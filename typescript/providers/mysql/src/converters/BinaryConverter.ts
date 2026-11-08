import { TypeConverter, type Value } from "@deterministic-code/datasources";

const bytes = (value: Value): Value =>
  typeof value === "string" ? Uint8Array.from(Buffer.from(value, "base64")) : value;

export class BinaryConverter extends TypeConverter<Value, Value> {
  convert = (from: Value): Value => bytes(from);
  reverseConvert = (to: Value): Value => bytes(to);
}
