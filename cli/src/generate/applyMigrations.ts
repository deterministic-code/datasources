import { isAbsolute, join } from "node:path";
import { generate as generateSql } from "@deterministic-code/datasources-code-generators/generate-sql";
import { memoryReader } from "@deterministic-code/generators-common/deterministic-reader";
import type { Flags, MigrationsCreateCommand } from "../parse.ts";
import type { Deps } from "../run.ts";
import { loadSpec } from "./loadSpec.ts";
import { providerByName } from "./providers.ts";
import { overlaySqlFlags } from "./sqlFlags.ts";
import { withStoredProcedures } from "./storedProcedures.ts";
import { writeEntries } from "./writeEntries.ts";

const at = (root: string, path: string): string => (isAbsolute(path) ? path : join(root, path));

export const applyMigrations = async (
  command: MigrationsCreateCommand,
  flags: Flags,
  deps: Deps,
): Promise<void> => {
  providerByName(command.provider);
  const loaded = await loadSpec(deps.fs, at(flags.cwd, command.input));
  const entries = await generateSql({
    reader: memoryReader(loaded.files),
    settings: overlaySqlFlags(withStoredProcedures(loaded.settings, command.storedProcedures), command),
  });
  const root = at(flags.cwd, command.migrationsRoot);
  if (flags.dryRun) {
    for (const entry of entries) deps.log(join(root, entry.filename));
    return;
  }
  for (const path of await writeEntries(deps.fs, root, entries)) deps.log(path);
};
