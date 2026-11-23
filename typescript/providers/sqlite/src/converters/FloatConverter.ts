import { TypeConverter, type Value } from "@deterministic-code/datasources";

const finite = (text: string): number | undefined => {
  if (text.trim() === "") return undefined;
  const number = Number(text);
  return Number.isFinite(number) ? number : undefined;
};

export class FloatConverter extends TypeConverter<Value, Value> {
  convert = (from: Value): Value => {
    if (typeof from === "number") return from;
    if (typeof from === "bigint") return Number(from);
    if (typeof from === "string") {
      const number = finite(from);
      if (number !== undefined) return number;
    }
    return from;
  };

  reverseConvert = (to: Value): Value => to;
}
