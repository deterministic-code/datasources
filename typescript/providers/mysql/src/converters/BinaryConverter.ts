import { TypeConverter, type Cell } from "@deterministic-code/datasources";

const bytes = (value: Cell): Cell =>
  typeof value === "string" ? Uint8Array.from(Buffer.from(value, "base64")) : value;

export class BinaryConverter extends TypeConverter<Cell, Cell> {
  convert = (from: Cell): Cell => bytes(from);
  reverseConvert = (to: Cell): Cell => bytes(to);
}
