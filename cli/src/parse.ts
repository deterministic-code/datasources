import { isEcosystem } from "./detect.ts";
import type { Ecosystem } from "./catalog.ts";
import { parseCaseFlag, type CaseFormat, type CasingFlags } from "./generate/casing.ts";
import { parseLetterCaseFlag, type LetterCase, type SqlFlags } from "./generate/sqlFlags.ts";
import { isDialect, type Dialect } from "./generate/GenerateRequest.ts";

export type Flags = {
  cwd: string;
  languages: Ecosystem[];
  datasources: string[];
  dryRun: boolean;
  yes: boolean;
};

export const generateTargets = ["all", "types", "repositories", "datasource"] as const;

export type GenerateTarget = (typeof generateTargets)[number];

export type GenerateCommand = {
  kind: "generate";
  target: GenerateTarget;
  dialect: Dialect;
  provider: string;
  input: string;
  output: string;
  migrations: boolean;
  migrate: boolean;
  storedProcedures: boolean;
  migrationsRoot: string;
} & CasingFlags &
  SqlFlags;

export type MigrationsCreateCommand = {
  kind: "migrations-create";
  provider: string;
  input: string;
  storedProcedures: boolean;
  migrationsRoot: string;
} & SqlFlags;

export type VersionCommand = {
  kind: "version";
  input: string;
};

export type Command =
  | { kind: "setup"; providers: string[] }
  | { kind: "add"; providers: string[] }
  | { kind: "remove"; providers: string[] }
  | { kind: "help" }
  | GenerateCommand
  | MigrationsCreateCommand
  | VersionCommand;

export const help = `Usage: datasources <command> [options]

setup                 Interactive .env, datasource install, and optional deterministic/
  --cwd <dir>                 Consumer project directory (default: .)
  --languages ts|rust|csharp  Project languages (default: every type found)
  --datasources <name...>     Datasources to install (default: existing .env)
  --dry-run                   Print package-manager commands without running them
  --yes                       Setup without prompts (defaults / existing .env)

add [name...]         Add datasource packages
  --cwd <dir>                 Consumer project directory (default: .)
  --languages ts|rust|csharp  Project languages (default: every type found)
  --datasources <name...>     Datasources to add
  --dry-run                   Print package-manager commands without running them

remove [name...]      Remove datasource packages
  --cwd <dir>                 Consumer project directory (default: .)
  --languages ts|rust|csharp  Project languages (default: every type found)
  --datasources <name...>     Datasources to remove
  --dry-run                   Print package-manager commands without running them

generate <all|types|repositories|datasource>
                      Types, repository classes, and SQL
                      https://deterministic-code.github.io/datasources/cli.html#generate
  --dialect csharp|typescript|rust
  --provider <name>           Spec provider (default: deterministic)
  --input <dir>               Deterministic project (default: ./deterministic)
  --output <dir>              Generated files (default: ./generated)
  --no-migrations             Skip SQL migrations (generate all writes them)
  --no-auto-migrate-up        Skip migrate-up (generate all applies them)
  --migrations-root <dir>     Root prefix for generate all (default: sql)
  --stored-procedures         Emit stored-procedure migrations when the dialect can
  --pluralize-table-names     Plural table names in SQL and runtime queries (default)
  --no-pluralize-table-names  Singular table names (procedure names stay plural)
  --table-names Pascal|Camel|Snake|Kebab|Auto
  --column-names Pascal|Camel|Snake|Kebab|Auto
  --object-names Upper|Lower|Auto
  --sql-keywords Upper|Lower|Auto
  --file-names Pascal|Camel|Snake|Kebab|Auto
  --type-names|--types Pascal|Camel|Snake|Kebab|Auto
  --field-names|--fields Pascal|Camel|Snake|Kebab|Auto
  --folder-names|--directories Pascal|Camel|Snake|Kebab|Auto

migrations create     SQL migrations from the spec (migraters layout)
  --provider <name>           Spec provider (default: deterministic)
  --input <dir>               Deterministic project (default: ./deterministic)
  --migrations-root <dir>     Root prefix (default: sql)
  --stored-procedures         Emit stored-procedure migrations when the dialect can
  --pluralize-table-names     Plural table names in SQL (default)
  --no-pluralize-table-names  Singular table names (procedure names stay plural)
  --table-names Pascal|Camel|Snake|Kebab|Auto
  --column-names Pascal|Camel|Snake|Kebab|Auto
  --object-names Upper|Lower|Auto
  --sql-keywords Upper|Lower|Auto

version               Snapshot top-level YAML as the next version-*/diff.yaml
  --cwd <dir>                 Consumer project directory (default: .)
  --input <dir>               Deterministic project (default: ./deterministic)
  --dry-run                   Print the version path without writing
`;

const isTarget = (value: string): value is GenerateTarget => (generateTargets as readonly string[]).includes(value);

const take = (argv: string[], index: number, name: string): string => argv[index] ?? (() => { throw new Error(`Missing value for ${name}`); })();

