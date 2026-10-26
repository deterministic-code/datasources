import { describe, expect, it } from "vitest";
import { help, parseArgs } from "./parse.ts";

describe("parseArgs", () => {
  it("parses commands and flags", () => {
    expect(parseArgs(["setup", "--yes", "--cwd", "app"], ".")).toEqual({
      command: { kind: "setup", providers: [] },
      flags: { cwd: "app", languages: [], datasources: [], dryRun: false, yes: true },
    });
    expect(
      parseArgs(["setup", "--yes", "--languages", "csharp", "--datasources", "sqlite", "postgres"]).command,
    ).toEqual({
      kind: "setup",
      providers: ["sqlite", "postgres"],
    });
    expect(parseArgs(["setup", "--yes", "--languages", "csharp", "rust"]).flags.languages).toEqual(["csharp", "rust"]);
    expect(parseArgs(["setup", "--yes", "--languages=typescript,csharp"]).flags.languages).toEqual(["ts", "csharp"]);
    expect(parseArgs(["setup", "--yes", "--datasources=sqlite,postgres"]).command.providers).toEqual([
      "sqlite",
      "postgres",
    ]);
    expect(() => parseArgs(["setup", "sqlite"])).toThrow(/Usage: datasources setup/);
    expect(() => parseArgs(["setup", "--yes", "--languages", "csharp", "sqlite"])).toThrow(/Unknown --languages sqlite/);
    expect(parseArgs(["add", "sqlite", "--cwd=app", "--dry-run"])).toEqual({
      command: { kind: "add", providers: ["sqlite"] },
      flags: { cwd: "app", languages: [], datasources: [], dryRun: true, yes: false },
    });
    expect(parseArgs(["add", "--datasources", "sqlite", "postgres", "sqlite"]).command.providers).toEqual([
      "sqlite",
      "postgres",
    ]);
    expect(parseArgs(["remove", "mysql", "--languages", "rust", "-y"])).toMatchObject({
      command: { kind: "remove", providers: ["mysql"] },
      flags: { languages: ["rust"], yes: true },
    });
    expect(parseArgs(["--languages=ts", "add", "json"]).flags.languages).toEqual(["ts"]);
    expect(parseArgs(["--help"]).command).toEqual({ kind: "help" });
    expect(parseArgs(["-h"]).command).toEqual({ kind: "help" });
    expect(parseArgs(["help"]).command).toEqual({ kind: "help" });
    expect(parseArgs([]).command).toEqual({ kind: "help" });
    expect(help).toMatch(/datasources <command>/);
    expect(help).toMatch(/setup[\s\S]*--yes/);
    expect(help).toMatch(/--languages/);
    expect(help).toMatch(/--datasources/);
    expect(help).not.toMatch(/^Options:/m);
  });

  it("keeps cwd when a flag value is missing", () => {
    expect(parseArgs(["--cwd"], "here").flags.cwd).toBe("here");
    expect(() => parseArgs(["--languages"])).toThrow(/Missing value for --languages/);
    expect(() => parseArgs(["--languages="])).toThrow(/Missing value for --languages/);
    expect(() => parseArgs(["--datasources="])).toThrow(/Missing value for --datasources/);
    expect(() => parseArgs(["--languages=go"])).toThrow(/Unknown --languages/);
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
      fileNames: undefined,
      types: undefined,
      fields: undefined,
      directories: undefined,
      pluralizeTableNames: undefined,
      tableNames: undefined,
      columnNames: undefined,
      objectNames: undefined,
      sqlKeywords: undefined,
    });
    expect(
      parseArgs([
        "generate",
        "types",
        "--dialect",
        "typescript",
        "--file-names",
        "kebab",
        "--types=Pascal",
        "--fields",
        "camel",
        "--directories=Snake",
      ]).command,
    ).toMatchObject({
      fileNames: "Kebab",
      types: "Pascal",
      fields: "Camel",
      directories: "Snake",
    });
    expect(
      parseArgs([
        "generate",
        "types",
        "--dialect=typescript",
        "--file-names=Snake",
        "--types",
        "Camel",
        "--fields=Pascal",
      ]).command,
    ).toMatchObject({
      fileNames: "Snake",
      types: "Camel",
      fields: "Pascal",
    });
    expect(() => parseArgs(["generate", "types", "--dialect", "typescript", "--file-names", "screaming"])).toThrow(
      /--file-names must be one of/,
    );
    expect(
      parseArgs([
        "generate",
        "all",
        "--dialect",
        "typescript",
        "--no-migrations",
        "--no-auto-migrate-up",
        "--migrations-root",
        "db",
      ]).command,
    ).toMatchObject({
      migrations: false,
      migrate: false,
      migrationsRoot: "db",
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
    expect(() => parseArgs(["generate", "all", "--dialect", "typescript", "--migrations-path", "changes"])).toThrow(
      /Usage: datasources generate/,
    );
    expect(() => parseArgs(["generate", "all"])).toThrow(/--dialect is required/);
    expect(() => parseArgs(["generate", "all", "--dialect"])).toThrow(/Missing value for --dialect/);
    expect(help).toMatch(/generate <all\|types\|repositories\|datasource>/);
    expect(help).toMatch(/https:\/\/deterministic-code\.github\.io\/datasources\/cli\.html#generate/);
    expect(help).toMatch(/--stored-procedures/);
    expect(help).toMatch(/--file-names/);
    expect(help).toMatch(/--type-names\|--types/);
    expect(help).toMatch(/--field-names\|--fields/);
    expect(help).toMatch(/--folder-names\|--directories/);
    expect(help).toMatch(/--pluralize-table-names/);
    expect(help).toMatch(/--no-pluralize-table-names/);
    expect(help).toMatch(/--table-names/);
    expect(help).toMatch(/--column-names/);
    expect(help).toMatch(/--object-names/);
    expect(help).toMatch(/--sql-keywords/);
    expect(
      parseArgs([
        "generate",
        "all",
        "--dialect",
        "typescript",
        "--pluralize-table-names",
        "--table-names",
        "pascal",
        "--column-names=Camel",
        "--object-names",
        "upper",
        "--sql-keywords=lower",
      ]).command,
    ).toMatchObject({
      pluralizeTableNames: true,
      tableNames: "Pascal",
      columnNames: "Camel",
      objectNames: "Upper",
      sqlKeywords: "Lower",
    });
    expect(
      parseArgs([
        "generate",
        "all",
        "--dialect=typescript",
        "--no-pluralize-table-names",
        "--table-names=Snake",
        "--column-names",
        "kebab",
        "--object-names=Auto",
        "--sql-keywords",
        "auto",
      ]).command,
    ).toMatchObject({
      pluralizeTableNames: false,
      tableNames: "Snake",
      columnNames: "Kebab",
      objectNames: "Auto",
      sqlKeywords: "Auto",
    });
    expect(() => parseArgs(["generate", "all", "--dialect", "typescript", "--table-names", "screaming"])).toThrow(
      /--table-names must be one of/,
    );
    expect(() => parseArgs(["generate", "all", "--dialect", "typescript", "--object-names", "screaming"])).toThrow(
      /--object-names must be one of/,
    );
    expect(() => parseArgs(["generate", "all", "--dialect", "typescript", "--sql-keywords=screaming"])).toThrow(
      /--sql-keywords must be one of/,
    );
    expect(parseArgs(["generate", "types", "--dialect", "rust", "--type-names", "snake", "--field-names=kebab", "--folder-names", "pascal"]).command).toMatchObject({
      types: "Snake",
      fields: "Kebab",
      directories: "Pascal",
    });
    expect(parseArgs(["generate", "types", "--dialect", "rust", "--type-names=camel", "--folder-names=kebab"]).command).toMatchObject({
      types: "Camel",
      directories: "Kebab",
    });
  });

  it("parses migrations create", () => {
    expect(parseArgs(["migrations", "create"]).command).toEqual({
      kind: "migrations-create",
      provider: "deterministic",
      input: "deterministic",
      storedProcedures: false,
      migrationsRoot: "sql",
      pluralizeTableNames: undefined,
      tableNames: undefined,
      columnNames: undefined,
      objectNames: undefined,
      sqlKeywords: undefined,
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
      ]).command,
    ).toEqual({
      kind: "migrations-create",
      provider: "deterministic",
      input: "in",
      storedProcedures: false,
      migrationsRoot: "db",
      pluralizeTableNames: undefined,
      tableNames: undefined,
      columnNames: undefined,
      objectNames: undefined,
      sqlKeywords: undefined,
    });
    expect(parseArgs(["migrations", "create", "--stored-procedures"]).command).toMatchObject({
      storedProcedures: true,
    });
    expect(
      parseArgs([
        "migrations",
        "create",
        "--no-pluralize-table-names",
        "--table-names=Pascal",
        "--column-names",
        "camel",
        "--object-names=upper",
        "--sql-keywords",
        "Lower",
      ]).command,
    ).toMatchObject({
      pluralizeTableNames: false,
      tableNames: "Pascal",
      columnNames: "Camel",
      objectNames: "Upper",
      sqlKeywords: "Lower",
    });
    expect(parseArgs(["migrations", "create", "--migrations-root=out"]).command).toMatchObject({
      migrationsRoot: "out",
    });
    expect(() => parseArgs(["migrations"])).toThrow(/Usage: datasources migrations create/);
    expect(() => parseArgs(["migrations", "up"])).toThrow(/Usage: datasources migrations create/);
    expect(() => parseArgs(["migrations", "create", "extra"])).toThrow(/Usage: datasources migrations create/);
    expect(() => parseArgs(["migrations", "create", "--migrations-path", "here"])).toThrow(
      /Usage: datasources migrations create/,
    );
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
