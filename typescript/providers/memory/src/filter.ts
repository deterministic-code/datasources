import type { MemoryRecord } from "./store.ts";

export function parseFilter(text?: string | null): MemoryRecord {
  if (text === undefined || text === null || text.trim() === "") {
    return {};
  }
  const parsed: unknown = JSON.parse(text);
  if (parsed === null || typeof parsed !== "object" || Array.isArray(parsed)) {
    throw new Error("Memory filter must be an object");
  }
  return parsed as MemoryRecord;
}

export function matches(obj: MemoryRecord, filter: MemoryRecord): boolean {
  return Object.entries(filter).every(([key, value]) => obj[key] === value);
}

export function asRecord(obj: unknown): MemoryRecord {
  if (obj === null || typeof obj !== "object" || Array.isArray(obj)) {
    throw new Error("Repository objects must be plain objects");
  }
  return obj as MemoryRecord;
}

export function parametersToObject(
  parameters?: { name: string; value: unknown }[],
): MemoryRecord {
  const record: MemoryRecord = {};
  if (!parameters) {
    return record;
  }
  for (const parameter of parameters) {
    record[parameter.name] = parameter.value;
  }
  return record;
}
