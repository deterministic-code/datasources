import { TypeConverter, type Cell } from "@deterministic-code/datasources";

export class UuidConverter extends TypeConverter<Cell, Cell> {
  convert = (from: Cell): Cell => from;
  reverseConvert = (to: Cell): Cell => to;
}