export const parseArgs = (argv: string[], cwd = "."): { command: Command; flags: Flags } => {
  const flags: Flags = { cwd, languages: [], datasources: [], dryRun: false, yes: false };
  const generate = {
    dialect: "",
    provider: "deterministic",
    input: "deterministic",
    output: "generated",
    migrations: true,
    migrate: true,
    storedProcedures: false,
    migrationsRoot: "sql",
    fileNames: undefined as CaseFormat | undefined,
    types: undefined as CaseFormat | undefined,
    fields: undefined as CaseFormat | undefined,
    directories: undefined as CaseFormat | undefined,
    pluralizeTableNames: undefined as boolean | undefined,
    tableNames: undefined as CaseFormat | undefined,
    columnNames: undefined as CaseFormat | undefined,
    objectNames: undefined as LetterCase | undefined,
    sqlKeywords: undefined as LetterCase | undefined,
  };
  const rest: string[] = [];
  for (let i = 0; i < argv.length; i++) {
    const arg = argv[i]!;
    if (arg === "--cwd") flags.cwd = argv[++i] ?? flags.cwd;
    else if (arg.startsWith("--cwd=")) flags.cwd = arg.slice("--cwd=".length);
    else if (arg === "--languages") {
      i = takeList(argv, ++i, "--languages", (value) => flags.languages.push(language(value)));
    } else if (arg.startsWith("--languages="))
      splitList(arg.slice("--languages=".length), "--languages").forEach((value) => flags.languages.push(language(value)));
    else if (arg === "--datasources") {
      i = takeList(argv, ++i, "--datasources", (value) => flags.datasources.push(value));
    } else if (arg.startsWith("--datasources="))
      flags.datasources.push(...splitList(arg.slice("--datasources=".length), "--datasources"));
    else if (arg === "--dry-run") flags.dryRun = true;
    else if (arg === "--yes" || arg === "-y") flags.yes = true;
    else if (arg === "--dialect") generate.dialect = take(argv, ++i, "--dialect");
    else if (arg.startsWith("--dialect=")) generate.dialect = arg.slice("--dialect=".length);
    else if (arg === "--provider") generate.provider = take(argv, ++i, "--provider");
    else if (arg.startsWith("--provider=")) generate.provider = arg.slice("--provider=".length);
    else if (arg === "--input") generate.input = take(argv, ++i, "--input");
    else if (arg.startsWith("--input=")) generate.input = arg.slice("--input=".length);
    else if (arg === "--output") generate.output = take(argv, ++i, "--output");
    else if (arg.startsWith("--output=")) generate.output = arg.slice("--output=".length);
    else if (arg === "--no-migrations") generate.migrations = false;
    else if (arg === "--no-auto-migrate-up") generate.migrate = false;
    else if (arg === "--stored-procedures") generate.storedProcedures = true;
    else if (arg === "--pluralize-table-names") generate.pluralizeTableNames = true;
    else if (arg === "--no-pluralize-table-names") generate.pluralizeTableNames = false;
    else if (arg === "--table-names") generate.tableNames = parseCaseFlag(take(argv, ++i, arg), arg);
    else if (arg.startsWith("--table-names=")) generate.tableNames = parseCaseFlag(arg.slice("--table-names=".length), "--table-names");
    else if (arg === "--column-names") generate.columnNames = parseCaseFlag(take(argv, ++i, arg), arg);
    else if (arg.startsWith("--column-names=")) generate.columnNames = parseCaseFlag(arg.slice("--column-names=".length), "--column-names");
    else if (arg === "--object-names") generate.objectNames = parseLetterCaseFlag(take(argv, ++i, arg), arg);
    else if (arg.startsWith("--object-names=")) generate.objectNames = parseLetterCaseFlag(arg.slice("--object-names=".length), "--object-names");
    else if (arg === "--sql-keywords") generate.sqlKeywords = parseLetterCaseFlag(take(argv, ++i, arg), arg);
    else if (arg.startsWith("--sql-keywords=")) generate.sqlKeywords = parseLetterCaseFlag(arg.slice("--sql-keywords=".length), "--sql-keywords");
    else if (arg === "--migrations-root") generate.migrationsRoot = take(argv, ++i, "--migrations-root");
    else if (arg.startsWith("--migrations-root=")) generate.migrationsRoot = arg.slice("--migrations-root=".length);
    else if (arg === "--file-names" || arg === "--type-names" || arg === "--types" || arg === "--field-names" || arg === "--fields" || arg === "--folder-names" || arg === "--directories") {
      const key = arg === "--file-names" ? "fileNames" : arg === "--type-names" || arg === "--types" ? "types" : arg === "--field-names" || arg === "--fields" ? "fields" : "directories";
      generate[key] = parseCaseFlag(take(argv, ++i, arg), arg);
    } else if (arg.startsWith("--file-names=")) generate.fileNames = parseCaseFlag(arg.slice("--file-names=".length), "--file-names");
    else if (arg.startsWith("--type-names=")) generate.types = parseCaseFlag(arg.slice("--type-names=".length), "--type-names");
    else if (arg.startsWith("--types=")) generate.types = parseCaseFlag(arg.slice("--types=".length), "--types");
    else if (arg.startsWith("--field-names=")) generate.fields = parseCaseFlag(arg.slice("--field-names=".length), "--field-names");
    else if (arg.startsWith("--fields=")) generate.fields = parseCaseFlag(arg.slice("--fields=".length), "--fields");
    else if (arg.startsWith("--folder-names=")) generate.directories = parseCaseFlag(arg.slice("--folder-names=".length), "--folder-names");
    else if (arg.startsWith("--directories=")) generate.directories = parseCaseFlag(arg.slice("--directories=".length), "--directories");
    else if (arg === "--help" || arg === "-h") return { command: { kind: "help" }, flags };
    else rest.push(arg);
  }
  const [verb, ...names] = rest;
  return { command: commandOf(verb, names, flags.datasources, generate), flags };
};

