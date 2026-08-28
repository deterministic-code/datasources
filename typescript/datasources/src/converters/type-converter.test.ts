import { afterEach, describe, expect, it } from "vitest";
import { DatasourceError } from "../DatasourceError.ts";
import {
  TypeConverter,
  canonicalConverters,
  identityProviderTypes,
  inferSqlType,
  normalizeSqlType,
  sqlProviderTypes,
} from "./TypeConverter.ts";
import type { Constructable } from "./Constructable.ts";
import { identityFieldConverter } from "./identityFieldConverter.ts";
import { booleanFieldConverter } from "./booleanFieldConverter.ts";
import { dateTimeFieldConverter } from "./dateTimeFieldConverter.ts";
import { decimalFieldConverter } from "./decimalFieldConverter.ts";
import { binaryFieldConverter } from "./binaryFieldConverter.ts";
import { uuidFieldConverter } from "./uuidFieldConverter.ts";
import type { Cells } from "./Cell.ts";

const isoDate = "2020-01-02T03:04:05.000Z";

class User {
  id = 0;
  username = "";
  isEnabled = false;
  created = new Date("2020-01-02T03:04:05.000Z");
}

class Empty {}

class SkipsUndefined {
  id = 1;
  ghost = undefined;
}

class NeedsArg {
  constructor() {
    throw new Error("nope");
  }
}

class Mixed {
  label = "";
  count = 1n;
  ratio = 1.5;
  data = new Uint8Array([1, 2]);
  extra = { nested: true };
  empty = null;
  xmlParser = "";
}

afterEach(() => {
  TypeConverter.reset();
});

const registerSqlite = () =>
  TypeConverter.use("sqlite", sqlProviderTypes({}, { booleanStorage: "integer" }));

