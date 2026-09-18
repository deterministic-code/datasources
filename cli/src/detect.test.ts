import { describe, expect, it } from "vitest";
import { csharpTarget, detect, isEcosystem, resolveEcosystem } from "./detect.ts";

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

  it("resolves a single ecosystem or an explicit override", () => {
    expect(resolveEcosystem(["ts"])).toBe("ts");
    expect(resolveEcosystem(["ts", "rust"], "rust")).toBe("rust");
    expect(resolveEcosystem([], "csharp")).toBe("csharp");
    expect(() => resolveEcosystem([])).toThrow(/No package.json/);
    expect(() => resolveEcosystem(["ts", "rust"])).toThrow(/Multiple project types/);
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
