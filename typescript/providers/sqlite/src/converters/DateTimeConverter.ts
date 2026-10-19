import { TypeConverter, type Cell } from "@deterministic-code/datasources";

export class DateTimeConverter extends TypeConverter<Cell, Cell> {
  convert = (from: Cell): Cell =>
    typeof from === "string" ? new Date(from) : from;
  reverseConvert = (to: Cell): Cell =>
    to instanceof Date ? to.toISOString() : to;
}
