import { isDeepStrictEqual } from "node:util";

export type Json = string | number | boolean | null | Json[] | { [key: string]: Json };

type JsonObject = { [key: string]: Json };

type KeyedSplit = {
  added?: JsonObject[];
  changed?: JsonObject[];
  removed?: string[];
};

const unchanged = Symbol("unchanged");

type MaybeDelta = Json | typeof unchanged;

export const delta = (before: Json, after: Json): Json | undefined => {
  const value = build(before, after);
  return value === unchanged ? undefined : value;
};

const build = (before: Json, after: Json): MaybeDelta => {
  if (isDeepStrictEqual(before, after)) return unchanged;
  if (isKeyedCollection(before) && isKeyedCollection(after)) return keyedSplit(before, after);
  if (isObject(before) && isObject(after)) return objectDelta(before, after);
  return after;
};

const keyedSplit = (beforeArr: JsonObject[], afterArr: JsonObject[]): MaybeDelta => {
  const beforeMap = keyedMap(beforeArr);
  const afterMap = keyedMap(afterArr);
  const added: JsonObject[] = [];
  const changed: JsonObject[] = [];
  for (const entry of afterArr) {
    const key = Object.keys(entry)[0]!;
    const value = entry[key]!;
    if (!beforeMap.has(key)) added.push({ [key]: value });
    else {
      const nested = build(beforeMap.get(key)!, value);
      if (nested !== unchanged) changed.push({ [key]: nested });
    }
  }
  const removed = [...beforeMap.keys()].filter((key) => !afterMap.has(key)).sort();
  const result: KeyedSplit = {
    ...(added.length ? { added } : {}),
    ...(changed.length ? { changed } : {}),
    ...(removed.length ? { removed } : {}),
  };
  return Object.keys(result).length === 0 ? unchanged : result;
};

const objectDelta = (beforeObj: JsonObject, afterObj: JsonObject): MaybeDelta => {
  const result: JsonObject = {};
  for (const key of Object.keys(afterObj)) {
    if (!(key in beforeObj)) {
      result[key] = afterObj[key]!;
      continue;
    }
    const nested = build(beforeObj[key]!, afterObj[key]!);
    if (nested !== unchanged) result[key] = nested;
  }
  const removed = Object.keys(beforeObj)
    .filter((key) => !(key in afterObj))
    .sort();
  if (removed.length) result.removed = removed;
  return Object.keys(result).length === 0 ? unchanged : result;
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

const keyedMap = (arr: JsonObject[]): Map<string, Json> => {
  const map = new Map<string, Json>();
  for (const entry of arr) {
    const key = Object.keys(entry)[0]!;
    map.set(key, entry[key]!);
  }
  return map;
};
