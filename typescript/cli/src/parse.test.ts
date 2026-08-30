import { describe, expect, it } from "vitest";
import { help, parseArgs } from "./parse.ts";

describe("parseArgs", () => {
  it("parses commands and flags", () => {
    expect(parseArgs(["setup", "--yes", "--cwd", "app"], ".")).toEqual({
      command: { kind: "setup" },
      flags: { cwd: "app", dryRun: false, yes: true, engine: true },
    });
    expect(parseArgs(["add", "sqlite", "--cwd=app", "--dry-run", "--no-engine"])).toEqual({
      command: { kind: "add", providers: ["sqlite"] },
      flags: { cwd: "app", dryRun: true, yes: false, engine: false },
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
  });

  it("keeps cwd when a flag value is missing", () => {
    expect(parseArgs(["--cwd"], "here").flags.cwd).toBe("here");
    expect(() => parseArgs(["--ecosystem"])).toThrow(/Unknown --ecosystem/);
    expect(() => parseArgs(["--ecosystem=go"])).toThrow(/Unknown --ecosystem/);
    expect(() => parseArgs(["wat"])).toThrow(/Unknown command: wat/);
  });
});
