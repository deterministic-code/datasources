import { describe, expect, it } from "vitest";
import { csharpTarget, detect, isEcosystem, resolveLanguages } from "./detect.ts";

describe("detect", () => {
  it("finds npm, cargo, and csharp manifests", () => {
    expect(detect(["package.json"])).toEqual(["ts"]);
    expect(detect(["Cargo.toml"])).toEqual(["rust"]);
    expect(detect(["App.csproj"])).toEqual(["csharp"]);
    expect(detect(["App.sln"])).toEqual(["csharp"]);
    expect(detect(["App.slnx"])).toEqual(["csharp"]);
    expect(detect(["package.json", "Cargo.toml", "App.csproj"])).toEqual([
      "ts",
      "rust",
      "csharp",
    ]);
    expect(detect([])).toEqual([]);
  });

  it("resolves found languages or an explicit override", () => {
    expect(resolveLanguages(["ts"])).toEqual(["ts"]);
    expect(resolveLanguages(["ts", "rust"], ["rust"])).toEqual(["rust"]);
    expect(resolveLanguages([], ["csharp"])).toEqual(["csharp"]);
    expect(resolveLanguages(["ts", "rust"])).toEqual(["ts", "rust"]);
    expect(() => resolveLanguages([])).toThrow(/No package.json/);
  });

  it("picks a non-test csproj", () => {
    expect(csharpTarget(["App.Tests.csproj", "App.csproj"])).toBe("App.csproj");
    expect(csharpTarget(["Z.csproj", "A.csproj"])).toBe("A.csproj");
    expect(csharpTarget(["Only.Tests.csproj"])).toBe("Only.Tests.csproj");
    expect(csharpTarget(["App.sln"])).toBe("App.sln");
    expect(csharpTarget(["App.slnx"])).toBe("App.slnx");
    expect(csharpTarget([])).toBe(".");
    expect(isEcosystem("ts")).toBe(true);
    expect(isEcosystem("go")).toBe(false);
  });
});
