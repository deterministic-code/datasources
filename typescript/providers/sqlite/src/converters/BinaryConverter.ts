import { TypeConverter, type Cell } from "@deterministic-code/datasources";

export class BinaryConverter extends TypeConverter<Cell, Cell> {
  convert = (from: Cell): Cell => from;
  reverseConvert = (to: Cell): Cell => to;
}
