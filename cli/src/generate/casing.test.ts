import { createCasingStrategy } from "@deterministic-code/generators-common/casing-strategy";
import { describe, expect, it } from "vitest";
import { caseDirectories, overlayCasingFlags, parseCaseFlag, repositoriesPrefix, typesPrefix } from "./casing.ts";

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

  it("cases layout prefixes and keeps dialect folders literal", () => {
    const ts = createCasingStrategy("typescript", { "languages.typescript.casing.directories": "Pascal" });
    expect(typesPrefix("typescript", ts, "datasource")).toBe("typescript/Types/Generated/Datasource");
    expect(typesPrefix("csharp", ts, "views")).toBe("csharp");
    expect(repositoriesPrefix("rust", ts)).toBe("rust/Repositories");
    expect(caseDirectories("typescript/types/generated/datasource/user.ts", ts)).toBe(
      "typescript/Types/Generated/Datasource/user.ts",
    );
    expect(caseDirectories("sql/custom/__tests__/frontend", ts)).toBe("sql/custom/__tests__/frontend");
    expect(caseDirectories("features/user/custom/file.ts", ts)).toBe("Features/User/custom/file.ts");
  });
});
