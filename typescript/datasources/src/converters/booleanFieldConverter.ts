import { DatasourceError } from "../DatasourceError.ts";
import type { Cell } from "./Cell.ts";
import type { FieldConverter } from "./FieldConverter.ts";
import { formatCell } from "./formatCell.ts";

export const booleanFieldConverter = (
  storage: "integer" | "native" = "integer",
): FieldConverter => ({
  to: (value) => {
    if (value === null) return null;
    if (value === true || value === 1 || value === 1n) return true;
    if (value === false || value === 0 || value === 0n) return false;
    throw new DatasourceError(`Cannot convert ${formatCell(value)} to boolean`);
  },
  from: (value) => {
    if (value === null) return null;
    const flag =
      value === true || value === 1 || value === 1n
        ? true
        : value === false || value === 0 || value === 0n
          ? false
          : throwBool(value);
    return storage === "native" ? flag : flag ? 1 : 0;
  },
});

const throwBool = (value: Cell): never => {
  throw new DatasourceError(`Cannot convert ${formatCell(value)} to boolean`);
};
