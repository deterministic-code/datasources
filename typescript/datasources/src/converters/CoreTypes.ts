import type { Cell } from "./Cell.ts";
import type { ValueConverter } from "./ValueConverter.ts";

const identity: ValueConverter = {
  convert: (value) => value,
  reverseConvert: (value) => value,
};

const bool: ValueConverter = {
  convert: (value) =>
    typeof value === "boolean"
      ? value
      : typeof value === "bigint"
        ? value !== 0n
        : typeof value === "number"
          ? value !== 0
          : value,
  reverseConvert: (value) => value,
};

const datetime: ValueConverter = {
  convert: (value) =>
    value instanceof Date ? value : typeof value === "string" ? new Date(value) : value,
  reverseConvert: (value) =>
    value instanceof Date ? value.toISOString() : value,
};

const typeName = (value: Cell | undefined): string | undefined =>
  value instanceof Date
    ? "datetime"
    : value instanceof Uint8Array
      ? "binary"
      : typeof value === "boolean"
        ? "bool"
        : undefined;

export const CoreTypes: Record<string, ValueConverter> = {
  bool,
  datetime,
  guid: identity,
  decimal: identity,
  binary: identity,
};

export const typeNameOf = typeName;
