export type JoinKind = "inner" | "left" | "right";

export type Join = {
  kind: JoinKind;
  table: string;
  left: string;
  right: string;
};

export const Join = {
  inner: (table: string, left: string, right: string): Join =>
    ({ kind: "inner", table, left, right }),
  left: (table: string, left: string, right: string): Join =>
    ({ kind: "left", table, left, right }),
  right: (table: string, left: string, right: string): Join =>
    ({ kind: "right", table, left, right }),
};
