import type { TypeField } from "@deterministic-code/deterministic-generators-common/parser";
import type { SpecField } from "@deterministic-code/datasources";

export const converterFieldOf = (field: TypeField): SpecField => ({
  type: field.type,
  name: field.name,
  size: field.size,
  defaultValue: field.defaultValue,
});
