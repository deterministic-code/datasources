import { DatasourceError } from "../DatasourceError.ts";
import { DatasourceMapper } from "../mappers/DatasourceMapper.ts";
import type { Cell, Cells } from "./Cell.ts";
import type { Constructable } from "./Constructable.ts";
import type { FieldConverter } from "./FieldConverter.ts";
import type { ProviderTypes } from "./ProviderTypes.ts";
import type { TypeMapping } from "./TypeMapping.ts";
import { binaryFieldConverter } from "./binaryFieldConverter.ts";
import { booleanFieldConverter } from "./booleanFieldConverter.ts";
import { dateTimeFieldConverter } from "./dateTimeFieldConverter.ts";
import { decimalFieldConverter } from "./decimalFieldConverter.ts";
import { identityFieldConverter } from "./identityFieldConverter.ts";
import { uuidFieldConverter } from "./uuidFieldConverter.ts";

export type { Cell, Cells } from "./Cell.ts";
export type { Constructable } from "./Constructable.ts";
export type { FieldConverter } from "./FieldConverter.ts";
export type { ProviderTypes } from "./ProviderTypes.ts";
export type { TypeMapping } from "./TypeMapping.ts";

const registry = new Map<string, ProviderTypes>();
const host = "typescript";

export const TypeConverter = {
  use: (source: string, types: ProviderTypes) => {
    registry.set(source, types);
  },
  reset: () => {
    registry.clear();
  },
  mapping: <T extends Cells>(opts: {
    source: string;
    destination: string;
    type?: Constructable<T>;
    name?: string;
    schema?: Record<string, string>;
    converters?: Record<string, FieldConverter>;
    names?: Record<string, string>;
  }): TypeMapping<T> => {
    if (opts.destination !== host)
      throw new DatasourceError(`TypeConverter destination must be "${host}"`);
    const table = registry.get(opts.source);
    if (!table)
      throw new DatasourceError(`Provider "${opts.source}" is not registered`);
    const inferred = opts.type
      ? inferSchema(opts.type, table.rename)
      : { schema: {}, names: {} as Record<string, string> };
    const schema = { ...inferred.schema, ...opts.schema };
    const namePairs = { ...inferred.names, ...opts.names };
    for (const sqlName of Object.keys(schema)) {
      if (namePairs[sqlName] === undefined)
        namePairs[sqlName] = table.rename ? camel(sqlName) : sqlName;
    }
    const names = DatasourceMapper.mapping({
      source: opts.source,
      destination: host,
      names: namePairs,
    });
    const converterFor = (sqlName: string): FieldConverter => {
      const override = opts.converters?.[sqlName];
      if (override) return override;
      const sqlType = schema[sqlName];
      return sqlType
        ? (table.types[normalize(sqlType)] ?? identityFieldConverter)
        : identityFieldConverter;
    };
    return {
      name: opts.name ?? opts.type?.name ?? "",
      source: opts.source,
      destination: host,
      schema,
      to: (row) => {
        const out: Cells = opts.type ? new opts.type() : {};
        for (const [sqlName, value] of Object.entries(row))
          out[destName(names, sqlName, table.rename)] = converterFor(sqlName).to(value);
        return out as T;
      },
      from: (obj) => {
        const out: Cells = {};
        for (const [field, value] of Object.entries(obj as Cells)) {
          const sqlName = sqlNameOf(names, field, table.rename);
          out[sqlName] = converterFor(sqlName).from(value);
        }
        return out;
      },
    };
  },
};

export const canonicalConverters = (
  booleanStorage: "integer" | "native" = "integer",
): Record<string, FieldConverter> => ({
  TEXT: identityFieldConverter,
  INTEGER: identityFieldConverter,
  REAL: identityFieldConverter,
  NUMERIC: decimalFieldConverter(),
  BOOLEAN: booleanFieldConverter(booleanStorage),
  DATETIME: dateTimeFieldConverter,
  BLOB: binaryFieldConverter,
  UUID: uuidFieldConverter,
});

export const sqlProviderTypes = (
  aliases: Record<string, string> = {},
  options: { rename?: boolean; booleanStorage?: "integer" | "native" } = {},
): ProviderTypes => {
  const types = canonicalConverters(options.booleanStorage ?? "integer");
  for (const [alias, target] of Object.entries(aliases))
    types[alias] = types[target] ?? identityFieldConverter;
  return { rename: options.rename ?? true, types };
};

export const identityProviderTypes = (): ProviderTypes => ({
  rename: false,
  types: {},
});

export const inferSqlType = (value: Cell | object): string => {
  if (value instanceof Date) return "DATETIME";
  if (value instanceof Uint8Array) return "BLOB";
  if (typeof value === "boolean") return "BOOLEAN";
  if (typeof value === "bigint") return "INTEGER";
  if (typeof value === "number")
    return Number.isInteger(value) ? "INTEGER" : "REAL";
  return "TEXT";
};

const inferSchema = <T>(
  type: Constructable<T>,
  rename: boolean,
): { schema: Record<string, string>; names: Record<string, string> } => {
  let instance: T;
  try {
    instance = new type();
  } catch {
    throw new DatasourceError("TypeConverter type must be constructable");
  }
  const schema: Record<string, string> = {};
  const names: Record<string, string> = {};
  for (const [key, value] of Object.entries(instance as object)) {
    if (value === undefined) continue;
    const sqlName = rename ? snake(key) : key;
    schema[sqlName] = inferSqlType(value as Cell);
    names[sqlName] = key;
  }
  return { schema, names };
};

const snake = (name: string): string =>
  name
    .replace(/([a-z0-9])([A-Z])/g, "$1_$2")
    .replace(/([A-Z]+)([A-Z][a-z])/g, "$1_$2")
    .toLowerCase();

const camel = (name: string): string =>
  name.replace(/_([a-z])/g, (_, letter: string) => letter.toUpperCase());

const destName = (
  names: { to: (name: string) => string },
  sqlName: string,
  rename: boolean,
) => {
  const dest = names.to(sqlName);
  return dest !== sqlName || !rename ? dest : camel(sqlName);
};

const sqlNameOf = (
  names: { from: (name: string) => string },
  destName: string,
  rename: boolean,
) => {
  const sql = names.from(destName);
  return sql !== destName || !rename ? sql : snake(destName);
};

export const normalizeSqlType = (sqlType: string): string => {
  const trimmed = sqlType.trim().toUpperCase();
  if (trimmed.startsWith("TINYINT(1)")) return "TINYINT(1)";
  if (trimmed.startsWith("CHAR(36)")) return "CHAR(36)";
  const paren = trimmed.indexOf("(");
  return paren === -1 ? trimmed : trimmed.slice(0, paren).trim();
};

const normalize = normalizeSqlType;
