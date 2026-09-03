import type { TypeMapFields } from "./TypeMap.ts";

export type TypeMapGroup<E> = {
  key: string;
  fields: TypeMapFields<E>;
};
