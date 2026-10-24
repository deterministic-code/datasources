import type { NestedSelectRelationKind } from "./NestedSelectRelationKind.ts";
import type { NestedSelectType } from "./NestedSelectType.ts";

export type NestedSelectRelation = {
  name: string;
  kind: NestedSelectRelationKind;
  from: string;
  to: string;
  target: NestedSelectType;
};