describe("TypeConverter", () => {
  it("infers the User schema and round-trips a row", () => {
    registerSqlite();
    const mapping = TypeConverter.mapping<User>({
      source: "sqlite",
      destination: "typescript",
      type: User,
      names: { is_enabled: "isEnabled" },
    });
    expect(mapping.source).toBe("sqlite");
    expect(mapping.destination).toBe("typescript");
    expect(mapping.schema).toEqual({
      id: "INTEGER",
      username: "TEXT",
      is_enabled: "BOOLEAN",
      created: "DATETIME",
    });
    const user = mapping.to({
      id: 7,
      username: "ada",
      is_enabled: 1,
      created: "2020-01-02T03:04:05.000Z",
    });
    expect(user).toBeInstanceOf(User);
    expect(user.id).toBe(7);
    expect(user.username).toBe("ada");
    expect(user.isEnabled).toBe(true);
    expect(user.created).toEqual(new Date("2020-01-02T03:04:05.000Z"));
    expect(mapping.from(user)).toEqual({
      id: 7,
      username: "ada",
      is_enabled: 1,
      created: "2020-01-02T03:04:05.000Z",
    });
  });

  it("merges an explicit schema and field converters", () => {
    registerSqlite();
    const twice = {
      to: (value: number | string | boolean | bigint | Date | Uint8Array | null) =>
        typeof value === "number" ? value * 2 : value,
      from: (value: number | string | boolean | bigint | Date | Uint8Array | null) =>
        typeof value === "number" ? value / 2 : value,
    };
    const mapping = TypeConverter.mapping<User>({
      source: "sqlite",
      destination: "typescript",
      type: User,
      schema: { username: "VARCHAR(32)" },
      converters: { id: twice },
    });
    expect(mapping.schema.username).toBe("VARCHAR(32)");
    expect(mapping.to({ id: 3, username: "x" }).id).toBe(6);
    expect(mapping.from({ ...new User(), id: 6, username: "x" }).id).toBe(3);
  });

  it("maps without a type using identity when the column is unknown", () => {
    registerSqlite();
    const mapping = TypeConverter.mapping({
      source: "sqlite",
      destination: "typescript",
    });
    expect(mapping.schema).toEqual({});
    expect(mapping.to({ display_name: "a" })).toEqual({ displayName: "a" });
    expect(mapping.from({ displayName: "a" })).toEqual({ display_name: "a" });
  });

  it("fills dest names from schema keys", () => {
    registerSqlite();
    const mapping = TypeConverter.mapping({
      source: "sqlite",
      destination: "typescript",
      type: Empty,
      schema: { display_name: "TEXT" },
    });
    expect(mapping.to({ display_name: "a" })).toEqual({ displayName: "a" });
    TypeConverter.use("json", identityProviderTypes());
    const json = TypeConverter.mapping({
      source: "json",
      destination: "typescript",
      schema: { displayName: "TEXT" },
    });
    expect(json.to({ displayName: "a" })).toEqual({ displayName: "a" });
  });

  it("leaves keys unchanged for identity providers", () => {
    TypeConverter.use("json", identityProviderTypes());
    const mapping = TypeConverter.mapping<User>({
      source: "json",
      destination: "typescript",
      type: User,
    });
    expect(mapping.schema).toEqual({
      id: "INTEGER",
      username: "TEXT",
      isEnabled: "BOOLEAN",
      created: "DATETIME",
    });
    expect(mapping.to({ isEnabled: true }).isEnabled).toBe(true);
  });

  it("rejects a wrong destination or missing provider", () => {
    registerSqlite();
    expect(() =>
      TypeConverter.mapping({ source: "sqlite", destination: "csharp" }),
    ).toThrow(/destination must be "typescript"/);
    TypeConverter.reset();
    expect(() =>
      TypeConverter.mapping({ source: "sqlite", destination: "typescript" }),
    ).toThrow(DatasourceError);
  });

  it("rejects a type that cannot be constructed", () => {
    registerSqlite();
    expect(() =>
      TypeConverter.mapping({
        source: "sqlite",
        destination: "typescript",
        type: NeedsArg as Constructable<Cells>,
      }),
    ).toThrow(/must be constructable/);
  });

  it("skips undefined fields and accepts an empty class", () => {
    registerSqlite();
    expect(
      TypeConverter.mapping({
        source: "sqlite",
        destination: "typescript",
        type: Empty,
      }).schema,
    ).toEqual({});
    expect(
      TypeConverter.mapping({
        source: "sqlite",
        destination: "typescript",
        type: SkipsUndefined,
      }).schema,
    ).toEqual({ id: "INTEGER" });
  });

  it("infers REAL, INTEGER bigint, BLOB, and fallback TEXT", () => {
    registerSqlite();
    const mapping = TypeConverter.mapping({
      source: "sqlite",
      destination: "typescript",
      type: Mixed,
    });
    expect(mapping.schema).toEqual({
      label: "TEXT",
      count: "INTEGER",
      ratio: "REAL",
      data: "BLOB",
      extra: "TEXT",
      empty: "TEXT",
      xml_parser: "TEXT",
    });
    expect(inferSqlType("x")).toBe("TEXT");
    expect(inferSqlType(true)).toBe("BOOLEAN");
  });

  it("builds sql alias tables and falls back for unknown aliases", () => {
    const table = sqlProviderTypes(
      { VARCHAR: "TEXT", NOPE: "MISSING", CHAR: "TEXT" },
      {},
    );
    expect(table.rename).toBe(true);
    expect(table.types.VARCHAR).toBe(table.types.TEXT);
    expect(table.types.NOPE).toBe(identityFieldConverter);
    expect(sqlProviderTypes({}, { rename: false }).rename).toBe(false);
    expect(canonicalConverters("native").BOOLEAN.from(true)).toBe(true);
    expect(normalizeSqlType(" varchar(32) ")).toBe("VARCHAR");
    expect(normalizeSqlType("TINYINT(1)")).toBe("TINYINT(1)");
    expect(normalizeSqlType("CHAR(36)")).toBe("CHAR(36)");
    expect(normalizeSqlType("TEXT")).toBe("TEXT");
  });
});

