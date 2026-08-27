import type { Parameter } from "./Parameter.ts";

export function parametersToRecord(
  parameters?: Parameter[] | null,
): Record<string, unknown> {
  if (!parameters) return {};
  const record: Record<string, unknown> = {};
  for (const parameter of parameters) record[parameter.name] = parameter.value;
  return record;
}
