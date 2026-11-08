import { TypeConverter, type Value } from "@deterministic-code/datasources";

export const isBase64 = (value: string): boolean => {
  const text = value.trim();
  return (
    text.length >= 4 &&
    text.length % 4 === 0 &&
    text.includes("=") &&
    /^[A-Za-z0-9+/]+=*$/.test(text) &&
    Buffer.from(text, "base64").toString("base64") === text
  );
};

const bytes = (value: Value): Value =>
  typeof value === "string" ? Uint8Array.from(Buffer.from(value, "base64")) : value;

export class BinaryConverter extends TypeConverter<Value, Value> {
  convert = (from: Value): Value => bytes(from);
  reverseConvert = (to: Value): Value => bytes(to);
}
