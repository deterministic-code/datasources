import { ColumnDefault } from "@deterministic-code/datasources";
import { describe, expect, it } from "vitest";
import { DefaultValueConverter } from "./DefaultValueConverter.ts";
import { SqliteDriver } from "./SqliteDriver.ts";

describe("SqliteDriver converters", () => {
  it("maps UtcNow and sized strings", () => {
    const driver = new SqliteDriver();
    expect(driver.defaultValueConverter.convert("datetime", "UtcNow")).toBe(
      `(strftime('%Y-%m-%dT%H:%M:%fZ', 'now'))`,
    );
    expect(driver.typeConverter.convertField({ type: "string", size: 64 })).toBe(
      "VARCHAR(64)",
    );
    expect(driver.typeConverter.convertField({ type: "string" })).toBe("TEXT");
    expect(driver.typeConverter.convertField({ type: "string", size: "unlimited" })).toBe(
      "TEXT",
    );
    expect(driver.typeConverter.convertField({ type: "uuid" })).toBe("TEXT");
    expect(() => driver.typeConverter.convert("nope")).toThrow(/Unknown spec field type/);
    expect(() => driver.typeConverter.reverseConvert("nope")).toThrow(
      /Unknown native field type/,
    );
    expect(() => driver.defaultValueConverter.convert("nope", "x")).toThrow(
      /Unknown spec field type/,
    );
    expect(() =>
      driver.defaultValueConverter.convert("datetime", "DateTime('not-a-date')"),
    ).toThrow(/not a valid datetime/);
    expect(driver.defaultValueConverter.convert("reference", "1")).toBeNull();
    expect(driver.defaultValueConverter.reverseConvert("string", "'hi'")).toBe("hi");
    expect(() => driver.defaultValueConverter.reverseConvert("nope", "x")).toThrow(
      /Unknown spec field type/,
    );
    expect(() => driver.defaultValueConverter.reverseConvert("string", "oops")).toThrow(
      /Native default/,
    );
    expect(
      driver.defaultValueConverter.convert("datetime", "DateTime('2020-01-01T00:00:00Z')"),
    ).toBe("'2020-01-01T00:00:00Z'");
    expect(driver.defaultValueConverter.convert("uuid", "Empty")).toContain("00000000");
    expect(() =>
      driver.defaultValueConverter.convert("datetime", "2020-01-01T00:00:00Z"),
    ).toThrow(/not a valid datetime/);
    expect(() =>
      driver.defaultValueConverter.convert(
        "uuid",
        "00000000-0000-0000-0000-000000000000",
      ),
    ).toThrow(/not a valid uuid/);
    expect(
      SqliteDriver.procedureSpecs("User", [[]], {
        routineName: (stem) => stem,
        pluralStem: (entity) => `${entity}s`,
      }).some((row) => row.name === "read_Users"),
    ).toBe(true);
    expect(driver.defaultValueConverter.convert("datetime", "Now")).toBe(
      `(strftime('%Y-%m-%dT%H:%M:%f', 'now', 'localtime'))`,
    );
    expect(driver.defaultValueConverter.convert("binary", "Hex('ab')")).toBe("X'ab'");
    expect(driver.defaultValueConverter.convert("uuid", "NewId")).toContain("randomblob");
    expect(DefaultValueConverter.parse("boolean", true)).toEqual({
      token: "Boolean",
      arg: true,
    });
    expect(DefaultValueConverter.parse("string", null)).toEqual({ token: "None" });
    expect(
      driver.defaultValueConverter.reverseConvert("datetime", "'2020-01-01T00:00:00Z'"),
    ).toBe("DateTime('2020-01-01T00:00:00Z')");
    expect(driver.defaultValueConverter.reverseConvert("binary", "X'ab'")).toBe("Hex('ab')");
    expect(
      driver.defaultValueConverter.reverseConvert(
        "uuid",
        "'11111111-1111-1111-1111-111111111111'",
      ),
    ).toBe("uuid('11111111-1111-1111-1111-111111111111')");
    expect(
      driver.defaultValueConverter.convert("string", null as unknown as string),
    ).toBeNull();
    expect(
      driver.defaultValueConverter.reverseConvert(
        "uuid",
        driver.defaultValueConverter.convert("uuid", "NewId")!,
      ),
    ).toBe("NewId");
    expect(driver.defaultValueConverter.convert("boolean", "false")).toBe("0");
    const emptyQuote = new DefaultValueConverter({
      newId: () => "n",
      now: () => "now",
      utcNow: () => "utc",
      booleanOn: "1",
      booleanOff: "0",
      hex: (arg) => arg,
      hexReverse: /^(.*)$/,
      quote: () => "",
    });
    expect(emptyQuote.convert("string", "hi")).toBeNull();
    expect(
      driver.defaultValueConverter.convert(
        "uuid",
        "uuid('11111111-1111-1111-1111-111111111111')",
      ),
    ).toBe("'11111111-1111-1111-1111-111111111111'");
    expect(() => driver.defaultValueConverter.convert("binary", "ab")).toThrow(
      /not a valid binary/,
    );
    expect(driver.defaultValueConverter.convert("boolean", "true")).toBe("1");
    expect(SqliteDriver.nativeDefault({ name: "n", type: "INTEGER" })).toBe("");
    expect(
      SqliteDriver.nativeDefault({
        name: "n",
        type: "INTEGER",
        default: new ColumnDefault("7"),
      }),
    ).toBe("7");
    expect(SqliteDriver.nativeParam({ name: "p", type: "payload" })).toBe("JSON");
    expect(
      SqliteDriver.nativeParam({ name: "p", type: "integer", field: { type: "integer" } }),
    ).toBe("INTEGER");
    expect(SqliteDriver.nativeParam({ name: "p", type: "integer" })).toBe("INTEGER");
    expect(SqliteDriver.nativeParam({ name: "p", type: "mystery" })).toBe("mystery");
    expect(
      SqliteDriver.nativeDefault({
        name: "n",
        type: "INTEGER",
        field: { type: "integer" },
        default: new ColumnDefault("7"),
      }),
    ).toBe("7");
    expect(
      SqliteDriver.nativeDefault({
        name: "n",
        type: "INTEGER",
        field: { type: "reference" },
        default: new ColumnDefault("1"),
      }),
    ).toBe("1");
    expect(SqliteDriver.utcNow()).toContain("strftime");
  });
});
