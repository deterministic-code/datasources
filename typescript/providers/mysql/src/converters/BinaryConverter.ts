import { TypeConverter, type Value } from "@deterministic-code/datasources";

const isBase64 = (text: string): boolean =>
  text.length % 4 === 0 && /^[A-Za-z0-9+/]*={0,2}$/.test(text);

const bytes = (value: Value): Value =>
  typeof value === "string"
    ? Uint8Array.from(Buffer.from(value, isBase64(value) ? "base64" : "latin1"))
    : value;

export class BinaryConverter extends TypeConverter<Value, Value> {
  convert = (from: Value): Value => bytes(from);
  reverseConvert = (to: Value): Value => bytes(to);
}
