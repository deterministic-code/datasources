import { DatasourceError } from "../DatasourceError.ts";
import type { Cell, Cells } from "./Cell.ts";
import type { Constructable } from "./Constructable.ts";
import type { FieldConverter } from "./FieldConverter.ts";
import type { ProviderTypes } from "./ProviderTypes.ts";
import type { TypeMapping } from "./TypeMapping.ts";
import { identityFieldConverter, inferSqlType } from "./fieldConverters.ts";

export type { Cell, Cells } from "./Cell.ts";
export type { Constructable } from "./Constructable.ts";
export type { FieldConverter } from "./FieldConverter.ts";
export type { ProviderTypes } from "./ProviderTypes.ts";
export type { TypeMapping } from "./TypeMapping.ts";

const registry = new Map<string, ProviderTypes>();

export const TypeConverter = {
  use: (source: string, types: ProviderTypes) => {
    registry.set(source, types);
  },
  reset: () => {
    registry.clear();
  },
  table: (source: string): ProviderTypes => {
    const table = registry.get(source);
    if (!table) throw new DatasourceError(`Provider "${source}" is not registered`);
    return table;
  },
  keys: (type?: Constructable<Cells>): string[] =>
    type
      ? Object.entries(construct(type) as object)
          .filter(([, value]) => value !== undefined)
          .map(([key]) => key)
      : [],
  mapping: <T extends Cells>(opts: {
    source: string;
    type?: Constructable<T>;
    name?: string;
    converters?: Record<string, FieldConverter>;
  }): TypeMapping<T> => {
    const table = TypeConverter.table(opts.source);
    const samples: Cells = opts.type ? construct(opts.type) : {};
    const converterFor = (key: string): FieldConverter => {
      if (opts.converters?.[key]) return opts.converters[key];
      const sample = samples[key];
      return sample === undefined
        ? identityFieldConverter
        : (table.types[inferSqlType(sample as Cell)] ?? identityFieldConverter);
    };
    return {
      name: opts.name ?? opts.type?.name ?? "",
      source: opts.source,
      to: (row) => {
        const out: Cells = opts.type ? construct(opts.type) : {};
        for (const [key, value] of Object.entries(row))
          out[key] = converterFor(key).to(value);
        return out as T;
      },
      from: (obj) => {
        const out: Cells = {};
        for (const [key, value] of Object.entries(obj as Cells))
          out[key] = converterFor(key).from(value);
        return out;
      },
    };
  },
};

const construct = <T extends Cells>(type: Constructable<T>): T => {
  try {
    return new type();
  } catch {
    throw new DatasourceError("TypeConverter type must be constructable");
  }
};
