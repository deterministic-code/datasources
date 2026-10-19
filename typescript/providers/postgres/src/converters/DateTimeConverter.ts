import { TypeConverter, type Cell } from "@deterministic-code/datasources";

export const isIsoDate = (value: string): boolean =>
  /^\d{4}-\d{2}-\d{2}T/.test(value) && !Number.isNaN(Date.parse(value));

export class DateTimeConverter extends TypeConverter<Cell, Cell> {
  convert = (from: Cell): Cell =>
    typeof from === "string" ? new Date(from) : from;
  reverseConvert = (to: Cell): Cell =>
    to instanceof Date ? to.toISOString() : to;
}
