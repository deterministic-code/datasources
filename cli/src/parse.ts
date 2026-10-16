import { isEcosystem } from "./detect.ts";
import type { Ecosystem } from "./catalog.ts";
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
};

export type MigrationsCreateCommand = {
  kind: "migrations-create";
  provider: string;
  input: string;
  migrationsRoot: string;
  migrationsPath?: string;
};

export type VersionCommand = {
  kind: "version";
  input: string;
};

export type Command =
  | { kind: "setup" }
  | { kind: "add"; providers: string[] }
  | { kind: "remove"; providers: string[] }
  | { kind: "help" }
  | GenerateCommand
  | MigrationsCreateCommand
  | VersionCommand;

export const help = `Usage: datasources <command> [options]

setup                 Interactive .env, provider install, and optional deterministic/
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
  --dialect csharp|typescript|rust
  --provider <name>           Spec provider (default: deterministic)
  --input <dir>               Deterministic project (default: ./deterministic)
  --output <dir>              Generated files (default: ./generated)

migrations create     SQL migrations from the spec (migraters layout)
  --provider <name>           Spec provider (default: deterministic)
  --input <dir>               Deterministic project (default: ./deterministic)
  --migrations-root <dir>     Root prefix (default: sql)
  --migrations-path <dir>     One migrations directory (single SQL dialect)

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
    migrationsRoot: "sql",
    migrationsPath: "",
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
    else if (arg === "--migrations-root") generate.migrationsRoot = take(argv, ++i, "--migrations-root");
    else if (arg.startsWith("--migrations-root=")) generate.migrationsRoot = arg.slice("--migrations-root=".length);
    else if (arg === "--migrations-path") generate.migrationsPath = take(argv, ++i, "--migrations-path");
    else if (arg.startsWith("--migrations-path=")) generate.migrationsPath = arg.slice("--migrations-path=".length);
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
    migrationsRoot: string;
    migrationsPath: string;
  },
): Command => {
  if (verb === undefined || verb === "help") return { kind: "help" };
  if (verb === "setup") return { kind: "setup" };
  if (verb === "add") return { kind: "add", providers: names };
  if (verb === "remove") return { kind: "remove", providers: names };
  if (verb === "generate") return generateCommand(names, generate);
  if (verb === "migrations") return migrationsCommand(names, generate);
  if (verb === "version") return versionCommand(names, generate.input);
  throw new Error(`Unknown command: ${verb}\n${help}`);
};

const migrationsCommand = (
  names: string[],
  generate: { provider: string; input: string; migrationsRoot: string; migrationsPath: string },
): MigrationsCreateCommand => {
  if (names.length !== 1 || names[0] !== "create")
    throw new Error(`Usage: datasources migrations create\n${help}`);
  return {
    kind: "migrations-create",
    provider: generate.provider,
    input: generate.input,
    migrationsRoot: generate.migrationsRoot,
    ...(generate.migrationsPath === "" ? {} : { migrationsPath: generate.migrationsPath }),
  };
};

const versionCommand = (names: string[], input: string): VersionCommand => {
  if (names.length) throw new Error(`Usage: datasources version\n${help}`);
  return { kind: "version", input };
};

const generateCommand = (
  names: string[],
  generate: { dialect: string; provider: string; input: string; output: string },
): GenerateCommand => {
  if (names.length !== 1) throw new Error(`Usage: datasources generate <all|types|repositories|datasource>\n${help}`);
  const target = names[0] as string;
  if (!isTarget(target)) throw new Error(`Usage: datasources generate <all|types|repositories|datasource>\n${help}`);
  if (!isDialect(generate.dialect)) throw new Error("--dialect is required (csharp|typescript|rust)");
  return { kind: "generate", target, dialect: generate.dialect, provider: generate.provider, input: generate.input, output: generate.output };
};

const ecosystem = (value: string | undefined): Ecosystem =>
  value && isEcosystem(value)
    ? value
    : (() => {
        throw new Error(`Unknown --ecosystem ${value}. Use ts|rust|csharp`);
      })();
