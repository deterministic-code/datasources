import type { Cell } from "../converters/Cell.ts";
import type { Condition } from "./Condition.ts";
import { Operator } from "./Operator.ts";

export type Comparison = (column: string, value: Cell) => Condition;

export const eq: Comparison = (column, value) => new Operator.Equals(column, value);
export const ne: Comparison = (column, value) => new Operator.NotEquals(column, value);
export const gt: Comparison = (column, value) => new Operator.GreaterThan(column, value);
export const lt: Comparison = (column, value) => new Operator.LessThan(column, value);
export const ge: Comparison = (column, value) => new Operator.GreaterThanOrEqual(column, value);
export const le: Comparison = (column, value) => new Operator.LessThanOrEqual(column, value);
