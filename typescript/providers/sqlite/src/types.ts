import { CoreTypes, type ValueConverter } from "@datasources/core";

export const SqliteTypes: Record<string, ValueConverter> = { ...CoreTypes };
