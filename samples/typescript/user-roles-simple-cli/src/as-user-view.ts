import type { Cell } from "@datasources/core";
import type { GroupView } from "./group-view.ts";
import type { RoleView } from "./role-view.ts";
import type { UserView } from "./user-view.ts";

const id = (value: Cell | undefined): number =>
  typeof value === "number"
    ? value
    : typeof value === "bigint"
      ? Number(value)
      : typeof value === "string" && !Number.isNaN(Number(value))
        ? Number(value)
        : 0;

const text = (value: Cell | undefined): string =>
  value instanceof Date ? value.toISOString() : value == null ? "" : String(value);

const stamp = (value: Cell | undefined): string =>
  typeof value === "string" && /^\d{4}-\d{2}-\d{2} /.test(value)
    ? new Date(`${value.replace(" ", "T")}Z`).toISOString()
    : text(value);

const flag = (value: Cell | undefined): boolean =>
  typeof value === "boolean" ? value : typeof value === "number" || typeof value === "bigint" ? value != 0 : false;

const child = <T extends RoleView | GroupView>(value: T): T => ({
  id: id(value.id),
  name: text(value.name),
  is_enabled: flag(value.is_enabled),
}) as T;

export const asUserView = (rows: UserView[]): UserView[] =>
  [...rows]
    .map((row) => ({
      id: id(row.id),
      username: text(row.username),
      is_enabled: flag(row.is_enabled),
      created: stamp(row.created),
      roles: [...(row.roles ?? [])].map(child).sort((a, b) => a.id - b.id),
      groups: [...(row.groups ?? [])].map(child).sort((a, b) => a.id - b.id),
    }))
    .sort((a, b) => a.id - b.id);
