import type { TypeMapGroup } from "./TypeMapGroup.ts";

export type TypeMapFields<T> = {
  [K in keyof T]?: T[K] extends readonly (infer E)[] ? TypeMapGroup<E> : string;
};

export type TypeMap<T> = {
  key: string;
  fields: TypeMapFields<T>;
};
