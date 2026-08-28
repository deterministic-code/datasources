import type { Cell } from "./Cell.ts";
import type { FieldConverter } from "./FieldConverter.ts";

const pass = (value: Cell): Cell => value;

export const identityFieldConverter: FieldConverter = {
  to: pass,
  from: pass,
};
