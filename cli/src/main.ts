import { requireProviders } from "./catalog.ts";
import { applyGenerate } from "./generate/applyGenerate.ts";
import { applyMigrations } from "./generate/applyMigrations.ts";
import { applyMigrateUp } from "./generate/migrateUp.ts";
import { sqlFlagsOf } from "./generate/sqlFlags.ts";
import { inquirerPrompt, nodeFs, spawnRunner } from "./io.ts";
import { help, parseArgs } from "./parse.ts";
import { applyAddRemove, applySetup } from "./setup.ts";
import { applyVersion } from "./version/applyVersion.ts";

const deps = {
  fs: nodeFs,
  run: spawnRunner,
  log: (line: string) => console.log(line),
  err: (line: string) => console.error(line),
  prompt: inquirerPrompt,
};

try {
  const parsed = parseArgs(process.argv.slice(2), ".");
  const command = parsed.command;
  const flags = parsed.flags;
  if (command.kind === "help") deps.log(help.trimEnd());
  else if (command.kind === "setup") await applySetup(flags, deps, command.providers);
  else if (command.kind === "add")
    await applyAddRemove(flags, deps, requireProviders(command.providers.length ? command.providers : (() => { throw new Error("Usage: datasources add --datasources <name...>"); })()), []);
  else if (command.kind === "remove")
    await applyAddRemove(flags, deps, [], requireProviders(command.providers.length ? command.providers : (() => { throw new Error("Usage: datasources remove --datasources <name...>"); })()));
  else if (command.kind === "version") await applyVersion(command, flags, deps);
  else if (command.kind === "migrations-create") await applyMigrations(command, flags, deps);
  else {
    await applyGenerate(command, flags, deps);
    if (command.target === "all" && command.migrations)
      await applyMigrations(
        {
          kind: "migrations-create",
          provider: command.provider,
          input: command.input,
          storedProcedures: command.storedProcedures,
          migrationsRoot: command.migrationsRoot,
          ...sqlFlagsOf(command),
        },
        flags,
        deps,
      );
    if (command.target === "all" && command.migrate) await applyMigrateUp(command, flags, deps);
  }
} catch (error) {
  console.error(error instanceof Error ? error.message : "Unknown error");
  process.exit(1);
}
