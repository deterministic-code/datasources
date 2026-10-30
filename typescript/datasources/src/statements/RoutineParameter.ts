import type { SpecField } from "../converters/SpecField.ts";

export type RoutineParameter = {
  name: string
  type: string
  field?: SpecField
};