const commandOf = (
  verb: string | undefined,
  names: string[],
  datasources: string[],
  generate: {
    dialect: string;
    provider: string;
    input: string;
    output: string;
    migrations: boolean;
    migrate: boolean;
    storedProcedures: boolean;
    migrationsRoot: string;
  } & CasingFlags &
    SqlFlags,
): Command => {
  if (verb === undefined || verb === "help") return { kind: "help" };
  if (verb === "setup") {
    if (names.length) throw new Error(`Usage: datasources setup\n${help}`);
    return { kind: "setup", providers: unique(datasources) };
  }
  if (verb === "add") return { kind: "add", providers: unique([...datasources, ...names]) };
  if (verb === "remove") return { kind: "remove", providers: unique([...datasources, ...names]) };
  if (verb === "generate") return generateCommand(names, generate);
  if (verb === "migrations") return migrationsCommand(names, generate);
  if (verb === "version") return versionCommand(names, generate.input);
  throw new Error(`Unknown command: ${verb}\n${help}`);
};

const migrationsCommand = (
  names: string[],
  generate: { provider: string; input: string; storedProcedures: boolean; migrationsRoot: string } & SqlFlags,
): MigrationsCreateCommand => {
  if (names.length !== 1 || names[0] !== "create")
    throw new Error(`Usage: datasources migrations create\n${help}`);
  return {
    kind: "migrations-create",
    provider: generate.provider,
    input: generate.input,
    storedProcedures: generate.storedProcedures,
    migrationsRoot: generate.migrationsRoot,
    pluralizeTableNames: generate.pluralizeTableNames,
    tableNames: generate.tableNames,
    columnNames: generate.columnNames,
    objectNames: generate.objectNames,
    sqlKeywords: generate.sqlKeywords,
  };
};

const versionCommand = (names: string[], input: string): VersionCommand => {
  if (names.length) throw new Error(`Usage: datasources version\n${help}`);
  return { kind: "version", input };
};

const generateCommand = (
  names: string[],
  generate: {
    dialect: string;
    provider: string;
    input: string;
    output: string;
    migrations: boolean;
    migrate: boolean;
    storedProcedures: boolean;
    migrationsRoot: string;
  } & CasingFlags &
    SqlFlags,
): GenerateCommand => {
  if (names.length !== 1) throw new Error(`Usage: datasources generate <all|types|repositories|datasource>\n${help}`);
  const target = names[0] as string;
  if (!isTarget(target)) throw new Error(`Usage: datasources generate <all|types|repositories|datasource>\n${help}`);
  if (!isDialect(generate.dialect)) throw new Error("--dialect is required (csharp|typescript|rust)");
  return {
    kind: "generate",
    target,
    dialect: generate.dialect,
    provider: generate.provider,
    input: generate.input,
    output: generate.output,
    migrations: generate.migrations,
    migrate: generate.migrate,
    storedProcedures: generate.storedProcedures,
    migrationsRoot: generate.migrationsRoot,
    fileNames: generate.fileNames,
    types: generate.types,
    fields: generate.fields,
    directories: generate.directories,
    pluralizeTableNames: generate.pluralizeTableNames,
    tableNames: generate.tableNames,
    columnNames: generate.columnNames,
    objectNames: generate.objectNames,
    sqlKeywords: generate.sqlKeywords,
  };
};

const language = (value: string): Ecosystem =>
  value === "typescript"
    ? "ts"
    : isEcosystem(value)
      ? value
      : (() => {
          throw new Error(`Unknown --languages ${value}. Use ts|rust|csharp`);
        })();

const unique = (names: string[]): string[] => [...new Set(names)];

const splitList = (value: string, name: string): string[] => {
  const names = value.split(",").map((part) => part.trim()).filter(Boolean);
  return names.length ? names : (() => { throw new Error(`Missing value for ${name}`); })();
};

const takeList = (argv: string[], index: number, name: string, push: (value: string) => void): number => {
  if (index >= argv.length || argv[index]!.startsWith("-")) throw new Error(`Missing value for ${name}`);
  let i = index;
  while (i < argv.length && !argv[i]!.startsWith("-")) push(argv[i++]!);
  return i - 1;
};
