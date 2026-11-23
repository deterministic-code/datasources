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

const hex = (text: string): Uint8Array | undefined => {
  const digits = text.trim();
  if (!/^\\x[0-9a-fA-F]*$/i.test(digits) || (digits.length & 1) !== 0) return undefined;
  return Uint8Array.from(Buffer.from(digits.slice(2), "hex"));
};

const bytes = (value: Value): Value => {
  if (typeof value !== "string") return value;
  if (isBase64(value)) return Uint8Array.from(Buffer.from(value, "base64"));
  return hex(value) ?? Uint8Array.from(Buffer.from(value, "latin1"));
};

export class BinaryConverter extends TypeConverter<Value, Value> {
  convert = (from: Value): Value => bytes(from);
  reverseConvert = (to: Value): Value => bytes(to);
}
