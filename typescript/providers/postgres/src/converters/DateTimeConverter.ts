import { TypeConverter, type Cell } from "@datasources/core";

export class DateTimeConverter extends TypeConverter<Cell, Cell> {
  convert = (from: Cell): Cell =>
    typeof from === "string" ? new Date(from) : from;
  reverseConvert = (to: Cell): Cell => to;
}
