import type { TypeMapDictionary } from "./TypeMapDictionary.ts";
import type { TypeMapGroup } from "./TypeMapGroup.ts";

type Cell = string | number | boolean | null;

export type TypeMapFields<T> = {
  [K in keyof T]?: T[K] extends readonly (infer E)[]
    ? TypeMapGroup<E>
    : T[K] extends Record<string, infer V>
      ? [V] extends [Cell]
        ? TypeMapDictionary
        : string
      : string;
};

export type TypeMap<T> = {
  key: string;
  fields: TypeMapFields<T>;
};
