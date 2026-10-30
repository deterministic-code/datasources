import { describe, expect, it } from "vitest";
import { DeterministicTypeConverter } from "./DeterministicTypeConverter.ts";

const sqlite = () =>
  new DeterministicTypeConverter(
    {
      string: "TEXT",
      character: "TEXT",
      integer: "INTEGER",
      decimal: "NUMERIC",
      binary: "BLOB",
      uuid: "TEXT",
    },
    {
      dialectName: "sqlite",
      stringUnlimited: "TEXT",
      stringSized: (n) => `VARCHAR(${n})`,
    },
  );

const sqlserver = () =>
  new DeterministicTypeConverter(
    {
      string: "NVARCHAR(MAX)",
      character: "NCHAR(1)",
      decimal: "DECIMAL",
      binary: "VARBINARY(MAX)",
    },
    {
      dialectName: "sqlserver",
      stringUnlimited: "NVARCHAR(MAX)",
      stringSized: (n) => `NVARCHAR(${n})`,
      character: "NCHAR",
      decimal: { kind: "required", type: "DECIMAL" },
      binaryUnlimited: "VARBINARY(MAX)",
      binarySized: (n) => `VARBINARY(${n})`,
    },
  );

const postgres = () =>
  new DeterministicTypeConverter(
    { decimal: "NUMERIC", binary: "BYTEA", character: "CHAR(1)" },
    {
      dialectName: "postgres",
      stringUnlimited: "TEXT",
      stringSized: (n) => `VARCHAR(${n})`,
      character: "CHAR",
      decimal: { kind: "optional", type: "NUMERIC", fallback: "NUMERIC" },
    },
  );

describe("DeterministicTypeConverter", () => {
  it("maps names and sized type definitions", () => {
    expect(sqlite().convert("uuid")).toBe("TEXT");
    expect(sqlite().convert({ type: "string" })).toBe("TEXT");
    expect(sqlite().convert({ type: "string", size: 64 })).toBe("VARCHAR(64)");
    expect(sqlserver().convert({ type: "character" })).toBe("NCHAR(1)");
    expect(sqlserver().convert({ type: "character", size: 4 })).toBe("NCHAR(4)");
    expect(postgres().convert({ type: "decimal" })).toBe("NUMERIC");
    expect(postgres().convert({ type: "decimal", size: [10, 2] })).toBe("NUMERIC(10, 2)");
    expect(sqlserver().convert({ type: "decimal", name: "amount", size: [10, 2] })).toBe(
      "DECIMAL(10, 2)",
    );
    expect(sqlserver().convert({ type: "binary", size: 16 })).toBe("VARBINARY(16)");
    expect(sqlserver().convert({ type: "binary" })).toBe("VARBINARY(MAX)");
  });

  it("rejects invalid decimals and unknown types", () => {
    expect(() => sqlserver().convert({ type: "decimal", name: "amount" })).toThrow(
      /requires size/,
    );
    expect(() => postgres().convert({ type: "decimal", name: "amount", size: 4 })).toThrow(
      /tuple/,
    );
    expect(() =>
      postgres().convert({ type: "decimal", name: "amount", size: [1.5, 2] }),
    ).toThrow(/integers/);
    expect(() => sqlite().convert({ type: "mystery" })).toThrow(/Unknown spec field type/);
    expect(() =>
      new DeterministicTypeConverter(
        { decimal: "DECIMAL" },
        {
          stringUnlimited: "TEXT",
          stringSized: (n) => `VARCHAR(${n})`,
          decimal: { kind: "required", type: "DECIMAL" },
        },
      ).convert({ type: "decimal", name: "amount" }),
    ).toThrow(/for sql/);
  });

  it("reverses unique natives", () => {
    expect(sqlite().convert("integer")).toBe("INTEGER");
    expect(sqlite().reverseConvert("INTEGER")).toBe("integer");
    expect(() => sqlite().reverseConvert("NOPE")).toThrow(/Unknown native/);
    const first = new DeterministicTypeConverter(
      { integer: "INT", unsignedinteger: "INT" },
      { stringUnlimited: "TEXT", stringSized: (n) => `VARCHAR(${n})` },
    );
    expect(first.reverseConvert("INT")).toBe("integer");
  });
});
