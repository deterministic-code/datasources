import { TypeConverter, type Cell } from "@deterministic-code/datasources";

export class IntegerConverter extends TypeConverter<Cell, Cell> {
  convert = (from: Cell): Cell =>
    typeof from === "string" && from.length > 0 && /^-?\d+$/.test(from) ? Number(from) : from;

  reverseConvert = (to: Cell): Cell => to;
}
