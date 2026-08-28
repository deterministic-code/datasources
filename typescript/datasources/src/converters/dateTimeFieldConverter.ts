import { DatasourceError } from "../DatasourceError.ts";
import type { FieldConverter } from "./FieldConverter.ts";
import { formatCell } from "./formatCell.ts";

export const dateTimeFieldConverter: FieldConverter = {
  to: (value) => {
    if (value === null) return null;
    if (value instanceof Date) return isValidDate(value);
    if (typeof value === "string") return parseIso(value);
    throw new DatasourceError(`Cannot convert ${formatCell(value)} to Date`);
  },
  from: (value) => {
    if (value === null) return null;
    if (value instanceof Date) return isValidDate(value).toISOString();
    if (typeof value === "string") return parseIso(value).toISOString();
    throw new DatasourceError(`Cannot convert ${formatCell(value)} to ISO 8601`);
  },
};

const parseIso = (value: string): Date => {
  const date = new Date(value);
  if (Number.isNaN(date.getTime()))
    throw new DatasourceError(`Cannot convert ${formatCell(value)} to Date`);
  return date;
};

const isValidDate = (value: Date): Date => {
  if (Number.isNaN(value.getTime()))
    throw new DatasourceError(`Cannot convert ${formatCell(value)} to Date`);
  return value;
};
