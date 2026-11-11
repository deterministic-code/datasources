import type { Value, Cells } from "../converters/Value.ts";
import type { IFlatToNestedMapper } from "./IFlatToNestedMapper.ts";
import type { TypeMap, TypeMapFields } from "./TypeMap.ts";
import type { TypeMapDictionary } from "./TypeMapDictionary.ts";
import type { TypeMapGroup } from "./TypeMapGroup.ts";

type GroupPlan = { key: string; fields: Record<string, string>; destKey: string };
type DictPlan = { key: string; value: string };

const stringFields = (fields: TypeMapFields<object>): Record<string, string> =>
  Object.fromEntries(
    Object.entries(fields).filter((entry): entry is [string, string] => typeof entry[1] === "string"),
  );

const destOf = (fields: Record<string, string>, column: string) =>
  Object.entries(fields).find(([, source]) => source === column)?.[0] ?? column;

const cellKey = (value: Value | undefined) => (value == null ? null : String(value));

export class FlatToNestedMapper<T> implements IFlatToNestedMapper<T> {
  readonly spec: TypeMap<T>;
  private readonly scalars: Record<string, string>;
  private readonly groups: Record<string, GroupPlan>;
  private readonly dictionaries: Record<string, DictPlan>;
  private readonly parentDest: string;

  constructor(spec: TypeMap<T>) {
    this.spec = spec;
    this.scalars = {};
    this.groups = {};
    this.dictionaries = {};
    for (const [name, value] of Object.entries(spec.fields) as [
      string,
      string | TypeMapGroup<object> | TypeMapDictionary,
    ][]) {
      if (typeof value === "string") this.scalars[name] = value;
      else if ("fields" in value) {
        const fields = stringFields(value.fields);
        this.groups[name] = { key: value.key, fields, destKey: destOf(fields, value.key) };
      } else this.dictionaries[name] = { key: value.key, value: value.value };
    }
    this.parentDest = destOf(this.scalars, spec.key);
  }

  map = (rows: Iterable<T | Cells>): T[] => this._collapse(rows);

  reverseMap = (values: T[]): Cells[] => [...values].flatMap((value) => this.explode(value));

  private _map = (row: Cells): T => {
    const dest = {} as Record<string, Value | Cells[] | Record<string, Value>>;
    for (const [name, column] of Object.entries(this.scalars)) dest[name] = row[column] ?? null;
    for (const [name, group] of Object.entries(this.groups)) {
      const key = row[group.key];
      dest[name] =
        key == null
          ? []
          : [
              Object.fromEntries(
                Object.entries(group.fields).map(([field, column]) => [field, row[column] ?? null]),
              ),
            ];
    }
    for (const [name, dict] of Object.entries(this.dictionaries)) {
      const key = cellKey(row[dict.key]);
      dest[name] = key == null ? {} : { [key]: row[dict.value] ?? null };
    }
    return dest as T;
  };

  private _collapse = (rows: Iterable<T | Cells>): T[] => {
    const byParent = new Map<string, T>();
    const seen = new Set<string>();
    for (const raw of rows) {
      const item = this.mapped(raw) ? raw : this._map(raw);
      const parent = cellKey((item as Record<string, Value>)[this.parentDest]);
      if (parent == null) continue;
      const existing = byParent.get(parent);
      if (!existing) {
        byParent.set(parent, item);
        this.mark(seen, parent, item);
        continue;
      }
      this.merge(existing, item, parent, seen);
    }
    return [...byParent.values()];
  };

  private explode = (value: T): Cells[] => {
    const record = value as Record<string, Value | Cells[] | Record<string, Value>>;
    let rows: Cells[] = [this.scalarsOf(record)];
    for (const [name, group] of Object.entries(this.groups)) {
      const children = (record[name] as Cells[] | undefined)?.length
        ? (record[name] as Cells[])
        : [undefined];
      rows = rows.flatMap((row) => children.map((child) => this.withChild(row, group, child)));
    }
    for (const [name, dict] of Object.entries(this.dictionaries)) {
      const entries = Object.entries(record[name] as Record<string, Value>);
      const children = entries.length ? entries : [[undefined, undefined] as [string | undefined, Value | undefined]];
      rows = rows.flatMap((row) => children.map(([key, cell]) => this.withEntry(row, dict, key, cell)));
    }
    return rows;
  };

  private scalarsOf = (record: Record<string, Value | Cells[] | Record<string, Value>>): Cells => {
    const row: Cells = {};
    for (const [name, column] of Object.entries(this.scalars))
      row[column] = (record[name] as Value | undefined) ?? null;
    return row;
  };

  private withChild = (row: Cells, group: GroupPlan, child: Cells | undefined): Cells => {
    const next = { ...row };
    for (const [field, column] of Object.entries(group.fields)) next[column] = child?.[field] ?? null;
    return next;
  };

  private withEntry = (row: Cells, dict: DictPlan, key: string | undefined, cell: Value | undefined): Cells => ({
    ...row,
    [dict.key]: key ?? null,
    [dict.value]: cell ?? null,
  });

  private mapped = (row: T | Cells): row is T =>
    Object.keys(this.groups).every((name) => Array.isArray((row as T & Record<string, Cells[]>)[name])) &&
    Object.keys(this.dictionaries).every((name) => {
      const dest = (row as Record<string, unknown>)[name];
      return dest instanceof Object && !Array.isArray(dest);
    });

  private mark = (seen: Set<string>, parent: string, item: T) => {
    const record = item as Record<string, Cells[] | Record<string, Value>>;
    for (const [name, group] of Object.entries(this.groups))
      for (const child of record[name] as Cells[]) {
        const key = cellKey(child[group.destKey]);
        if (key != null) seen.add(`${parent}:${name}:${key}`);
      }
    for (const name of Object.keys(this.dictionaries))
      for (const key of Object.keys(record[name] as Record<string, Value>))
        seen.add(`${parent}:${name}:${key}`);
  };

  private merge = (into: T, from: T, parent: string, seen: Set<string>) => {
    const dest = into as Record<string, Cells[] | Record<string, Value>>;
    const source = from as Record<string, Cells[] | Record<string, Value>>;
    for (const [name, group] of Object.entries(this.groups)) {
      const children = dest[name] as Cells[];
      for (const child of source[name] as Cells[]) {
        const key = cellKey(child[group.destKey]);
        if (key == null || seen.has(`${parent}:${name}:${key}`)) continue;
        seen.add(`${parent}:${name}:${key}`);
        children.push(child);
      }
    }
    for (const name of Object.keys(this.dictionaries)) {
      const entries = dest[name] as Record<string, Value>;
      for (const [key, cell] of Object.entries(source[name] as Record<string, Value>)) {
        if (seen.has(`${parent}:${name}:${key}`)) continue;
        seen.add(`${parent}:${name}:${key}`);
        entries[key] = cell;
      }
    }
  };
}
