import { createCasingStrategy } from "@deterministic-code/generators-common/casing-strategy";
import { describe, expect, it } from "vitest";
import { caseDirectories, overlayCasingFlags, parseCaseFlag } from "./casing.ts";
import { languageOf } from "./RepositoryGenerator.ts";

describe("languageOf", () => {
  it("rejects an unknown dialect", async () => {
    await expect(languageOf("go")).rejects.toThrow(/Unknown dialect: go/);
  });
});

describe("casing", () => {
  it("parses and overlays generate flags", () => {
    expect(parseCaseFlag("kebab", "--file-names")).toBe("Kebab");
    expect(() => parseCaseFlag("screaming", "--file-names")).toThrow(/--file-names must be one of/);
    expect(
      overlayCasingFlags({ existing: "1" }, "typescript", {
        fileNames: "Pascal",
        types: "Snake",
        fields: "Camel",
        directories: "Kebab",
      }),
    ).toEqual({
      existing: "1",
      "languages.typescript.casing.file_names": "Pascal",
      "languages.typescript.casing.types": "Snake",
      "languages.typescript.casing.fields": "Camel",
      "languages.typescript.casing.directories": "Kebab",
    });
    expect(overlayCasingFlags({ existing: "1" }, "csharp", {})).toEqual({ existing: "1" });
  });

  it("cases Paths directories and keeps sql folders literal", () => {
    const ts = createCasingStrategy("typescript", { "languages.typescript.casing.directories": "Pascal" });
    expect(caseDirectories("types/view", ts)).toBe("Types/View");
    expect(caseDirectories("repositories", ts)).toBe("Repositories");
    expect(caseDirectories("sql/custom/__tests__/frontend", ts)).toBe("sql/custom/__tests__/frontend");
    expect(caseDirectories("features/user/custom/file.ts", ts)).toBe("Features/User/custom/file.ts");
  });
});
