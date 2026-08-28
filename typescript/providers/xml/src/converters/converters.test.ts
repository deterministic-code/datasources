
import { afterEach, describe, expect, it } from "vitest";
import { TypeConverter } from "@datasources/core";
import { xmlTypes, registerConverters } from "./index.ts";

afterEach(() => TypeConverter.reset());

describe("xml converters", () => {
  it("registers every mapped sql type", () => {
    registerConverters();
    const mapping = TypeConverter.mapping({
      source: "xml",
      destination: "typescript",
    });
    expect(mapping.source).toBe("xml");
    for (const sqlType of Object.keys(xmlTypes.types)) {
      const row = { col: sample(sqlType) };
      const dest = mapping.to(row);
      expect(dest).toBeTruthy();
      expect(mapping.from(dest)).toBeTruthy();
    }
  });
});

function sample(sqlType: string): string | number {
  const key = sqlType.toUpperCase();
  if (key.includes("BOOL") || key === "BIT" || key === "TINYINT(1)") return 1;
  if (key.includes("DATE") || key.includes("TIME")) return "2020-01-02T03:04:05.000Z";
  if (key.includes("BLOB") || key.includes("BINARY") || key === "BYTEA" || key === "RAW" || key === "VARBINARY")
    return "aGk=";
  if (key.includes("UUID") || key === "UNIQUEIDENTIFIER" || key === "CHAR(36)")
    return "550e8400-e29b-41d4-a716-446655440000";
  if (key.includes("NUMERIC") || key.includes("DECIMAL") || key === "NUMBER") return "1.25";
  if (key.includes("INT") || key.includes("REAL") || key.includes("FLOAT") || key.includes("DOUBLE"))
    return 1;
  return "x";
}
