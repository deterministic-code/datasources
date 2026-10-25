import { isEcosystem } from "./detect.ts";
import type { Ecosystem } from "./catalog.ts";
import { parseCaseFlag, type CaseFormat, type CasingFlags } from "./generate/casing.ts";
import { isDialect, type Dialect } from "./generate/GenerateRequest.ts";

export type Flags = {
  cwd: string;
  ecosystem?: Ecosystem;
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
} & CasingFlags;

export type MigrationsCreateCommand = {
  kind: "migrations-create";
  provider: string;
  input: string;
  storedProcedures: boolean;
  migrationsRoot: string;
};

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

setup [provider...]   Interactive .env, provider install, and optional deterministic/
  --cwd <dir>                 Consumer project directory (default: .)
  --ecosystem ts|rust|csharp  Required when more than one project type exists
  --dry-run                   Print package-manager commands without running them
  --yes                       Setup without prompts (defaults / existing .env)

add <provider...>     Add provider packages
  --cwd <dir>                 Consumer project directory (default: .)
  --ecosystem ts|rust|csharp  Required when more than one project type exists
  --dry-run                   Print package-manager commands without running them

remove <provider...>  Remove provider packages
  --cwd <dir>                 Consumer project directory (default: .)
  --ecosystem ts|rust|csharp  Required when more than one project type exists
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
  --file-names Pascal|Camel|Snake|Kebab|Auto
  --types Pascal|Camel|Snake|Kebab|Auto
  --fields Pascal|Camel|Snake|Kebab|Auto
  --directories Pascal|Camel|Snake|Kebab|Auto

migrations create     SQL migrations from the spec (migraters layout)
  --provider <name>           Spec provider (default: deterministic)
  --input <dir>               Deterministic project (default: ./deterministic)
  --migrations-root <dir>     Root prefix (default: sql)
  --stored-procedures         Emit stored-procedure migrations when the dialect can

version               Snapshot top-level YAML as the next version-*/diff.yaml
  --cwd <dir>                 Consumer project directory (default: .)
  --input <dir>               Deterministic project (default: ./deterministic)
  --dry-run                   Print the version path without writing
`;

const isTarget = (value: string): value is GenerateTarget => (generateTargets as readonly string[]).includes(value);

const take = (argv: string[], index: number, name: string): string => argv[index] ?? (() => { throw new Error(`Missing value for ${name}`); })();

export const parseArgs = (argv: string[], cwd = "."): { command: Command; flags: Flags } => {
  const flags: Flags = { cwd, dryRun: false, yes: false };
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
  };
  const rest: string[] = [];
  for (let i = 0; i < argv.length; i++) {
    const arg = argv[i]!;
    if (arg === "--cwd") flags.cwd = argv[++i] ?? flags.cwd;
    else if (arg.startsWith("--cwd=")) flags.cwd = arg.slice("--cwd=".length);
    else if (arg === "--ecosystem") flags.ecosystem = ecosystem(argv[++i]);
    else if (arg.startsWith("--ecosystem=")) flags.ecosystem = ecosystem(arg.slice("--ecosystem=".length));
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
    else if (arg === "--migrations-root") generate.migrationsRoot = take(argv, ++i, "--migrations-root");
    else if (arg.startsWith("--migrations-root=")) generate.migrationsRoot = arg.slice("--migrations-root=".length);
    else if (arg === "--file-names") generate.fileNames = parseCaseFlag(take(argv, ++i, "--file-names"), "--file-names");
    else if (arg.startsWith("--file-names=")) generate.fileNames = parseCaseFlag(arg.slice("--file-names=".length), "--file-names");
    else if (arg === "--types") generate.types = parseCaseFlag(take(argv, ++i, "--types"), "--types");
    else if (arg.startsWith("--types=")) generate.types = parseCaseFlag(arg.slice("--types=".length), "--types");
    else if (arg === "--fields") generate.fields = parseCaseFlag(take(argv, ++i, "--fields"), "--fields");
    else if (arg.startsWith("--fields=")) generate.fields = parseCaseFlag(arg.slice("--fields=".length), "--fields");
    else if (arg === "--directories") generate.directories = parseCaseFlag(take(argv, ++i, "--directories"), "--directories");
    else if (arg.startsWith("--directories=")) generate.directories = parseCaseFlag(arg.slice("--directories=".length), "--directories");
    else if (arg === "--help" || arg === "-h") return { command: { kind: "help" }, flags };
    else rest.push(arg);
  }
  const [verb, ...names] = rest;
  return { command: commandOf(verb, names, generate), flags };
};

const commandOf = (
  verb: string | undefined,
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
  } & CasingFlags,
): Command => {
  if (verb === undefined || verb === "help") return { kind: "help" };
  if (verb === "setup") return { kind: "setup", providers: names };
  if (verb === "add") return { kind: "add", providers: names };
  if (verb === "remove") return { kind: "remove", providers: names };
  if (verb === "generate") return generateCommand(names, generate);
  if (verb === "migrations") return migrationsCommand(names, generate);
  if (verb === "version") return versionCommand(names, generate.input);
  throw new Error(`Unknown command: ${verb}\n${help}`);
};

const migrationsCommand = (
  names: string[],
  generate: { provider: string; input: string; storedProcedures: boolean; migrationsRoot: string },
): MigrationsCreateCommand => {
  if (names.length !== 1 || names[0] !== "create")
    throw new Error(`Usage: datasources migrations create\n${help}`);
  return {
    kind: "migrations-create",
    provider: generate.provider,
    input: generate.input,
    storedProcedures: generate.storedProcedures,
    migrationsRoot: generate.migrationsRoot,
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
  } & CasingFlags,
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
  };
};

const ecosystem = (value: string | undefined): Ecosystem =>
  value && isEcosystem(value)
    ? value
    : (() => {
        throw new Error(`Unknown --ecosystem ${value}. Use ts|rust|csharp`);
      })();
