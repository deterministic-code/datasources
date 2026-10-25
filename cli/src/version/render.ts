import { parse, stringify } from "yaml";
import { delta, isObject, type Json } from "./delta.ts";

export type FilePair = { path: string; before?: Json; after?: Json };

export const render = (files: FilePair[]): string => {
  const out: { [key: string]: Json } = {};
  for (const { path, before, after } of files) {
    if (before === undefined && after === undefined) continue;
    if (before === undefined) {
      out[path] = { added: true, content: after ?? null };
      continue;
    }
    if (after === undefined) {
      out[path] = { removed: true };
      continue;
    }
    const nested = delta(before, after);
    if (nested !== undefined) out[path] = nested;
  }
  return Object.keys(out).length === 0 ? "" : stringify(out);
};

export const parseYaml = (text: string): Json => parse(text) as Json;

export const asDocument = (value: Json, source: string): { [path: string]: Json } =>
  isObject(value) ? value : (() => { throw new Error(`${source}: expected a mapping`); })();
