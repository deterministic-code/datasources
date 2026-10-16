import { isDeepStrictEqual } from "node:util";

export type Json = string | number | boolean | null | Json[] | { [key: string]: Json };

type JsonObject = { [key: string]: Json };

type KeyedSplit = {
  added?: JsonObject[];
  changed?: JsonObject[];
  removed?: string[];
};

export const apply = (before: Json, next: Json): Json => {
  if (next == null) return before;
  if (isKeyedCollection(before) && isKeyedSplit(next)) return applyKeyed(before, next);
  if (isObject(before) && isObject(next)) return applyObject(before, next);
  return next;
};

export const applyFile = (before: Json | undefined, envelope: Json): Json | undefined => {
  if (envelope == null) return before;
  if (isObject(envelope)) {
    if (envelope.removed === true && Object.keys(envelope).length === 1) return undefined;
    if (envelope.added === true) return envelope.content;
  }
  return apply(before ?? null, envelope);
};

export const applyDocument = (
  before: { [path: string]: Json },
  doc: { [path: string]: Json },
): { [path: string]: Json } => {
  const out = { ...before };
  for (const [path, envelope] of Object.entries(doc)) {
    const next = applyFile(out[path], envelope);
    if (next === undefined) delete out[path];
    else out[path] = next;
  }
  return out;
};

const applyKeyed = (beforeVal: JsonObject[], split: KeyedSplit): Json => {
  const removed = new Set(split.removed ?? []);
  const changed = new Map<string, Json>();
  for (const entry of split.changed ?? []) {
    const key = Object.keys(entry)[0]!;
    changed.set(key, entry[key]!);
  }
  return [
    ...beforeVal.flatMap((entry) => {
      const key = Object.keys(entry)[0]!;
      if (removed.has(key)) return [];
      return changed.has(key) ? [{ [key]: apply(entry[key]!, changed.get(key)!) }] : [entry];
    }),
    ...(split.added ?? []),
  ];
};

const applyObject = (beforeVal: JsonObject, next: JsonObject): Json => {
  const removed = new Set(Array.isArray(next.removed) ? next.removed : []);
  const out: JsonObject = {};
  for (const key of Object.keys(beforeVal)) {
    if (removed.has(key)) continue;
    out[key] = key in next ? apply(beforeVal[key]!, next[key]!) : beforeVal[key]!;
  }
  for (const key of Object.keys(next)) {
    if (key === "removed" && Array.isArray(next.removed)) continue;
    if (!(key in beforeVal)) out[key] = next[key]!;
  }
  return out;
};

const isKeyedSplit = (value: Json): value is KeyedSplit & JsonObject => {
  if (!isObject(value)) return false;
  const allowed = new Set(["added", "changed", "removed"]);
  return Object.keys(value).every((key) => allowed.has(key));
};

export const isObject = (value: Json | undefined): value is JsonObject =>
  value !== null &&
  value !== undefined &&
  typeof value === "object" &&
  !Array.isArray(value) &&
  Object.getPrototypeOf(value) === Object.prototype;

const isKeyedCollection = (value: Json): value is JsonObject[] =>
  Array.isArray(value) &&
  value.length > 0 &&
  value.every(
    (item) =>
      item !== null &&
      typeof item === "object" &&
      !Array.isArray(item) &&
      Object.keys(item).length === 1,
  );

export const snapshotsEqual = (left: Json | undefined, right: Json | undefined): boolean =>
  isDeepStrictEqual(left ?? null, right ?? null);
