import type { Cell } from "./Cell.ts";
import type { TypeConverter } from "./TypeConverter.ts";

const byProvider = new Map<string, Record<string, TypeConverter<Cell, Cell>>>();

export const DefaultTypeConverters = {
  use: (provider: string, types: Record<string, TypeConverter<Cell, Cell>>) => {
    byProvider.set(provider, types);
  },
  for: (provider: string): Record<string, TypeConverter<Cell, Cell>> =>
    byProvider.get(provider) ?? {},
};
