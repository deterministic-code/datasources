import { describe, expect, it } from "vitest";
import { help, parseArgs } from "./parse.ts";

describe("parseArgs", () => {
  it("parses commands and flags", () => {
    expect(parseArgs(["setup", "--yes", "--cwd", "app"], ".")).toEqual({
      command: { kind: "setup", providers: [] },
      flags: { cwd: "app", dryRun: false, yes: true },
    });
    expect(parseArgs(["setup", "--yes", "--ecosystem", "csharp", "sqlite", "postgres"]).command).toEqual({
      kind: "setup",
      providers: ["sqlite", "postgres"],
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
      migrations: true,
      migrate: true,
      storedProcedures: false,
      migrationsRoot: "sql",
    });
    expect(
      parseArgs([
        "generate",
        "all",
        "--dialect",
        "typescript",
        "--no-migrations",
        "--no-migrate",
        "--migrations-root",
        "db",
        "--migrations-path",
        "changes",
      ]).command,
    ).toMatchObject({
      migrations: false,
      migrate: false,
      migrationsRoot: "db",
      migrationsPath: "changes",
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
    expect(parseArgs(["generate", "all", "--dialect", "typescript", "--stored-procedures"]).command).toMatchObject({
      storedProcedures: true,
    });
    expect(() => parseArgs(["generate"])).toThrow(/Usage: datasources generate/);
    expect(() => parseArgs(["generate", "nope", "--dialect", "typescript"])).toThrow(/Usage: datasources generate/);
    expect(() => parseArgs(["generate", "all", "extra", "--dialect", "typescript"])).toThrow(/Usage: datasources generate/);
    expect(() => parseArgs(["generate", "all"])).toThrow(/--dialect is required/);
    expect(() => parseArgs(["generate", "all", "--dialect"])).toThrow(/Missing value for --dialect/);
    expect(help).toMatch(/generate <all\|types\|repositories\|datasource>/);
    expect(help).toMatch(/--stored-procedures/);
  });

  it("parses migrations create", () => {
    expect(parseArgs(["migrations", "create"]).command).toEqual({
      kind: "migrations-create",
      provider: "deterministic",
      input: "deterministic",
      storedProcedures: false,
      migrationsRoot: "sql",
    });
    expect(
      parseArgs([
        "migrations",
        "create",
        "--provider",
        "deterministic",
        "--input",
        "in",
        "--migrations-root",
        "db",
        "--migrations-path",
        "./sql/sqlite/migrations",
      ]).command,
    ).toEqual({
      kind: "migrations-create",
      provider: "deterministic",
      input: "in",
      storedProcedures: false,
      migrationsRoot: "db",
      migrationsPath: "./sql/sqlite/migrations",
    });
    expect(parseArgs(["migrations", "create", "--stored-procedures"]).command).toMatchObject({
      storedProcedures: true,
    });
    expect(parseArgs(["migrations", "create", "--migrations-root=out", "--migrations-path=here"]).command).toMatchObject({
      migrationsRoot: "out",
      migrationsPath: "here",
    });
    expect(() => parseArgs(["migrations"])).toThrow(/Usage: datasources migrations create/);
    expect(() => parseArgs(["migrations", "up"])).toThrow(/Usage: datasources migrations create/);
    expect(() => parseArgs(["migrations", "create", "extra"])).toThrow(/Usage: datasources migrations create/);
    expect(() => parseArgs(["migrations", "create", "--migrations-path"])).toThrow(/Missing value for --migrations-path/);
    expect(help).toMatch(/migrations create/);
  });

  it("parses version", () => {
    expect(parseArgs(["version"]).command).toEqual({ kind: "version", input: "deterministic" });
    expect(parseArgs(["version", "--input", "in"]).command).toEqual({ kind: "version", input: "in" });
    expect(parseArgs(["version", "--input=spec"]).command).toEqual({ kind: "version", input: "spec" });
    expect(() => parseArgs(["version", "extra"])).toThrow(/Usage: datasources version/);
    expect(help).toMatch(/version[\s\S]*version-\*\/diff\.yaml/);
  });
});
