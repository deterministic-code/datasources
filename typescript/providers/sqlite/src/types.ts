import type { Cell } from "@datasources/core";
import { TypeConverter } from "@datasources/core";

export const SqliteTypes: Record<string, TypeConverter<Cell, Cell>> = {};
