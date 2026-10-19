import { TypeConverter, type Cell } from "@deterministic-code/datasources";

export class BoolConverter extends TypeConverter<Cell, Cell> {
  convert = (from: Cell): Cell => (typeof from === "number" ? from !== 0 : from);
  reverseConvert = (to: Cell): Cell => to;
}
