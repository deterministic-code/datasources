import type { FieldConverter } from "./FieldConverter.ts";

export type ProviderTypes = {
  rename: boolean;
  types: Record<string, FieldConverter>;
};