describe("field converters", () => {
  it("passes identity and nulls", () => {
    expect(identityFieldConverter.to("a")).toBe("a");
    expect(identityFieldConverter.from(null)).toBe(null);
  });

  it("converts boolean 0|1 and native storage", () => {
    const integer = booleanFieldConverter();
    const native = booleanFieldConverter("native");
    expect(integer.to(1)).toBe(true);
    expect(integer.to(0)).toBe(false);
    expect(integer.to(1n)).toBe(true);
    expect(integer.to(0n)).toBe(false);
    expect(integer.to(true)).toBe(true);
    expect(integer.to(false)).toBe(false);
    expect(integer.to(null)).toBe(null);
    expect(integer.from(true)).toBe(1);
    expect(integer.from(false)).toBe(0);
    expect(integer.from(1)).toBe(1);
    expect(integer.from(0n)).toBe(0);
    expect(integer.from(null)).toBe(null);
    expect(native.from(true)).toBe(true);
    expect(native.from(false)).toBe(false);
    expect(() => integer.to("yes")).toThrow(/boolean/);
    expect(() => integer.from("yes")).toThrow(/boolean/);
    expect(() => integer.to(new Date(isoDate))).toThrow(/boolean/);
    expect(() => integer.to(new Uint8Array())).toThrow(/Uint8Array/);
  });

  it("converts ISO datetimes", () => {
    const iso = isoDate;
    const date = new Date(iso);
    expect(dateTimeFieldConverter.to(iso)).toEqual(date);
    expect(dateTimeFieldConverter.to(date)).toEqual(date);
    expect(dateTimeFieldConverter.to(null)).toBe(null);
    expect(dateTimeFieldConverter.from(date)).toBe(iso);
    expect(dateTimeFieldConverter.from(iso)).toBe(iso);
    expect(dateTimeFieldConverter.from(null)).toBe(null);
    expect(() => dateTimeFieldConverter.to("not-a-date")).toThrow(/Date/);
    expect(() => dateTimeFieldConverter.to(1)).toThrow(/Date/);
    expect(() => dateTimeFieldConverter.from(1)).toThrow(/ISO 8601/);
    expect(() => dateTimeFieldConverter.to(new Date(Number.NaN))).toThrow(/Date/);
    expect(() => dateTimeFieldConverter.from(new Date(Number.NaN))).toThrow(/Date/);
  });

  it("converts decimals with optional size", () => {
    const plain = decimalFieldConverter();
    const sized = decimalFieldConverter([5, 2]);
    const whole = decimalFieldConverter([3, 0]);
    expect(plain.to(null)).toBe(null);
    expect(plain.to(12.5)).toBe("12.5");
    expect(plain.to(10n)).toBe("10");
    expect(plain.from("1.20")).toBe("1.20");
    expect(sized.to("1.2")).toBe("1.20");
    expect(sized.to(-3)).toBe("-3.00");
    expect(whole.to("12")).toBe("12");
    expect(() => sized.to("1234.5")).toThrow(/exceeds size/);
    expect(() => plain.to(true)).toThrow(/decimal/);
    expect(() => sized.to("x")).toThrow(/decimal/);
    expect(() => plain.to(Number.NaN)).toThrow(/decimal/);
  });

  it("converts binary to base64 and back", () => {
    const bytes = new Uint8Array([104, 105]);
    expect(binaryFieldConverter.to(bytes)).toBe("aGk=");
    expect(binaryFieldConverter.to("aGk=")).toBe("aGk=");
    expect(binaryFieldConverter.to(null)).toBe(null);
    expect(binaryFieldConverter.from("aGk=")).toEqual(bytes);
    expect(binaryFieldConverter.from(bytes)).toEqual(bytes);
    expect(binaryFieldConverter.from(null)).toBe(null);
    expect(() => binaryFieldConverter.to(1)).toThrow(/binary/);
    expect(() => binaryFieldConverter.from(1)).toThrow(/bytes/);
  });

  it("validates RFC-4122 uuids", () => {
    const id = "550e8400-e29b-41d4-a716-446655440000";
    expect(uuidFieldConverter.to(id)).toBe(id);
    expect(uuidFieldConverter.from(id)).toBe(id);
    expect(uuidFieldConverter.to(null)).toBe(null);
    expect(() => uuidFieldConverter.to("nope")).toThrow(/UUID/);
    expect(() => uuidFieldConverter.from(1)).toThrow(/UUID/);
  });
});
