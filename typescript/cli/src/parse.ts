import { isEcosystem } from "./detect.ts";
import type { Ecosystem } from "./catalog.ts";

export type Flags = {
  cwd: string;
  ecosystem?: Ecosystem;
  dryRun: boolean;
  yes: boolean;
  engine: boolean;
};

export type Command =
  | { kind: "setup" }
  | { kind: "add"; providers: string[] }
  | { kind: "remove"; providers: string[] }
  | { kind: "help" };

export const help = `Usage: datasources <command> [options]

Commands:
  setup                 Interactive .env and provider install
  add <provider...>     Add provider packages
  remove <provider...>  Remove provider packages

Options:
  --cwd <dir>                 Consumer project directory (default: .)
  --ecosystem ts|rust|csharp  Required when more than one project type exists
  --dry-run                   Print package-manager commands without running them
  --yes                       Setup without prompts (defaults / existing .env)
  --no-engine                 Skip Datasources.Sqlite.Microsoft
`;

export const parseArgs = (argv: string[], cwd = "."): { command: Command; flags: Flags } => {
  const flags: Flags = { cwd, dryRun: false, yes: false, engine: true };
  const rest: string[] = [];
  for (let i = 0; i < argv.length; i++) {
    const arg = argv[i]!;
    if (arg === "--cwd") flags.cwd = argv[++i] ?? flags.cwd;
    else if (arg.startsWith("--cwd=")) flags.cwd = arg.slice("--cwd=".length);
    else if (arg === "--ecosystem") flags.ecosystem = ecosystem(argv[++i]);
    else if (arg.startsWith("--ecosystem=")) flags.ecosystem = ecosystem(arg.slice("--ecosystem=".length));
    else if (arg === "--dry-run") flags.dryRun = true;
    else if (arg === "--yes" || arg === "-y") flags.yes = true;
    else if (arg === "--no-engine") flags.engine = false;
    else if (arg === "--help" || arg === "-h") return { command: { kind: "help" }, flags };
    else rest.push(arg);
  }
  const [verb, ...names] = rest;
  return {
    command: ({
      setup: () => ({ kind: "setup" as const }),
      add: () => ({ kind: "add" as const, providers: names }),
      remove: () => ({ kind: "remove" as const, providers: names }),
      help: () => ({ kind: "help" as const }),
      undefined: () => ({ kind: "help" as const }),
    }[verb ?? "undefined"] ??
      (() => {
        throw new Error(`Unknown command: ${verb}\n${help}`);
      }))(),
    flags,
  };
};

const ecosystem = (value: string | undefined): Ecosystem =>
  value && isEcosystem(value)
    ? value
    : (() => {
        throw new Error(`Unknown --ecosystem ${value}. Use ts|rust|csharp`);
      })();
