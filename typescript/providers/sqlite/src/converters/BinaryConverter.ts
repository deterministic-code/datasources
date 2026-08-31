import { TypeConverter, type Cell } from "@datasources/core";

export class BinaryConverter extends TypeConverter<Cell, Cell> {
  convert = (from: Cell): Cell => from;
  reverseConvert = (to: Cell): Cell => to;
}
