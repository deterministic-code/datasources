import { TypeConverter, type Value } from "@deterministic-code/datasources";

const stamp = (date: Date): string =>
  date.toISOString().replace("T", " ").replace("Z", "");

const utcDate = (text: string): Date | string => {
  const iso = /[zZ]|[+-]\d{2}:\d{2}$/.test(text)
    ? text
    : `${text.includes("T") ? text : text.replace(" ", "T")}Z`;
  const date = new Date(iso);
  return Number.isNaN(date.getTime()) ? text : date;
};

export class DateTimeConverter extends TypeConverter<Value, Value> {
  convert = (from: Value): Value =>
    typeof from === "string" ? utcDate(from) : from;
  reverseConvert = (to: Value): Value =>
    to instanceof Date
      ? stamp(to)
      : typeof to === "string" && /^\d{4}-\d{2}-\d{2}T/.test(to)
        ? stamp(new Date(to))
        : to;
}
