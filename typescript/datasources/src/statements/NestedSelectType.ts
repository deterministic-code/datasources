import type { NestedSelectRelation } from "./NestedSelectRelation.ts";

export type NestedSelectType = {
  name: string;
  key: string;
  properties: readonly string[];
  relations: readonly NestedSelectRelation[];
};
