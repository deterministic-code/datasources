import { describe, expect, it } from "vitest";
import { DefaultValueConverter, sqlStringLiteral } from "./DefaultValueConverter.ts";
import { PostgresDriver } from "./PostgresDriver.ts";

describe("PostgresDriver converters", () => {
  it("maps NewId and boolean defaults", () => {
    const driver = new PostgresDriver();
    expect(driver.defaultValueConverter.convert("uuid", "NewId")).toBe("gen_random_uuid()");
    expect(driver.defaultValueConverter.convert("boolean", "true")).toBe("TRUE");
    expect(driver.defaultValueConverter.convert("boolean", true as unknown as string)).toBe("TRUE");
    expect(driver.defaultValueConverter.convert("boolean", "false")).toBe("FALSE");
    expect(driver.defaultValueConverter.convert("uuid", "uuid('11111111-1111-1111-1111-111111111111')")).toContain(
      "11111111",
    );
    expect(() => driver.defaultValueConverter.reverseConvert("nope", "x")).toThrow(/Unknown spec field type/);
    expect(driver.defaultValueConverter.convert("integer", "1")).toBe("1");
    expect(driver.typeConverter.convertField({ type: "datetime" })).toBe("TIMESTAMPTZ");
    expect(driver.typeConverter.convertField({ type: "string", size: 64 })).toBe("VARCHAR(64)");
    expect(driver.typeConverter.convertField({ type: "character", size: 4 })).toBe("CHAR(4)");
    expect(driver.defaultValueConverter.convert("datetime", "Now")).toBe("LOCALTIMESTAMP");
    expect(driver.defaultValueConverter.convert("datetime", "DateTime('2020-01-02T00:00:00Z')")).toContain("2020-01-02");
    expect(() => driver.defaultValueConverter.convert("datetime", "2020-01-02T00:00:00Z")).toThrow(
      /not a valid datetime DateTime default/,
    );
    expect(driver.defaultValueConverter.convert("datetime", "UtcNow")).toBe(`(NOW() AT TIME ZONE 'UTC')`);
    expect(driver.defaultValueConverter.convert("binary", "Hex('ab')")).toBe(`'\\xab'`);
    expect(driver.typeConverter.convert("integer")).toBe("INTEGER");
    expect(driver.typeConverter.convert("unsignedinteger")).toBe("BIGINT");
    expect(driver.typeConverter.convert("uuid")).toBe("UUID");
    expect(driver.typeConverter.convert("boolean")).toBe("BOOLEAN");
    expect(driver.typeConverter.reverseConvert("INTEGER")).toBe("integer");
    expect(driver.typeConverter.reverseConvert("BIGINT")).toBe("unsignedinteger");
    expect(driver.defaultValueConverter.convert("uuid", "Empty")).toContain("00000000");
    expect(driver.defaultValueConverter.reverseConvert("uuid", "gen_random_uuid()")).toBe("NewId");
    expect(driver.defaultValueConverter.reverseConvert("binary", `'\\xab'`)).toBe("Hex('ab')");
    expect(
      driver.defaultValueConverter.reverseConvert("uuid", "'11111111-1111-1111-1111-111111111111'"),
    ).toBe("uuid('11111111-1111-1111-1111-111111111111')");
    expect(driver.defaultValueConverter.reverseConvert("uuid", `'${"00000000-0000-0000-0000-000000000000"}'`)).toBe(
      "Empty",
    );
    expect(() => driver.typeConverter.convert("nope")).toThrow(/Unknown spec field type/);
    expect(() => driver.defaultValueConverter.convert("nope", "x")).toThrow(/Unknown spec field type/);
    expect(() => driver.defaultValueConverter.convert("binary", "ab")).toThrow(/not a valid binary Hex default/);
    expect(driver.defaultValueConverter.convert("string", "ada")).toBe(`'ada'`);
    expect(sqlStringLiteral("ada")).toBe("'ada'");
    expect(sqlStringLiteral("a'b")).toBe("'a''b'");
    expect(DefaultValueConverter.parse("string", "ada")).toEqual({ token: "String", arg: "ada" });
    expect(DefaultValueConverter.parse("character", "x")).toEqual({ token: "String", arg: "x" });
    expect(driver.defaultValueConverter.convert("character", "x")).toBe("'x'");
    expect(driver.defaultValueConverter.reverseConvert("string", "'ada'")).toBe("ada");
    expect(driver.defaultValueConverter.reverseConvert("integer", "1")).toBe("");
    expect(driver.defaultValueConverter.reverseConvert("datetime", "LOCALTIMESTAMP")).toBe("Now");
    expect(driver.defaultValueConverter.reverseConvert("datetime", "(NOW() AT TIME ZONE 'UTC')")).toBe("UtcNow");
    expect(driver.defaultValueConverter.reverseConvert("datetime", "'2020-01-02T00:00:00Z'")).toBe(
      "DateTime('2020-01-02T00:00:00Z')",
    );
    expect(driver.defaultValueConverter.convert("unsignedinteger", "1")).toBe("1");
    expect(driver.defaultValueConverter.convert("float", "1.5")).toBe("1.5");
    expect(driver.defaultValueConverter.convert("decimal", "1.5")).toBe("1.5");
    expect(() => driver.defaultValueConverter.convert("uuid", "not-a-uuid")).toThrow(/not a valid uuid/);
    expect(() => driver.defaultValueConverter.reverseConvert("uuid", "nope")).toThrow(/not a valid uuid default/);
    expect(driver.typeConverter.convertField({ type: "string" })).toBe("TEXT");
    expect(driver.typeConverter.convertField({ type: "string", size: "unlimited" })).toBe("TEXT");
    expect(driver.typeConverter.convertField({ type: "character" })).toBe("CHAR(1)");
    expect(driver.typeConverter.convertField({ type: "decimal" })).toBe("NUMERIC");
    expect(driver.typeConverter.convertField({ name: "amount", type: "decimal", size: [10, 2] })).toBe(
      "NUMERIC(10, 2)",
    );
    expect(() => driver.typeConverter.convertField({ name: "amount", type: "decimal", size: 10 })).toThrow(
      /precision, scale/,
    );
    expect(() => driver.typeConverter.reverseConvert("nope")).toThrow(/Unknown native field type/);
    expect(PostgresDriver.nativeSpec("payload")).toBe("jsonb");
    expect(PostgresDriver.nativeParam({ name: "rows", type: "payload" })).toBe("jsonb");
    expect(PostgresDriver.nativeParam({ name: "id", type: "INT" })).toBe("INT");
    expect(PostgresDriver.nativeParam({ name: "id", type: "int", field: { type: "integer" } })).toBe("INTEGER");
    expect(driver.defaultValueConverter.convert("reference", "1")).toBeNull();
    expect(driver.defaultValueConverter.convert("string", null as unknown as string)).toBeNull();
    expect(
      PostgresDriver.nativeParam({
        name: "id",
        type: "payload",
        field: { type: "integer" },
      }),
    ).toBe("jsonb");
    expect(PostgresDriver.nativeType({ name: "id", type: "int" })).toBe("INTEGER");
    expect(PostgresDriver.nativeType({ name: "id", type: "int", field: { type: "integer" } })).toBe("INTEGER");
    expect(PostgresDriver.nativeDefault({ name: "flag", type: "bool" })).toBe("");
    expect(
      PostgresDriver.nativeDefault({
        name: "flag",
        type: "bool",
        default: { value: "1" },
      }),
    ).toBe("1");
    expect(
      PostgresDriver.nativeDefault({
        name: "flag",
        type: "bool",
        default: { value: "true" },
        field: { type: "boolean" },
      }),
    ).toBe("TRUE");
    expect(PostgresDriver.utcNow()).toBe(`(NOW() AT TIME ZONE 'UTC')`);
    const emptyNow = new DefaultValueConverter({
      newId: () => "id",
      now: () => "",
      utcNow: () => "utc",
      booleanOn: "TRUE",
      booleanOff: "FALSE",
      hex: () => "",
      hexReverse: /^$/,
      quote: (arg) => `'${arg}'`,
      quoteReverse: /^'(.*)'$/,
    });
    expect(emptyNow.convert("datetime", "Now")).toBeNull();
    expect(
      PostgresDriver.nativeDefault({
        name: "ref",
        type: "int",
        default: { value: "1" },
        field: { type: "reference" },
      }),
    ).toBe("1");
  });
});

