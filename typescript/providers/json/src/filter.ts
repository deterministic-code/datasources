export function parseFilter(text?: string | null): Record<string, unknown> {
  if (text === undefined || text === null || text.trim() === "") {
    return {};
  }
  const parsed: unknown = JSON.parse(text);
  if (parsed === null || typeof parsed !== "object" || Array.isArray(parsed)) {
    throw new Error("JSON filter must be an object");
  }
  return parsed as Record<string, unknown>;
}

export function matches(
  obj: Record<string, unknown>,
  filter: Record<string, unknown>,
): boolean {
  return Object.entries(filter).every(([key, value]) => obj[key] === value);
}

export function asRecord(obj: unknown): Record<string, unknown> {
  if (obj === null || typeof obj !== "object" || Array.isArray(obj)) {
    throw new Error("Repository objects must be plain objects");
  }
  return obj as Record<string, unknown>;
}

export function parametersToObject(
  parameters?: { name: string; value: unknown }[],
): Record<string, unknown> {
  const record: Record<string, unknown> = {};
  if (!parameters) {
    return record;
  }
  for (const parameter of parameters) {
    record[parameter.name] = parameter.value;
  }
  return record;
}
