import { TypeConverter, type Cell } from "@datasources/core";

export class BoolConverter extends TypeConverter<Cell, Cell> {
  convert = (from: Cell): Cell => (typeof from === "number" ? from !== 0 : from);
  reverseConvert = (to: Cell): Cell => (typeof to === "boolean" ? (to ? 1 : 0) : to);
}
