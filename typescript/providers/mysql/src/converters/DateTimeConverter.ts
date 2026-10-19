import { TypeConverter, type Cell } from "@deterministic-code/datasources";

const stamp = (date: Date): string =>
  date.toISOString().replace("T", " ").replace("Z", "");

export class DateTimeConverter extends TypeConverter<Cell, Cell> {
  convert = (from: Cell): Cell =>
    typeof from === "string" ? new Date(from) : from;
  reverseConvert = (to: Cell): Cell =>
    to instanceof Date
      ? stamp(to)
      : typeof to === "string" && /^\d{4}-\d{2}-\d{2}T/.test(to)
        ? stamp(new Date(to))
        : to;
}
