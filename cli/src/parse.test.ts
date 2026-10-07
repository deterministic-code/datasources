import { describe, expect, it } from "vitest";
import { help, parseArgs } from "./parse.ts";

describe("parseArgs", () => {
  it("parses commands and flags", () => {
    expect(parseArgs(["setup", "--yes", "--cwd", "app"], ".")).toEqual({
      command: { kind: "setup" },
      flags: { cwd: "app", dryRun: false, yes: true },
    });
    expect(parseArgs(["add", "sqlite", "--cwd=app", "--dry-run"])).toEqual({
      command: { kind: "add", providers: ["sqlite"] },
      flags: { cwd: "app", dryRun: true, yes: false },
    });
    expect(parseArgs(["remove", "mysql", "--ecosystem", "rust", "-y"])).toMatchObject({
      command: { kind: "remove", providers: ["mysql"] },
      flags: { ecosystem: "rust", yes: true },
    });
    expect(parseArgs(["--ecosystem=ts", "add", "json"]).flags.ecosystem).toBe("ts");
    expect(parseArgs(["--help"]).command).toEqual({ kind: "help" });
    expect(parseArgs(["-h"]).command).toEqual({ kind: "help" });
    expect(parseArgs(["help"]).command).toEqual({ kind: "help" });
    expect(parseArgs([]).command).toEqual({ kind: "help" });
    expect(help).toMatch(/datasources <command>/);
    expect(help).toMatch(/setup[\s\S]*--yes/);
    expect(help).not.toMatch(/^Options:/m);
  });

  it("keeps cwd when a flag value is missing", () => {
    expect(parseArgs(["--cwd"], "here").flags.cwd).toBe("here");
    expect(() => parseArgs(["--ecosystem"])).toThrow(/Unknown --ecosystem/);
    expect(() => parseArgs(["--ecosystem=go"])).toThrow(/Unknown --ecosystem/);
    expect(() => parseArgs(["wat"])).toThrow(/Unknown command: wat/);
  });

  it("parses generate", () => {
    expect(parseArgs(["generate", "all", "--dialect", "rust", "--provider", "deterministic", "--input", "in", "--output", "out"]).command).toEqual({
      kind: "generate",
      target: "all",
      dialect: "rust",
      provider: "deterministic",
      input: "in",
      output: "out",
    });
    expect(parseArgs(["generate", "types", "--dialect=csharp", "--provider=deterministic", "--input=in", "--output=out"]).command).toMatchObject({
      target: "types",
      dialect: "csharp",
      provider: "deterministic",
      input: "in",
      output: "out",
    });
    expect(parseArgs(["generate", "repositories", "--dialect", "typescript"]).command).toMatchObject({
      target: "repositories",
      dialect: "typescript",
      provider: "deterministic",
      input: "deterministic",
      output: "generated",
    });
    expect(parseArgs(["generate", "datasource", "--dialect", "typescript"]).command).toMatchObject({ target: "datasource" });
    expect(() => parseArgs(["generate"])).toThrow(/Usage: datasources generate/);
    expect(() => parseArgs(["generate", "nope", "--dialect", "typescript"])).toThrow(/Usage: datasources generate/);
    expect(() => parseArgs(["generate", "all", "extra", "--dialect", "typescript"])).toThrow(/Usage: datasources generate/);
    expect(() => parseArgs(["generate", "all"])).toThrow(/--dialect is required/);
    expect(() => parseArgs(["generate", "all", "--dialect"])).toThrow(/Missing value for --dialect/);
    expect(help).toMatch(/generate <all\|types\|repositories\|datasource>/);
  });
});
