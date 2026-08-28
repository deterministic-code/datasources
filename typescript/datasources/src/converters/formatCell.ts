import type { Cell } from "./Cell.ts";

export const formatCell = (value: Cell): string =>
  value instanceof Date
    ? Number.isNaN(value.getTime())
      ? "Invalid Date"
      : value.toISOString()
    : value instanceof Uint8Array
      ? "Uint8Array"
      : typeof value === "string"
        ? `"${value}"`
        : String(value);
