import { describe, expect, it } from "vitest";
import { DefaultValueConverter } from "./DefaultValueConverter.ts";
import { MysqlDriver } from "./MysqlDriver.ts";
import { applyKeywords } from "./applyKeywords.ts";

describe("MysqlDriver converters", () => {
  it("maps oversized strings to LONGTEXT", () => {
    const driver = new MysqlDriver();
    expect(driver.typeConverter.convertField({ type: "string", size: 20000 })).toBe(
      "LONGTEXT",
    );
    expect(driver.defaultValueConverter.convert("uuid", "NewId")).toBe("(UUID())");
    expect(driver.typeConverter.convertField({ type: "string", size: 64 })).toBe("VARCHAR(64)");
    expect(driver.typeConverter.convertField({ type: "character", size: 4 })).toBe("CHAR(4)");
    expect(driver.typeConverter.convertField({ type: "binary", size: 16 })).toBe("VARBINARY(16)");
    expect(driver.typeConverter.convertField({ type: "binary", size: 70000 })).toBe("LONGBLOB");
    expect(driver.defaultValueConverter.convert("datetime", "Now")).toBe("(NOW(3))");
    expect(driver.defaultValueConverter.convert("datetime", "UtcNow")).toBe("(UTC_TIMESTAMP(3))");
    expect(driver.defaultValueConverter.convert("boolean", true)).toBe("1");
    expect(() =>
      driver.defaultValueConverter.convert("datetime", "2020-01-01T00:00:00.000Z"),
    ).toThrow(/not a valid datetime/);
    expect(driver.defaultValueConverter.convert("binary", "Hex('ab')")).toBe("(X'ab')");
    expect(driver.defaultValueConverter.reverseConvert("binary", "(X'ab')")).toBe("Hex('ab')");
    expect(driver.defaultValueConverter.reverseConvert("binary", "X'ab'")).toBe("Hex('ab')");
    expect(() => driver.defaultValueConverter.reverseConvert("binary", "X'ab')")).toThrow(
      /Native default/,
    );
    expect(() => driver.defaultValueConverter.reverseConvert("binary", "(X'ab'")).toThrow(
      /Native default/,
    );
    expect(driver.defaultValueConverter.convert("string", "")).toBe("('')");
    expect(driver.defaultValueConverter.convert("string", "hi")).toBe("('hi')");
    expect(driver.defaultValueConverter.reverseConvert("string", "('hi')")).toBe("hi");
    expect(driver.defaultValueConverter.reverseConvert("string", "'hi'")).toBe("hi");
    expect(() => driver.defaultValueConverter.reverseConvert("string", "'hi')")).toThrow(
      /Native default/,
    );
    expect(() => driver.defaultValueConverter.reverseConvert("string", "('hi'")).toThrow(
      /Native default/,
    );
    expect(driver.defaultValueConverter.convert("uuid", "Empty")).toContain("00000000");
    expect(
      driver.defaultValueConverter.convert(
        "uuid",
        "uuid('11111111-1111-1111-1111-111111111111')",
      ),
    ).toContain("11111111");
    expect(() =>
      driver.defaultValueConverter.convert("uuid", "00000000-0000-0000-0000-000000000000"),
    ).toThrow(/not a valid uuid/);
    expect(() => driver.defaultValueConverter.convert("nope", "x")).toThrow(/Unknown spec field type/);
    expect(driver.typeConverter.convert("integer")).toBe("INT");
    expect(() => driver.typeConverter.convert("nope")).toThrow(/Unknown spec field type/);
    expect(driver.typeConverter.convertField({ type: "string" })).toBe("TEXT");
    expect(driver.typeConverter.convertField({ type: "string", size: "unlimited" })).toBe("TEXT");
    expect(driver.typeConverter.convertField({ type: "character" })).toBe("CHAR(1)");
    expect(driver.typeConverter.convertField({ type: "decimal" })).toBe("DECIMAL(10, 2)");
    expect(driver.typeConverter.convertField({ type: "decimal", size: [8, 3] })).toBe("DECIMAL(8, 3)");
    expect(() => driver.typeConverter.convertField({ name: "amount", type: "decimal", size: 8 })).toThrow(
      /decimal field/,
    );
    expect(driver.typeConverter.convertField({ type: "binary" })).toBe("BLOB");
    expect(driver.typeConverter.convertField({ type: "binary", size: "unlimited" })).toBe("BLOB");
    expect(driver.typeConverter.reverseConvert("INT")).toBe("integer");
    expect(() => driver.typeConverter.reverseConvert("nope")).toThrow(/Unknown native field type/);
    expect(() => driver.columnType("nope")).toThrow(/Unknown mysql type/);
    expect(
      driver.statementRenderer.procedures({
        plural: "Users",
        table: "users",
        columns: [],
        createParameters: [],
        writable: [],
        createColumns: [],
        updateColumns: [],
        typedColumns: [],
        byGroups: [],
        key: "id",
        keyType: "integer",
        generatedKey: false,
        routineName: (stem) => stem,
      }).length,
    ).toBeGreaterThan(0);
    expect(MysqlDriver.nativeDefault({ name: "n", type: "INT" })).toBe("");
    expect(MysqlDriver.nativeDefault({ name: "n", type: "INT", default: { value: "1" } })).toBe("1");
    expect(
      MysqlDriver.nativeDefault({
        name: "id",
        type: "uuid",
        field: { type: "uuid" },
        default: { value: "NewId" },
      }),
    ).toBe("(UUID())");
    expect(
      MysqlDriver.nativeDefault({
        name: "ref",
        type: "INT",
        field: { type: "reference" },
        default: { value: "1" },
      }),
    ).toBe("1");
    expect(MysqlDriver.quote("order")).toBe("`order`");
    expect(MysqlDriver.quote("OldName")).toBe("`OldName`");
    expect(MysqlDriver.nativeParam({ name: "payload", type: "payload" })).toBe("JSON");
    expect(MysqlDriver.nativeParam({ name: "id", type: "integer" })).toBe("INT");
    expect(MysqlDriver.nativeParam({ name: "id", type: "INT" })).toBe("INT");
    expect(MysqlDriver.nativeParam({ name: "id", type: "integer", field: { type: "integer" } })).toBe("INT");
    expect(MysqlDriver.utcNow()).toContain("UTC_TIMESTAMP");
    expect(MysqlDriver.parameter("id")).toBe(":id");
    expect(MysqlDriver.placeholders("SELECT $id")).toBe("SELECT :id");
    expect(applyKeywords("SELECT * FROM order", (word) => word.toUpperCase())).toContain("ORDER");
    expect(applyKeywords("'SELECT' /* SELECT */ -- SELECT\n", (word) => word)).toBe(
      "'SELECT' /* SELECT */ -- SELECT\n",
    );
    expect(driver.defaultValueConverter.convert("character", "a")).toContain("a");
    expect(driver.defaultValueConverter.reverseConvert("character", "('a')")).toBe("a");
    expect(() => driver.defaultValueConverter.reverseConvert("nope", "x")).toThrow(/Unknown spec field type/);
    expect(() => driver.defaultValueConverter.reverseConvert("datetime", "nope")).toThrow(/Native default/);
    expect(driver.defaultValueConverter.reverseConvert("uuid", "(UUID())")).toBe("NewId");
    const fallback = new DefaultValueConverter({
      newId: () => "(UUID())",
      now: () => "(NOW(3))",
      utcNow: () => "(UTC_TIMESTAMP(3))",
      booleanOn: "1",
      booleanOff: "0",
      hex: (arg) => `(X'${arg}')`,
      hexReverse: /^(?:\(X'(.*)'\)|X'(.*)')$/,
    });
    expect(fallback.convert("string", "hi")).toContain("hi");
    expect(fallback.convert("string", "hi")).not.toBe("");
    const emptyQuote = new DefaultValueConverter({
      newId: () => "n",
      now: () => "now",
      utcNow: () => "",
      booleanOn: "1",
      booleanOff: "0",
      hex: (arg) => arg,
      hexReverse: /^(.*)$/,
      quote: () => "",
    });
    expect(emptyQuote.convert("string", "hi")).toBeNull();
    expect(emptyQuote.convert("datetime", "UtcNow")).toBeNull();
  });
});
