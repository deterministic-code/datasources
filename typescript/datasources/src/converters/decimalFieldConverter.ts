import { DatasourceError } from "../DatasourceError.ts";
import type { Cell } from "./Cell.ts";
import type { FieldConverter } from "./FieldConverter.ts";
import { formatCell } from "./formatCell.ts";

export const decimalFieldConverter = (
  size?: readonly [precision: number, scale: number],
): FieldConverter => ({
  to: (value) => decimal(value, size),
  from: (value) => decimal(value, size),
});

const decimal = (
  value: Cell,
  size: readonly [number, number] | undefined,
): Cell => {
  if (value === null) return null;
  const text =
    typeof value === "string"
      ? value
      : typeof value === "number" && Number.isFinite(value)
        ? String(value)
        : typeof value === "bigint"
          ? value.toString()
          : failDecimal(value);
  if (!size) return text;
  const [precision, scale] = size;
  const match = /^(-?)(\d+)(?:\.(\d+))?$/.exec(text);
  if (!match) throw new DatasourceError(`Cannot convert ${formatCell(value)} to decimal`);
  const sign = match[1];
  const whole = match[2];
  const frac = (match[3] ?? "").padEnd(scale, "0").slice(0, scale);
  if (whole.length + (scale > 0 ? frac.length : 0) > precision)
    throw new DatasourceError(`Decimal ${text} exceeds size [${precision}, ${scale}]`);
  return scale > 0 ? `${sign}${whole}.${frac}` : `${sign}${whole}`;
};

const failDecimal = (value: Cell): never => {
  throw new DatasourceError(`Cannot convert ${formatCell(value)} to decimal`);
};
