import { TypeConverter, type Cell } from "@deterministic-code/datasources";

const text = (value: Cell): Cell =>
  value instanceof Uint8Array ? Buffer.from(value).toString("base64") : value;

const bytes = (value: Cell): Cell =>
  typeof value === "string" ? Uint8Array.from(Buffer.from(value, "base64")) : value;

export class BinaryConverter extends TypeConverter<Cell, Cell> {
  convert = (from: Cell): Cell => text(from);
  reverseConvert = (to: Cell): Cell => bytes(to);
}
