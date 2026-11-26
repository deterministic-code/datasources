import { readFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import { parse } from "yaml";
import { DatasourceError, type IDefaultValueConverter, type IMapping, type SpecField, type TypeConverter, type Value } from "@deterministic-code/datasources";
import { expect } from "vitest";

type Row = {
  type: string;
  reverse?: boolean;
  fromType?: string;
  toType?: string;
  from: unknown;
  to: unknown;
};

const path = join(dirname(fileURLToPath(import.meta.url)), "../../docs/DriverConversions.yaml");

const section = (driver: string): Record<string, Row[]> =>
  parse(readFileSync(path, "utf8"))[driver].typescript;

const errorName = (value: unknown): string | undefined =>
  value !== null && typeof value === "object" && "error" in value
    ? String((value as { error: string }).error)
    : undefined;

const decode = (raw: unknown): unknown => {
  if (raw === null || typeof raw !== "object" || Array.isArray(raw)) return raw;
  const map = raw as Record<string, unknown>;
  if ("field" in map) return map.field;
  if ("error" in map) return { error: map.error };
  if ("long" in map) return Number(map.long);
  if ("text" in map) return String(map.text);
  if ("bool" in map) return Boolean(map.bool);
  if ("float" in map) return Number(map.float);
  if ("decimal" in map) return String(map.decimal);
  if ("datetime" in map) return new Date(String(map.datetime));
  if ("guid" in map) return String(map.guid);
  if ("bytes" in map) return Uint8Array.from(map.bytes as number[]);
  if ("base64" in map) return String(map.base64);
  if ("latin1" in map) return Buffer.from(map.latin1 as number[]).toString("latin1");
  return raw;
};

const typeName = (value: unknown): string => {
  if (value instanceof Date) return "Date";
  if (value instanceof Uint8Array) return "Uint8Array";
  if (typeof value === "boolean") return "boolean";
  if (typeof value === "number") return "number";
  if (typeof value === "string") return "string";
  if (typeof value === "bigint") return "bigint";
  return typeof value;
};

const isField = (value: unknown): value is SpecField =>
  value !== null && typeof value === "object" && "type" in value && !("error" in value);

const throwOf = (name: string, act: () => unknown) => {
  if (name === "DatasourceError") {
    expect(act).toThrow(DatasourceError);
    return;
  }
  expect(act).toThrow();
};

export const assertTypes = (
  driver: string,
  types: IMapping<string, string> & { convertField: (field: SpecField) => string },
) => {
  for (const row of section(driver).typeConverter) {
    const from = decode(row.from);
    const act = () =>
      row.reverse
        ? types.reverseConvert(String(from))
        : isField(from)
          ? types.convertField(from)
          : types.convert(String(from));
    const error = errorName(row.to);
    if (error) throwOf(error, act);
    else expect(act()).toBe(row.to);
  }
};

export const assertDefaults = (driver: string, defaults: IDefaultValueConverter) => {
  for (const row of section(driver).defaultValueConverter) {
    const from = String(decode(row.from));
    const act = () =>
      row.reverse ? defaults.reverseConvert(row.type, from) : defaults.convert(row.type, from);
    const error = errorName(row.to);
    if (error) throwOf(error, act);
    else expect(act()).toBe(row.to ?? null);
  }
};

export const assertCells = (
  driver: string,
  catalog: Record<string, TypeConverter<Value, Value>>,
) => {
  for (const row of section(driver).rowTypeConverters) {
    const from = decode(row.from);
    expect(typeName(from)).toBe(row.fromType);
    const converter = catalog[row.type];
    const act = () => (row.reverse ? converter.reverseConvert(from as Value) : converter.convert(from as Value));
    const error = errorName(row.to);
    if (error) {
      throwOf(error, act);
      continue;
    }
    const got = act();
    const expected = decode(row.to);
    expect(typeName(got)).toBe(row.toType);
    expect(got).toEqual(expected);
  }
};
