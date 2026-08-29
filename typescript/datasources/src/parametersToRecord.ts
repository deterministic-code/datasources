import type { Parameter } from "./Parameter.ts";

export function parametersToRecord<T>(
  parameters?: Parameter<T>[] | null,
): Record<string, T> {
  if (!parameters) return {};
  const record: Record<string, T> = {};
  for (const parameter of parameters) record[parameter.name] = parameter.value;
  return record;
}
