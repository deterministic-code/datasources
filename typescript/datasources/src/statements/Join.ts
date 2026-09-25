import { Identifier } from "./Identifier.ts";
import type { IExpression } from "./IExpression.ts";
import { Operator } from "./Operator.ts";

export type JoinKind = "inner" | "left" | "right" | "full" | "cross";

export type Join = {
  kind: JoinKind;
  table: string;
  on?: IExpression;
};

const equality = (left: string, right: string): IExpression =>
  new Operator.Equals(new Identifier(left), new Identifier(right));

export const Join = {
  inner: (table: string, left: string, right: string): Join =>
    ({ kind: "inner", table, on: equality(left, right) }),
  left: (table: string, left: string, right: string): Join =>
    ({ kind: "left", table, on: equality(left, right) }),
  right: (table: string, left: string, right: string): Join =>
    ({ kind: "right", table, on: equality(left, right) }),
  full: (table: string, left: string, right: string): Join =>
    ({ kind: "full", table, on: equality(left, right) }),
  cross: (table: string): Join => ({ kind: "cross", table }),
};
