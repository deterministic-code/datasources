import { describe, expect, it } from "vitest";
import {
  emitsNativeRowVersion,
  isOccBinaryColumn,
  usesNativeRowVersion,
} from "./SpecField.ts";
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
      occBinary: "BLOB",
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
      occBinary: "BINARY(8)",
      rowVersion: "ROWVERSION",
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
  it("maps strings, characters, decimals, and binary variants", () => {
    expect(sqlite().convert({ type: "string" })).toBe("TEXT");
    expect(sqlite().convert({ type: "string", size: 64 })).toBe("VARCHAR(64)");
    expect(sqlserver().convert({ type: "character" })).toBe("NCHAR(1)");
    expect(sqlserver().convert({ type: "character", size: 4 })).toBe("NCHAR(4)");
    expect(postgres().convert({ type: "decimal" })).toBe("NUMERIC");
    expect(postgres().convert({ type: "decimal", size: [10, 2] })).toBe("NUMERIC(10, 2)");
    expect(sqlserver().convert({ type: "decimal", name: "amount", size: [10, 2] })).toBe(
      "DECIMAL(10, 2)",
    );
    expect(
      sqlserver().convert({
        type: "binary",
        isOptimisticConcurrency: true,
        useNativeRowVersion: true,
      }),
    ).toBe("ROWVERSION");
    expect(sqlserver().convert({ type: "binary", isOptimisticConcurrency: true })).toBe("BINARY(8)");
    expect(sqlserver().convert({ type: "binary", size: 16 })).toBe("VARBINARY(16)");
    expect(sqlserver().convert({ type: "binary" })).toBe("VARBINARY(MAX)");
    expect(sqlite().convert({ type: "binary", isOptimisticConcurrency: true })).toBe("BLOB");
    expect(sqlite().convert({ type: "uuid" })).toBe("TEXT");
    const occ = { type: "binary", isOptimisticConcurrency: true, useNativeRowVersion: true };
    expect(usesNativeRowVersion(occ)).toBe(true);
    expect(emitsNativeRowVersion(occ, "sqlserver")).toBe(true);
    expect(emitsNativeRowVersion(occ, "sqlite")).toBe(false);
    expect(isOccBinaryColumn({ type: "binary", isOptimisticConcurrency: true })).toBe(true);
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
    expect(sqlite().reverseConvert("INTEGER")).toBe("integer");
    expect(() => sqlite().reverseConvert("NOPE")).toThrow(/Unknown native/);
    const multi = new DeterministicTypeConverter(
      { integer: "INT", number: "INT" },
      { stringUnlimited: "TEXT", stringSized: (n) => `VARCHAR(${n})` },
    );
    expect(() => multi.reverseConvert("INT")).toThrow(/multiple spec types/);
  });
});
