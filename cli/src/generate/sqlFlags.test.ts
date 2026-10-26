import { createCasing } from "@deterministic-code/datasources-code-generators/sql/common/default-casing";
import { describe, expect, it } from "vitest";
import { overlaySqlFlags, parseLetterCaseFlag } from "./sqlFlags.ts";

describe("sqlFlags", () => {
  it("parses letter-case flags", () => {
    expect(parseLetterCaseFlag("upper", "--object-names")).toBe("Upper");
    expect(parseLetterCaseFlag("LOWER", "--sql-keywords")).toBe("Lower");
    expect(parseLetterCaseFlag("Auto", "--sql-keywords")).toBe("Auto");
    expect(() => parseLetterCaseFlag("screaming", "--object-names")).toThrow(/--object-names must be one of/);
  });

  it("overlays SQL flags onto datasource keys only", () => {
    expect(
      overlaySqlFlags({ existing: "1", "languages.typescript.casing.types": "Pascal" }, {
        pluralizeTableNames: false,
        tableNames: "Pascal",
        columnNames: "Camel",
        objectNames: "Upper",
        sqlKeywords: "Lower",
      }),
    ).toEqual({
      existing: "1",
      "languages.typescript.casing.types": "Pascal",
      "datasource.pluralize_datatable_names": "false",
      "datasource.casing.types": "Pascal",
      "datasource.casing.fields": "Camel",
      "datasource.casing.objects": "Upper",
      "datasource.casing.keywords": "Lower",
    });
    expect(overlaySqlFlags({ existing: "1" }, {})).toEqual({ existing: "1" });
    expect(overlaySqlFlags({ "datasource.pluralize_datatable_names": "false" }, { pluralizeTableNames: true })).toEqual({
      "datasource.pluralize_datatable_names": "true",
    });
  });

  it("writes Auto keywords and pack still uppercases", () => {
    const settings = overlaySqlFlags({}, { sqlKeywords: "Auto" });
    expect(settings["datasource.casing.keywords"]).toBe("Auto");
    expect(createCasing(settings).keyword("create")).toBe("CREATE");
  });
});
