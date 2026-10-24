import type { TypeMapGroup } from "./mappers/TypeMapGroup.ts";
import type { SelectStatement } from "./statements/SelectStatement.ts";

export type NestedSelectMapField = string | TypeMapGroup<Record<string, string>>;

export type NestedSelectStatement = {
  select: SelectStatement;
  map: { key: string; fields: Record<string, NestedSelectMapField> };
};
