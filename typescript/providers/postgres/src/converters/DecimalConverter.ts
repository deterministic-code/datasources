import { TypeConverter, type Cell } from "@deterministic-code/datasources";

export class DecimalConverter extends TypeConverter<Cell, Cell> {
  convert = (from: Cell): Cell => from;
  reverseConvert = (to: Cell): Cell =>
    typeof to === "number" ? String(to) : to;
}
