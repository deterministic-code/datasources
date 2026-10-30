import { describe, expect, it } from "vitest";
import { DefaultValueConverter, sqlStringLiteral } from "./DefaultValueConverter.ts";

const converter = () =>
  new DefaultValueConverter({
    newId: () => "gen_random_uuid()",
    now: () => "LOCALTIMESTAMP",
    utcNow: () => `(NOW() AT TIME ZONE 'UTC')`,
    booleanOn: "TRUE",
    booleanOff: "FALSE",
    hex: (arg) => `'\\x${arg}'`,
    hexReverse: /^'\\x(.*)'$/,
  });

const emptyNewId = () =>
  new DefaultValueConverter({
    newId: () => "",
    now: () => "now()",
    utcNow: () => "utc()",
    booleanOn: "1",
    booleanOff: "0",
    hex: (arg) => `X'${arg}'`,
    hexReverse: /^X'(.*)'$/,
  });

describe("DefaultValueConverter", () => {
  it("converts spec tokens", () => {
    const d = converter();
    expect(d.convert("datetime", "UtcNow")).toBe(`(NOW() AT TIME ZONE 'UTC')`);
    expect(d.convert("datetime", "Now")).toBe("LOCALTIMESTAMP");
    expect(d.convert("string", "o'reilly")).toBe("'o''reilly'");
    expect(d.convert("unsignedinteger", "3")).toBe("3");
    expect(d.convert("smallinteger", "-2")).toBe("-2");
    expect(d.convert("character", "Z")).toBe("'Z'");
    expect(DefaultValueConverter.parse("binary", "Hex('cd')")).toEqual({
      token: "Hex",
      arg: "cd",
    });
    expect(d.convert("datetime", "DateTime('2020-01-01T00:00:00Z')")).toBe(
      `'2020-01-01T00:00:00Z'`,
    );
    expect(d.convert("uuid", "NewId")).toBe("gen_random_uuid()");
    expect(d.convert("uuid", "Empty")).toBe(`'00000000-0000-0000-0000-000000000000'`);
    expect(d.convert("uuid", "uuid('aaaaaaaa-aaaa-aaaa-aaaa-aaaaaaaaaaaa')")).toBe(
      `'aaaaaaaa-aaaa-aaaa-aaaa-aaaaaaaaaaaa'`,
    );
    expect(d.convert("boolean", "true")).toBe("TRUE");
    expect(d.convert("boolean", "false")).toBe("FALSE");
    expect(d.convert("integer", "9")).toBe("9");
    expect(d.convert("string", "hi")).toBe("'hi'");
    expect(d.convert("character", "A")).toBe("'A'");
    expect(d.convert("binary", "Hex('ab')")).toBe(`'\\xab'`);
    expect(d.convert("string", undefined as unknown as string)).toBeNull();
    expect(emptyNewId().convert("uuid", "NewId")).toBeNull();
    expect(d.convert("reference", "1")).toBeNull();
    expect(d.convert("float", "1.5")).toBe("1.5");
    expect(d.convert("decimal", "2.5")).toBe("2.5");
    expect(DefaultValueConverter.parse("datetime", "Now")).toEqual({ token: "Now" });
    expect(DefaultValueConverter.parse("datetime", "UtcNow")).toEqual({ token: "UtcNow" });
    expect(DefaultValueConverter.parse("uuid", "NewId")).toEqual({ token: "NewId" });
    expect(DefaultValueConverter.parse("uuid", "Empty")).toEqual({ token: "Empty" });
    expect(DefaultValueConverter.parse("boolean", "true")).toEqual({ token: "Boolean", arg: true });
  });

  it("parses and rejects invalid defaults", () => {
    const d = converter();
    expect(DefaultValueConverter.parse("datetime", null)).toEqual({ token: "None" });
    expect(DefaultValueConverter.parse("boolean", true)).toEqual({ token: "Boolean", arg: true });
    expect(DefaultValueConverter.parse("datetime", "2020-01-01T00:00:00Z")).toEqual({
      token: "DateTime",
      arg: "2020-01-01T00:00:00Z",
    });
    expect(DefaultValueConverter.parse("uuid", "aaaaaaaa-aaaa-aaaa-aaaa-aaaaaaaaaaaa")).toEqual({
      token: "Uuid",
      arg: "aaaaaaaa-aaaa-aaaa-aaaa-aaaaaaaaaaaa",
    });
    expect(DefaultValueConverter.parse("binary", "ff")).toEqual({ token: "Hex", arg: "ff" });
    expect(() => d.convert("nope", "x")).toThrow(/Unknown spec field type/);
    expect(() => d.convert("datetime", "tomorrow")).toThrow(/not a valid datetime/);
    expect(sqlStringLiteral("o'reilly")).toBe("'o''reilly'");
  });

  it("reverses native defaults", () => {
    const d = converter();
    expect(d.reverseConvert("datetime", `(NOW() AT TIME ZONE 'UTC')`)).toBe("UtcNow");
    expect(d.reverseConvert("datetime", "LOCALTIMESTAMP")).toBe("Now");
    expect(d.reverseConvert("string", "'hi'")).toBe("hi");
    expect(d.reverseConvert("uuid", "gen_random_uuid()")).toBe("NewId");
    expect(d.reverseConvert("boolean", "TRUE")).toBe("TRUE");
    expect(d.reverseConvert("uuid", `'00000000-0000-0000-0000-000000000000'`)).toBe("Empty");
    expect(d.reverseConvert("datetime", `'2020-01-01T00:00:00Z'`)).toBe(
      "DateTime('2020-01-01T00:00:00Z')",
    );
    expect(d.reverseConvert("binary", `'\\xab'`)).toBe("Hex('ab')");
    expect(d.reverseConvert("uuid", `'aaaaaaaa-aaaa-aaaa-aaaa-aaaaaaaaaaaa'`)).toBe(
      "uuid('aaaaaaaa-aaaa-aaaa-aaaa-aaaaaaaaaaaa')",
    );
    expect(() => d.reverseConvert("nope", "x")).toThrow(/Unknown spec field type/);
    expect(() => d.reverseConvert("datetime", "nope")).toThrow(/Native default/);
  });
});
