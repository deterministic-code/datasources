import { DatasourceError } from "../DatasourceError.ts";
import type { Cell } from "./Cell.ts";
import type { FieldConverter } from "./FieldConverter.ts";
import { formatCell } from "./formatCell.ts";

const uuidPattern =
  /^[0-9a-f]{8}-[0-9a-f]{4}-[1-8][0-9a-f]{3}-[89ab][0-9a-f]{3}-[0-9a-f]{12}$/i;

export const uuidFieldConverter: FieldConverter = {
  to: (value) => uuid(value),
  from: (value) => uuid(value),
};

const uuid = (value: Cell): Cell => {
  if (value === null) return null;
  if (typeof value === "string" && uuidPattern.test(value)) return value;
  throw new DatasourceError(`Cannot convert ${formatCell(value)} to UUID`);
};
