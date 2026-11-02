import { TypeConverter, type Cell } from "@deterministic-code/datasources";

const decimalText = (text: string): string => {
  const negative = text.startsWith("-");
  const body = negative ? text.slice(1) : text;
  const [whole, frac = ""] = body.split(".");
  const trimmedFrac = frac.replace(/0+$/, "");
  const trimmedWhole = whole.replace(/^0+(?=\d)/, "");
  const digits = trimmedFrac ? `${trimmedWhole}.${trimmedFrac}` : trimmedWhole;
  if (digits === "0") return "0";
  return negative ? `-${digits}` : digits;
};

const canonicalDecimal = (from: Cell): Cell => {
  if (typeof from === "number") {
    if (!Number.isFinite(from)) return from;
    const text = String(from);
    return /e/i.test(text) ? text : decimalText(text);
  }
  if (typeof from === "bigint") return decimalText(from.toString());
  if (typeof from === "string" && /^-?\d+(\.\d+)?$/.test(from)) return decimalText(from);
  return from;
};

export class DecimalConverter extends TypeConverter<Cell, Cell> {
  convert = (from: Cell): Cell => canonicalDecimal(from);
  reverseConvert = (to: Cell): Cell => canonicalDecimal(to);
}
